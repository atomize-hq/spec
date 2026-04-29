use crate::XtaskError;
use std::fs;
use std::path::{Component, Path, PathBuf};

pub const FAMILY_ROOT_DIR: &str = "semantic-families";
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
