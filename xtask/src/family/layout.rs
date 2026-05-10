use crate::XtaskError;
use crate::family::harness::FamilyHarness;
use crate::family::manifest::FamilyManifest;
use crate::family::paths::{FAMILY_ROOT_DIR, REQUIRED_BUCKETS};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct PacketLayout {
    pub bucket_cases: BTreeMap<String, Vec<PathBuf>>,
    pub case_filenames: BTreeSet<String>,
}

pub fn validate_packet_layout(
    packet_root: &Path,
    manifest: &FamilyManifest,
    harness: &FamilyHarness,
) -> Result<PacketLayout, XtaskError> {
    ensure_required_file(packet_root.join("candidate.md"), "candidate.md")?;
    ensure_required_file(packet_root.join("family.toml"), "family.toml")?;
    validate_candidate_mentions_locked_cases(packet_root, harness)?;

    let fixtures_root = packet_root.join("fixtures");
    ensure_required_dir(&fixtures_root, "fixtures")?;
    reject_symlinks_under(&fixtures_root)?;
    validate_bucket_directory_set(&fixtures_root)?;

    let mut bucket_cases = BTreeMap::new();
    let mut case_filenames = BTreeSet::new();

    for bucket in &manifest.corpus.required_buckets {
        let cases = validate_bucket(
            &fixtures_root.join(bucket),
            bucket,
            manifest.corpus.min_cases_per_bucket,
            &mut case_filenames,
        )?;
        bucket_cases.insert(bucket.clone(), cases);
    }

    ensure_locked_starter_cases_exist(packet_root, harness)?;

    Ok(PacketLayout {
        bucket_cases,
        case_filenames,
    })
}

fn validate_bucket_directory_set(fixtures_root: &Path) -> Result<(), XtaskError> {
    let mut actual = BTreeSet::new();
    for entry in fs::read_dir(fixtures_root).map_err(|error| {
        XtaskError::InvalidInput(format!(
            "failed to read fixtures directory `{}`: {error}",
            fixtures_root.display()
        ))
    })? {
        let entry = entry.map_err(|error| {
            XtaskError::InvalidInput(format!(
                "failed to inspect fixtures directory `{}`: {error}",
                fixtures_root.display()
            ))
        })?;
        let name = entry.file_name().to_string_lossy().into_owned();
        let file_type = entry.file_type().map_err(|error| {
            XtaskError::InvalidInput(format!(
                "failed to inspect fixtures entry `{}`: {error}",
                entry.path().display()
            ))
        })?;

        if !file_type.is_dir() {
            return Err(XtaskError::InvalidInput(format!(
                "fixtures directory `{}` must contain only bucket directories; found `{}`",
                fixtures_root.display(),
                name
            )));
        }
        actual.insert(name);
    }

    let expected = REQUIRED_BUCKETS
        .iter()
        .map(|bucket| bucket.to_string())
        .collect::<BTreeSet<_>>();
    if actual != expected {
        return Err(XtaskError::InvalidInput(format!(
            "fixtures directory `{}` must contain exactly {:?}",
            fixtures_root.display(),
            REQUIRED_BUCKETS
        )));
    }

    Ok(())
}

fn validate_bucket(
    bucket_root: &Path,
    bucket: &str,
    min_cases_per_bucket: u64,
    all_case_filenames: &mut BTreeSet<String>,
) -> Result<Vec<PathBuf>, XtaskError> {
    ensure_required_dir(bucket_root, bucket)?;
    ensure_bucket_root_entries(bucket_root)?;
    ensure_required_file(bucket_root.join("Cargo.toml"), "Cargo.toml")?;
    ensure_required_dir(&bucket_root.join("src"), "src")?;
    ensure_required_file(bucket_root.join("src/main.rs"), "src/main.rs")?;
    ensure_required_dir(&bucket_root.join("units"), "units")?;

    let expected_suffix = expected_suffix_for_bucket(bucket);
    let mut cases = Vec::new();
    for entry in WalkDir::new(bucket_root.join("units")).follow_links(false) {
        let entry = entry.map_err(|error| {
            XtaskError::InvalidInput(format!(
                "failed to walk units under `{}`: {error}",
                bucket_root.display()
            ))
        })?;

        if entry.file_type().is_dir() {
            continue;
        }

        let path = entry.into_path();
        let relative = path.strip_prefix(bucket_root.join("units")).map_err(|_| {
            XtaskError::InvalidInput(format!(
                "units entry `{}` escaped its bucket root",
                path.display()
            ))
        })?;
        let filename = path
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| {
                XtaskError::InvalidInput(format!(
                    "units entry `{}` has a non-UTF-8 filename",
                    path.display()
                ))
            })?;
        if is_ignored_derived_units_artifact(relative, filename) {
            continue;
        }
        let component_count = relative.components().count();
        if component_count != 2 {
            return Err(XtaskError::InvalidInput(format!(
                "units entry `{}` must match units/<namespace>/<case>.unit.spec",
                path.display()
            )));
        }

        if !filename.ends_with(".unit.spec") {
            return Err(XtaskError::InvalidInput(format!(
                "units entry `{}` must end with `.unit.spec`",
                path.display()
            )));
        }
        if filename.ends_with(expected_suffix) {
            if !all_case_filenames.insert(filename.to_string()) {
                return Err(XtaskError::InvalidInput(format!(
                    "duplicate fixture filename `{filename}` found across packet `{}`",
                    packet_name(bucket_root)
                )));
            }
            cases.push(path);
        }
    }

    if cases.len() < min_cases_per_bucket as usize {
        return Err(XtaskError::InvalidInput(format!(
            "bucket `{bucket}` must contain at least {min_cases_per_bucket} `.unit.spec` file(s)"
        )));
    }

    Ok(cases)
}

fn is_ignored_derived_units_artifact(relative: &Path, filename: &str) -> bool {
    filename == ".gitignore"
        || relative.components().count() == 2
            && (filename.ends_with(".spec.passport.json")
                || filename.ends_with(".test.evidence.json"))
}

fn ensure_bucket_root_entries(bucket_root: &Path) -> Result<(), XtaskError> {
    let mut actual = BTreeSet::new();
    for entry in fs::read_dir(bucket_root).map_err(|error| {
        XtaskError::InvalidInput(format!(
            "failed to read bucket directory `{}`: {error}",
            bucket_root.display()
        ))
    })? {
        let entry = entry.map_err(|error| {
            XtaskError::InvalidInput(format!(
                "failed to inspect bucket directory `{}`: {error}",
                bucket_root.display()
            ))
        })?;
        actual.insert(entry.file_name().to_string_lossy().into_owned());
    }

    let expected = ["Cargo.toml", "src", "units"]
        .into_iter()
        .map(ToString::to_string)
        .collect::<BTreeSet<_>>();
    if actual != expected {
        return Err(XtaskError::InvalidInput(format!(
            "bucket `{}` must contain exactly Cargo.toml, src/, and units/",
            bucket_root.display()
        )));
    }

    Ok(())
}

fn validate_candidate_mentions_locked_cases(
    packet_root: &Path,
    harness: &FamilyHarness,
) -> Result<(), XtaskError> {
    let candidate = fs::read_to_string(packet_root.join("candidate.md")).map_err(|error| {
        XtaskError::InvalidInput(format!(
            "failed to read candidate.md under `{}`: {error}",
            packet_root.display()
        ))
    })?;

    for case in harness.scaffold.starter_cases {
        let mentions = candidate.matches(case.path).count();
        if mentions != 1 {
            return Err(XtaskError::InvalidInput(format!(
                "candidate.md for `{}` must mention locked starter case `{}` exactly once, found {mentions}",
                packet_name(packet_root),
                case.path
            )));
        }
    }

    Ok(())
}

fn ensure_locked_starter_cases_exist(
    packet_root: &Path,
    harness: &FamilyHarness,
) -> Result<(), XtaskError> {
    for case in harness.scaffold.starter_cases {
        let path = packet_root.join(case.path);
        let metadata = fs::metadata(&path).map_err(|error| {
            XtaskError::InvalidInput(format!(
                "packet `{}` is missing locked starter case `{}`: {error}",
                packet_name(packet_root),
                case.path
            ))
        })?;
        if !metadata.is_file() {
            return Err(XtaskError::InvalidInput(format!(
                "packet `{}` locked starter case `{}` is not a regular file",
                packet_name(packet_root),
                case.path
            )));
        }
    }

    Ok(())
}

fn ensure_required_file(path: PathBuf, label: &str) -> Result<(), XtaskError> {
    let metadata = fs::metadata(&path).map_err(|error| {
        XtaskError::InvalidInput(format!(
            "required file `{label}` is missing at `{}`: {error}",
            path.display()
        ))
    })?;
    if !metadata.is_file() {
        return Err(XtaskError::InvalidInput(format!(
            "required file `{label}` at `{}` is not a regular file",
            path.display()
        )));
    }
    Ok(())
}

fn ensure_required_dir(path: &Path, label: &str) -> Result<(), XtaskError> {
    let metadata = fs::metadata(path).map_err(|error| {
        XtaskError::InvalidInput(format!(
            "required directory `{label}` is missing at `{}`: {error}",
            path.display()
        ))
    })?;
    if !metadata.is_dir() {
        return Err(XtaskError::InvalidInput(format!(
            "required directory `{label}` at `{}` is not a directory",
            path.display()
        )));
    }
    Ok(())
}

fn reject_symlinks_under(root: &Path) -> Result<(), XtaskError> {
    for entry in WalkDir::new(root).follow_links(false) {
        let entry = entry.map_err(|error| {
            XtaskError::InvalidInput(format!("failed to walk `{}`: {error}", root.display()))
        })?;
        if entry.file_type().is_symlink() {
            return Err(XtaskError::InvalidInput(format!(
                "symlink entries are not allowed under `{}`; found `{}`",
                root.display(),
                entry.path().display()
            )));
        }
    }
    Ok(())
}

fn expected_suffix_for_bucket(bucket: &str) -> &'static str {
    match bucket {
        "aligned" => "_aligned.unit.spec",
        "drift" => "_drift.unit.spec",
        "under_specified" => "_under_specified.unit.spec",
        "unsupported_near_miss" => "_unsupported_near_miss.unit.spec",
        _ => ".unit.spec",
    }
}

fn packet_name(bucket_root: &Path) -> String {
    bucket_root
        .ancestors()
        .find_map(|ancestor| {
            let parent = ancestor.parent()?;
            if parent.file_name()? == FAMILY_ROOT_DIR {
                ancestor.file_name()
            } else {
                None
            }
        })
        .and_then(|name| name.to_str())
        .unwrap_or("<unknown>")
        .to_string()
}
