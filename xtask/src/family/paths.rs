use crate::XtaskError;
use std::fs;
use std::path::{Component, Path, PathBuf};

pub const FAMILY_ROOT_DIR: &str = "semantic-families";
pub const FAMILY_PROMOTION_ARTIFACT_ROOT: &str = ".semantic-family-artifacts/family-promotion";
pub const FAMILY_PROMOTION_INVENTORY_DIR: &str =
    ".semantic-family-artifacts/family-promotion/inventory";
pub const FAMILY_COVERAGE_LATEST_PATH: &str =
    ".semantic-family-artifacts/family-promotion/analysis/coverage.latest.json";
pub const FAMILY_RECOMMENDATION_ANALYSIS_LATEST_PATH: &str =
    ".semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json";
pub const M27_CORPUS_MANIFEST_PATH: &str = "semantic-families/corpus/rust-function.toml";
pub const REQUIRED_BUCKETS: [&str; 4] = [
    "aligned",
    "drift",
    "under_specified",
    "unsupported_near_miss",
];

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FamilyId(String);

impl FamilyId {
    pub fn parse(raw: &str) -> Result<Self, XtaskError> {
        if raw.is_empty() {
            return Err(XtaskError::InvalidInput(
                "family id must not be empty".to_string(),
            ));
        }

        let parts: Vec<&str> = raw.split('.').collect();
        if parts.len() < 3 {
            return Err(XtaskError::InvalidInput(format!(
                "family id `{raw}` must match ^[a-z0-9]+(\\.[a-z0-9_]+)+\\.v[0-9]+$"
            )));
        }

        if !parts[0]
            .chars()
            .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit())
        {
            return Err(XtaskError::InvalidInput(format!(
                "family id `{raw}` must match ^[a-z0-9]+(\\.[a-z0-9_]+)+\\.v[0-9]+$"
            )));
        }

        for segment in &parts[1..parts.len() - 1] {
            if segment.is_empty()
                || !segment
                    .chars()
                    .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_')
            {
                return Err(XtaskError::InvalidInput(format!(
                    "family id `{raw}` must match ^[a-z0-9]+(\\.[a-z0-9_]+)+\\.v[0-9]+$"
                )));
            }
        }

        let version = parts.last().unwrap();
        if version.len() < 2
            || !version.starts_with('v')
            || !version[1..].chars().all(|ch| ch.is_ascii_digit())
        {
            return Err(XtaskError::InvalidInput(format!(
                "family id `{raw}` must match ^[a-z0-9]+(\\.[a-z0-9_]+)+\\.v[0-9]+$"
            )));
        }

        Ok(Self(raw.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn packet_dir_name(&self) -> &str {
        &self.0
    }

    pub fn crate_stem(&self) -> String {
        let without_version = self
            .0
            .rsplit_once('.')
            .map(|(prefix, _)| prefix)
            .unwrap_or(self.as_str());

        without_version.replace(['.', '_'], "-")
    }
}

#[derive(Debug, Clone)]
pub struct PacketPaths {
    pub root: PathBuf,
    pub candidate: PathBuf,
    pub manifest: PathBuf,
    pub fixtures: PathBuf,
    pub artifacts: PathBuf,
}

impl PacketPaths {
    pub fn new(workspace_root: &Path, family: FamilyId) -> Self {
        let root = workspace_root
            .join(FAMILY_ROOT_DIR)
            .join(family.packet_dir_name());

        Self {
            candidate: root.join("candidate.md"),
            manifest: root.join("family.toml"),
            fixtures: root.join("fixtures"),
            artifacts: workspace_root
                .join(".semantic-family-artifacts")
                .join(FAMILY_ROOT_DIR)
                .join(family.packet_dir_name()),
            root,
        }
    }
}

pub(crate) fn family_promotion_dir(family: &FamilyId) -> PathBuf {
    Path::new(FAMILY_PROMOTION_ARTIFACT_ROOT)
        .join(family.packet_dir_name())
        .to_path_buf()
}

pub(crate) fn family_recommendation_latest_path(family: &FamilyId) -> PathBuf {
    family_promotion_dir(family).join("recommendation.latest.json")
}

pub(crate) fn family_promotion_execution_path(family: &FamilyId, run_id: &str) -> PathBuf {
    family_promotion_dir(family)
        .join(run_id)
        .join("promotion.execution.json")
}

pub(crate) fn family_promotion_blocker_path(family: &FamilyId, run_id: &str) -> PathBuf {
    family_promotion_dir(family)
        .join(run_id)
        .join("blocker.report.json")
}

pub fn ensure_packet_path_safe(
    workspace_root: &Path,
    packet_root: &Path,
) -> Result<(), XtaskError> {
    ensure_existing_components_are_not_symlinks(
        workspace_root,
        &workspace_root.join(FAMILY_ROOT_DIR),
    )?;
    ensure_existing_components_are_not_symlinks(workspace_root, packet_root)
}

pub(crate) fn validate_repo_relative_path(
    raw_path: &str,
    field: &str,
) -> Result<PathBuf, XtaskError> {
    let path = Path::new(raw_path);
    if path.as_os_str().is_empty() || path.is_absolute() {
        return Err(XtaskError::InvalidInput(format!(
            "{field} `{raw_path}` must be a non-empty repo-relative path"
        )));
    }

    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::Normal(segment) => normalized.push(segment),
            _ => {
                return Err(XtaskError::InvalidInput(format!(
                    "{field} `{raw_path}` must contain only normal path components"
                )));
            }
        }
    }

    if normalized.as_os_str().is_empty() {
        return Err(XtaskError::InvalidInput(format!(
            "{field} `{raw_path}` must be a non-empty repo-relative path"
        )));
    }

    Ok(normalized)
}

pub(crate) fn validate_existing_repo_relative_path(
    workspace_root: &Path,
    raw_path: &str,
    field: &str,
) -> Result<PathBuf, XtaskError> {
    let relative = validate_repo_relative_path(raw_path, field)?;
    validate_existing_relative_path(workspace_root, &relative, field)
}

pub(crate) fn validate_existing_relative_path(
    workspace_root: &Path,
    relative: &Path,
    field: &str,
) -> Result<PathBuf, XtaskError> {
    ensure_existing_components_are_not_symlinks(workspace_root, &workspace_root.join(relative))?;
    let absolute = workspace_root.join(relative);
    if !absolute.exists() {
        return Err(XtaskError::InvalidInput(format!(
            "{field} `{}` does not exist in the workspace",
            relative.display()
        )));
    }

    let canonical_workspace = workspace_root.canonicalize().map_err(|error| {
        XtaskError::WriteFailure(format!(
            "failed to canonicalize workspace root `{}`: {error}",
            workspace_root.display()
        ))
    })?;
    let canonical_absolute = absolute.canonicalize().map_err(|error| {
        XtaskError::WriteFailure(format!(
            "failed to canonicalize `{}`: {error}",
            absolute.display()
        ))
    })?;
    if !canonical_absolute.starts_with(&canonical_workspace) {
        return Err(XtaskError::InvalidInput(format!(
            "{field} `{}` escapes the workspace root",
            relative.display()
        )));
    }

    Ok(absolute)
}

pub(crate) fn ensure_repo_path_parent(path: &Path) -> Result<(), XtaskError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            XtaskError::WriteFailure(format!("failed to create `{}`: {error}", parent.display()))
        })?;
    }
    Ok(())
}

pub(crate) fn write_bytes_atomically(path: &Path, bytes: &[u8]) -> Result<(), XtaskError> {
    ensure_repo_path_parent(path)?;
    let parent = path.parent().ok_or_else(|| {
        XtaskError::WriteFailure(format!(
            "failed to resolve parent directory for `{}`",
            path.display()
        ))
    })?;
    let file_name = path
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(|| {
            XtaskError::WriteFailure(format!(
                "failed to resolve file name for `{}`",
                path.display()
            ))
        })?;
    let tmp_path = parent.join(format!("{file_name}.tmp"));
    fs::write(&tmp_path, bytes).map_err(|error| {
        XtaskError::WriteFailure(format!("failed to write `{}`: {error}", tmp_path.display()))
    })?;
    fs::rename(&tmp_path, path).map_err(|error| {
        XtaskError::WriteFailure(format!(
            "failed to move `{}` into `{}`: {error}",
            tmp_path.display(),
            path.display()
        ))
    })
}

pub(crate) fn path_is_semantic_family_fixture(relative: &Path) -> bool {
    let components = relative
        .components()
        .filter_map(|component| match component {
            Component::Normal(value) => value.to_str(),
            _ => None,
        })
        .collect::<Vec<_>>();
    components.len() >= 3
        && components.first() == Some(&"semantic-families")
        && components.contains(&"fixtures")
}

fn ensure_existing_components_are_not_symlinks(
    workspace_root: &Path,
    path: &Path,
) -> Result<(), XtaskError> {
    let relative = path.strip_prefix(workspace_root).map_err(|_| {
        XtaskError::InvalidInput(format!(
            "unsafe packet path `{}` escapes the workspace root",
            path.display()
        ))
    })?;

    let mut current = PathBuf::from(workspace_root);
    for component in relative.components() {
        match component {
            Component::Normal(segment) => {
                current.push(segment);
            }
            _ => {
                return Err(XtaskError::InvalidInput(format!(
                    "unsafe packet path `{}` contains non-normal components",
                    path.display()
                )));
            }
        }

        match fs::symlink_metadata(&current) {
            Ok(metadata) if metadata.file_type().is_symlink() => {
                return Err(XtaskError::InvalidInput(format!(
                    "unsafe packet path `{}` contains a symlinked component at `{}`",
                    path.display(),
                    current.display()
                )));
            }
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => break,
            Err(error) => {
                return Err(XtaskError::WriteFailure(format!(
                    "failed to inspect `{}`: {error}",
                    current.display()
                )));
            }
        }
    }

    Ok(())
}
