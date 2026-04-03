use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

const CONFIG_FILE_NAME: &str = "spec.toml";

#[derive(Debug, Clone, Default, Deserialize, PartialEq)]
#[serde(default, deny_unknown_fields)]
pub struct WorkspaceConfig {
    pub validation: ValidationConfig,
}

#[derive(Debug, Clone, Default, Deserialize, PartialEq)]
#[serde(default, deny_unknown_fields)]
pub struct ValidationConfig {
    pub allow_unsafe_local_test_expect: bool,
}

pub fn load_workspace_config(target: &Path) -> Result<WorkspaceConfig> {
    let Some(config_path) = find_workspace_config(target) else {
        return Ok(WorkspaceConfig::default());
    };

    let contents = fs::read_to_string(&config_path)
        .with_context(|| format!("Failed to read {}", config_path.display()))?;
    toml::from_str(&contents).with_context(|| format!("Failed to parse {}", config_path.display()))
}

fn find_workspace_config(target: &Path) -> Option<PathBuf> {
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
}
