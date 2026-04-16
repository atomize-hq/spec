use anyhow::{Context, Result, bail};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

const CONFIG_FILE_NAME: &str = "spec.toml";

#[derive(Debug, Clone, Default, Deserialize, PartialEq)]
#[serde(default, deny_unknown_fields)]
pub struct WorkspaceConfig {
    pub validation: ValidationConfig,
    pub pipeline: PipelineConfig,
    pub libraries: BTreeMap<String, PathBuf>,
}

#[derive(Debug, Clone, Default, Deserialize, PartialEq)]
#[serde(default, deny_unknown_fields)]
pub struct ValidationConfig {
    pub allow_unsafe_local_test_expect: bool,
}

#[derive(Debug, Clone, Default, Deserialize, PartialEq)]
#[serde(default, deny_unknown_fields)]
pub struct PipelineConfig {
    pub crate_root: Option<PathBuf>,
    pub cargo_target_dir: Option<PathBuf>,
    pub timeout_secs: Option<u64>,
    /// Explicit module prefix override for non-standard layouts. When absent,
    /// the prefix is auto-derived from the output path relative to `{crate_root}/src/`.
    pub generated_module_prefix: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedLibrary {
    pub alias: String,
    pub root: PathBuf,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WorkspaceContext {
    pub config: WorkspaceConfig,
    pub config_path: Option<PathBuf>,
    pub workspace_root: Option<PathBuf>,
    pub repo_root: Option<PathBuf>,
    pub libraries: Vec<ResolvedLibrary>,
}

pub fn load_workspace_config(target: &Path) -> Result<WorkspaceConfig> {
    Ok(load_workspace_context(target)?.config)
}

pub fn load_workspace_context(target: &Path) -> Result<WorkspaceContext> {
    let Some(config_path) = find_workspace_config(target) else {
        return Ok(WorkspaceContext {
            config: WorkspaceConfig::default(),
            config_path: None,
            workspace_root: None,
            repo_root: None,
            libraries: Vec::new(),
        });
    };

    let config = read_workspace_config(&config_path)?;
    let workspace_root = canonicalize_existing_dir(
        config_path
            .parent()
            .filter(|path| !path.as_os_str().is_empty())
            .unwrap_or_else(|| Path::new(".")),
    )?;
    let repo_root = find_repo_root(&workspace_root).unwrap_or_else(|| workspace_root.clone());
    let libraries = resolve_direct_libraries(&workspace_root, &repo_root, &config)?;

    Ok(WorkspaceContext {
        config,
        config_path: Some(config_path),
        workspace_root: Some(workspace_root),
        repo_root: Some(repo_root),
        libraries,
    })
}

fn validate_workspace_config(config: WorkspaceConfig) -> Result<WorkspaceConfig> {
    if matches!(config.pipeline.timeout_secs, Some(0)) {
        bail!("[pipeline].timeout_secs must be greater than 0");
    }

    Ok(config)
}

fn read_workspace_config(config_path: &Path) -> Result<WorkspaceConfig> {
    let contents = fs::read_to_string(config_path)
        .with_context(|| format!("Failed to read {}", config_path.display()))?;
    let config: WorkspaceConfig = toml::from_str(&contents)
        .with_context(|| format!("Failed to parse {}", config_path.display()))?;
    validate_workspace_config(config)
        .with_context(|| format!("Failed to parse {}", config_path.display()))
}

pub fn find_workspace_config(target: &Path) -> Option<PathBuf> {
    let start = if target.is_file() {
        target.parent().unwrap_or(target)
    } else {
        target
    };

    for dir in start.ancestors() {
        let candidate = dir.join(CONFIG_FILE_NAME);
        if candidate.is_file() {
            return Some(candidate);
        }
    }

    None
}

fn resolve_direct_libraries(
    workspace_root: &Path,
    repo_root: &Path,
    config: &WorkspaceConfig,
) -> Result<Vec<ResolvedLibrary>> {
    let mut resolved = Vec::with_capacity(config.libraries.len());
    let mut seen_roots = BTreeMap::<PathBuf, String>::new();
    let workspace_root = canonicalize_existing_dir(workspace_root)?;
    let repo_root = canonicalize_existing_dir(repo_root)?;

    for (alias, relative_root) in &config.libraries {
        let candidate = workspace_root.join(relative_root);
        if !candidate.exists() {
            bail!(
                "SPEC_LIBRARY_PATH_NOT_FOUND: library '{alias}' path does not exist: {}",
                candidate.display()
            );
        }

        let canonical_root = canonicalize_existing_dir(&candidate)?;

        if !canonical_root.starts_with(&repo_root) {
            bail!(
                "SPEC_LIBRARY_OUT_OF_ROOT: library '{alias}' resolves outside repo root: {}",
                canonical_root.display()
            );
        }

        if canonical_root == workspace_root {
            bail!(
                "SPEC_LIBRARY_ALIAS_SELF: library '{alias}' resolves to the invoking library root"
            );
        }

        if let Some(existing_alias) = seen_roots.get(&canonical_root) {
            bail!(
                "SPEC_DUPLICATE_LIBRARY_ROOT: libraries '{existing_alias}' and '{alias}' resolve to {}",
                canonical_root.display()
            );
        }

        seen_roots.insert(canonical_root.clone(), alias.clone());
        resolved.push(ResolvedLibrary {
            alias: alias.clone(),
            root: canonical_root,
        });
    }

    Ok(resolved)
}

fn canonicalize_existing_dir(path: &Path) -> Result<PathBuf> {
    let canonical = path
        .canonicalize()
        .with_context(|| format!("Failed to resolve {}", path.display()))?;
    if !canonical.is_dir() {
        bail!("{} is not a directory", canonical.display());
    }
    Ok(canonical)
}

fn find_repo_root(start: &Path) -> Option<PathBuf> {
    for dir in start.ancestors() {
        let git_marker = dir.join(".git");
        if git_marker.is_dir() || git_marker.is_file() {
            return Some(dir.to_path_buf());
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn loads_default_config_when_missing() {
        let temp_dir = TempDir::new().unwrap();
        let config = load_workspace_config(temp_dir.path()).unwrap();
        assert_eq!(config, WorkspaceConfig::default());
    }

    #[test]
    fn discovers_nearest_config_from_nested_path() {
        let temp_dir = TempDir::new().unwrap();
        let root_config = temp_dir.path().join("spec.toml");
        fs::write(
            &root_config,
            "[validation]\nallow_unsafe_local_test_expect = true\n",
        )
        .unwrap();
        let nested_dir = temp_dir.path().join("units/pricing");
        fs::create_dir_all(&nested_dir).unwrap();

        let config = load_workspace_config(&nested_dir).unwrap();
        assert!(config.validation.allow_unsafe_local_test_expect);
    }

    #[test]
    fn rejects_unknown_fields() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(
            temp_dir.path().join("spec.toml"),
            "[validation]\nallow_unsafe_local_test_expect = true\nextra = true\n",
        )
        .unwrap();

        let err = load_workspace_config(temp_dir.path())
            .unwrap_err()
            .to_string();
        assert!(err.contains("Failed to parse"));
    }

    #[test]
    fn loads_pipeline_timeout_secs() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(
            temp_dir.path().join("spec.toml"),
            "[pipeline]\ntimeout_secs = 30\n",
        )
        .unwrap();

        let config = load_workspace_config(temp_dir.path()).unwrap();
        assert_eq!(config.pipeline.timeout_secs, Some(30));
    }

    #[test]
    fn rejects_zero_timeout_secs() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(
            temp_dir.path().join("spec.toml"),
            "[pipeline]\ntimeout_secs = 0\n",
        )
        .unwrap();

        let err = load_workspace_config(temp_dir.path()).unwrap_err();
        let err = format!("{err:#}");
        assert!(err.contains("timeout_secs must be greater than 0"));
    }

    #[test]
    fn loads_libraries_from_root_config() {
        let temp_dir = TempDir::new().unwrap();
        let repo_root = temp_dir.path().join("repo");
        let root = repo_root.join("app-spec");
        let shared = repo_root.join("shared-spec");
        fs::create_dir_all(&root).unwrap();
        fs::create_dir_all(&shared).unwrap();
        fs::write(repo_root.join(".git"), "gitdir: .git/modules/repo\n").unwrap();
        fs::write(
            root.join("spec.toml"),
            "[libraries]\nshared = \"../shared-spec\"\n",
        )
        .unwrap();

        let context = load_workspace_context(&root).unwrap();
        assert_eq!(context.libraries.len(), 1);
        assert_eq!(context.libraries[0].alias, "shared");
        assert_eq!(context.libraries[0].root, shared.canonicalize().unwrap());
    }

    #[test]
    fn rejects_missing_library_path() {
        let temp_dir = TempDir::new().unwrap();
        let repo_root = temp_dir.path().join("repo");
        let root = repo_root.join("app-spec");
        fs::create_dir_all(&root).unwrap();
        fs::write(repo_root.join(".git"), "gitdir: .git/modules/repo\n").unwrap();
        fs::write(
            root.join("spec.toml"),
            "[libraries]\nshared = \"../missing-spec\"\n",
        )
        .unwrap();

        let err = load_workspace_context(&root).unwrap_err().to_string();
        assert!(err.contains("SPEC_LIBRARY_PATH_NOT_FOUND"));
    }

    #[test]
    fn rejects_library_alias_to_self() {
        let temp_dir = TempDir::new().unwrap();
        let repo_root = temp_dir.path().join("repo");
        let root = repo_root.join("app-spec");
        fs::create_dir_all(&root).unwrap();
        fs::write(repo_root.join(".git"), "gitdir: .git/modules/repo\n").unwrap();
        fs::write(root.join("spec.toml"), "[libraries]\napp = \".\"\n").unwrap();

        let err = load_workspace_context(&root).unwrap_err().to_string();
        assert!(err.contains("SPEC_LIBRARY_ALIAS_SELF"));
    }

    #[test]
    fn rejects_duplicate_canonical_library_roots() {
        let temp_dir = TempDir::new().unwrap();
        let repo_root = temp_dir.path().join("repo");
        let root = repo_root.join("app-spec");
        let shared = repo_root.join("shared-spec");
        fs::create_dir_all(&root).unwrap();
        fs::create_dir_all(&shared).unwrap();
        fs::write(repo_root.join(".git"), "gitdir: .git/modules/repo\n").unwrap();
        fs::write(
            root.join("spec.toml"),
            "[libraries]\nshared = \"../shared-spec\"\nshared_copy = \"../shared-spec/./\"\n",
        )
        .unwrap();

        let err = load_workspace_context(&root).unwrap_err().to_string();
        assert!(err.contains("SPEC_DUPLICATE_LIBRARY_ROOT"));
    }

    #[test]
    fn rejects_library_paths_outside_repo_root() {
        let temp_dir = TempDir::new().unwrap();
        let repo_root = temp_dir.path().join("repo");
        let root = repo_root.join("specs/app-spec");
        let outside = temp_dir.path().join("outside-spec");
        fs::create_dir_all(&root).unwrap();
        fs::create_dir_all(&outside).unwrap();
        fs::write(repo_root.join(".git"), "gitdir: .git/modules/repo\n").unwrap();
        fs::write(
            root.join("spec.toml"),
            "[libraries]\noutside = \"../../../outside-spec\"\n",
        )
        .unwrap();

        let err = load_workspace_context(&root).unwrap_err().to_string();
        assert!(err.contains("SPEC_LIBRARY_OUT_OF_ROOT"));
    }
}
