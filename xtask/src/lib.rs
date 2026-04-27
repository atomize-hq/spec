mod family;

use clap::{Args, Parser, Subcommand};
use family::{certify, prove, scaffold};
use std::ffi::OsString;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum XtaskError {
    #[error("{0}")]
    InvalidInput(String),
    #[error("{0}")]
    AlreadyExists(String),
    #[error("{0}")]
    ProveSuiteFailure(String),
    #[error("{0}")]
    CertifyProveFailure(String),
    #[error("{0}")]
    CertifySuiteFailure(String),
    #[error("{0}")]
    CertifyArtifactWriteFailure(String),
    #[error("{0}")]
    WriteFailure(String),
    #[error("{0}")]
    NotImplemented(String),
    #[error("{0}")]
    Internal(String),
}

impl XtaskError {
    pub fn exit_code(&self) -> i32 {
        match self {
            Self::InvalidInput(_) => 2,
            Self::AlreadyExists(_) | Self::ProveSuiteFailure(_) | Self::CertifyProveFailure(_) => 3,
            Self::WriteFailure(_) | Self::CertifySuiteFailure(_) => 4,
            Self::CertifyArtifactWriteFailure(_) => 5,
            Self::NotImplemented(_) | Self::Internal(_) => 1,
        }
    }
}

#[derive(Debug, Parser)]
#[command(bin_name = "xtask")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Family(FamilyArgs),
}

#[derive(Debug, Args)]
struct FamilyArgs {
    #[command(subcommand)]
    command: FamilyCommand,
}

#[derive(Debug, Subcommand)]
enum FamilyCommand {
    New { family: String },
    Prove { family: String },
    Certify { family: String },
}

pub fn run() -> i32 {
    let workspace_root = match std::env::current_dir() {
        Ok(path) => path,
        Err(error) => {
            eprintln!("failed to resolve current directory: {error}");
            return XtaskError::Internal("failed to resolve current directory".to_string())
                .exit_code();
        }
    };

    run_from(&workspace_root, std::env::args_os())
}

pub fn run_from<I, S>(workspace_root: &Path, args: I) -> i32
where
    I: IntoIterator<Item = S>,
    S: Into<OsString> + Clone,
{
    match dispatch(workspace_root, args) {
        Ok(()) => 0,
        Err(error) => {
            eprintln!("{error}");
            error.exit_code()
        }
    }
}

pub fn dispatch<I, S>(workspace_root: &Path, args: I) -> Result<(), XtaskError>
where
    I: IntoIterator<Item = S>,
    S: Into<OsString> + Clone,
{
    let cli =
        Cli::try_parse_from(args).map_err(|error| XtaskError::InvalidInput(error.to_string()))?;

    match cli.command {
        Command::Family(args) => match args.command {
            FamilyCommand::New { family } => scaffold::run(workspace_root, &family),
            FamilyCommand::Prove { family } => prove::run(workspace_root, &family),
            FamilyCommand::Certify { family } => certify::run(workspace_root, &family),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::family::{
        certify,
        layout::validate_packet_layout,
        manifest::parse_manifest_file,
        paths::{FamilyId, PacketPaths, REQUIRED_BUCKETS},
        prove,
        report::{
            CERTIFY_ARTIFACT_NAME, CommandOutput, CommandRunner, PROVE_ARTIFACT_NAME,
            certification_report_path,
        },
    };
    use std::cell::RefCell;
    use std::collections::{HashMap, VecDeque};
    use std::fs;
    use std::path::Path;
    use tempfile::TempDir;

    #[test]
    fn family_new_creates_locked_scaffold() {
        let temp_dir = workspace_root();
        let code = run_from(
            temp_dir.path(),
            [
                "xtask",
                "family",
                "new",
                "function.wrapper.pipeline.chain3.v1",
            ],
        );

        assert_eq!(code, 0);

        let family = FamilyId::parse("function.wrapper.pipeline.chain3.v1").unwrap();
        let paths = PacketPaths::new(temp_dir.path(), family);

        assert!(paths.root.is_dir());
        assert!(paths.candidate.is_file());
        assert!(paths.manifest.is_file());

        let manifest = fs::read_to_string(&paths.manifest).unwrap();
        assert!(manifest.contains("schema_version = 1"));
        assert!(manifest.contains("kind = \"function\""));
        assert!(manifest.contains(
            "required_buckets = [\"aligned\", \"drift\", \"under_specified\", \"unsupported_near_miss\"]"
        ));

        let candidate = fs::read_to_string(&paths.candidate).unwrap();
        assert!(candidate.contains("## Aligned"));
        assert!(candidate.contains("## Drift"));
        assert!(candidate.contains("## Under Specified"));
        assert!(candidate.contains("## Unsupported Near Miss"));

        for bucket in REQUIRED_BUCKETS {
            let bucket_root = paths.fixtures.join(bucket);
            assert!(bucket_root.join("Cargo.toml").is_file());
            assert!(bucket_root.join("src/main.rs").is_file());
            assert!(bucket_root.join("units/namespace").is_dir());
        }
    }

    #[test]
    fn family_new_rejects_invalid_family_id_without_writes() {
        let temp_dir = workspace_root();
        let code = run_from(temp_dir.path(), ["xtask", "family", "new", "../bad"]);

        assert_eq!(code, 2);
        assert!(
            fs::read_dir(temp_dir.path().join("semantic-families"))
                .unwrap()
                .next()
                .is_none()
        );
    }

    #[test]
    fn family_new_rejects_existing_packet() {
        let temp_dir = workspace_root();
        let family = FamilyId::parse("function.wrapper.pipeline.chain3.v1").unwrap();
        let paths = PacketPaths::new(temp_dir.path(), family);
        fs::create_dir(paths.root).unwrap();

        let code = run_from(
            temp_dir.path(),
            [
                "xtask",
                "family",
                "new",
                "function.wrapper.pipeline.chain3.v1",
            ],
        );

        assert_eq!(code, 3);
    }

    #[cfg(unix)]
    #[test]
    fn family_new_rejects_packet_root_symlink() {
        use std::os::unix::fs::symlink;

        let temp_dir = workspace_root();
        let family = FamilyId::parse("function.wrapper.pipeline.chain3.v1").unwrap();
        let paths = PacketPaths::new(temp_dir.path(), family);
        let outside = TempDir::new().unwrap();
        symlink(outside.path(), &paths.root).unwrap();

        let code = run_from(
            temp_dir.path(),
            [
                "xtask",
                "family",
                "new",
                "function.wrapper.pipeline.chain3.v1",
            ],
        );

        assert_eq!(code, 2);
    }

    #[test]
    fn manifest_validation_accepts_locked_example() {
        let temp_dir = workspace_root();
        let family = FamilyId::parse("function.wrapper.pipeline.chain3.v1").unwrap();
        let paths = PacketPaths::new(temp_dir.path(), family.clone());
        write_string(
            &paths.root.join("family.toml"),
            r#"schema_version = 1
family = "function.wrapper.pipeline.chain3.v1"
kind = "function"
compatibility_key = "function.wrapper.pipeline.chain3.v1"
summary = "Straight-line three-call wrapper pipeline over supported function deps."

[routing]
precedence = 1
must_not_shadow = [
  "function.wrapper.pipeline.v1",
  "function.arithmetic_leaf.monotone_down_nonnegative.v1",
  "function.arithmetic_leaf.monotone_up.v1",
]

[shape]
dep_count = 3
control_flow = "straight_line_only"
return_style = "let_then_return_or_direct_return"
loops = false
branching = false
requires_supported_function_deps = true

[args]
threading = "ordered_passthrough"
allow_nested_argument_expressions = false
allow_literal_only_extra_args = false

[corpus]
required_buckets = ["aligned", "drift", "under_specified", "unsupported_near_miss"]
min_cases_per_bucket = 1

[truth_surface]
requires_refresh_via = ["spec test"]
preserve_only_via = ["spec build", "spec generate", "spec status", "spec export"]
requires_stale_demote = true

[gates]
gate_a = true
gate_b = true
gate_c = true
gate_d = true
"#,
        );

        parse_manifest_file(&paths.manifest, &family).unwrap();
    }

    #[test]
    fn manifest_validation_rejects_extra_top_level_keys() {
        let temp_dir = workspace_root();
        let family = FamilyId::parse("function.wrapper.pipeline.chain3.v1").unwrap();
        let paths = PacketPaths::new(temp_dir.path(), family.clone());
        write_string(
            &paths.root.join("family.toml"),
            r#"schema_version = 1
family = "function.wrapper.pipeline.chain3.v1"
kind = "function"
compatibility_key = "function.wrapper.pipeline.chain3.v1"
summary = "summary"
unexpected = true

[routing]
precedence = 1
must_not_shadow = ["function.wrapper.pipeline.v1"]

[shape]
dep_count = 3
control_flow = "straight_line_only"
return_style = "direct_return"
loops = false
branching = false
requires_supported_function_deps = true

[args]
threading = "ordered_passthrough"
allow_nested_argument_expressions = false
allow_literal_only_extra_args = false

[corpus]
required_buckets = ["aligned", "drift", "under_specified", "unsupported_near_miss"]
min_cases_per_bucket = 1

[truth_surface]
requires_refresh_via = ["spec test"]
preserve_only_via = ["spec build", "spec generate", "spec status", "spec export"]
requires_stale_demote = true

[gates]
gate_a = true
gate_b = true
gate_c = true
gate_d = true
"#,
        );

        let error = parse_manifest_file(&paths.manifest, &family).unwrap_err();
        assert!(matches!(error, XtaskError::InvalidInput(_)));
    }

    #[test]
    fn manifest_validation_rejects_wrong_bucket_contract() {
        let temp_dir = workspace_root();
        let family = FamilyId::parse("function.wrapper.pipeline.chain3.v1").unwrap();
        let paths = PacketPaths::new(temp_dir.path(), family.clone());
        write_string(
            &paths.root.join("family.toml"),
            r#"schema_version = 1
family = "function.wrapper.pipeline.chain3.v1"
kind = "function"
compatibility_key = "function.wrapper.pipeline.chain3.v1"
summary = "summary"

[routing]
precedence = 1
must_not_shadow = ["function.wrapper.pipeline.v1"]

[shape]
dep_count = 3
control_flow = "straight_line_only"
return_style = "direct_return"
loops = false
branching = false
requires_supported_function_deps = true

[args]
threading = "ordered_passthrough"
allow_nested_argument_expressions = false
allow_literal_only_extra_args = false

[corpus]
required_buckets = ["aligned", "drift"]
min_cases_per_bucket = 1

[truth_surface]
requires_refresh_via = ["spec test"]
preserve_only_via = ["spec build", "spec generate", "spec status", "spec export"]
requires_stale_demote = true

[gates]
gate_a = true
gate_b = true
gate_c = true
gate_d = true
"#,
        );

        let error = parse_manifest_file(&paths.manifest, &family).unwrap_err();
        assert!(matches!(error, XtaskError::InvalidInput(_)));
    }

    #[test]
    fn packet_layout_validation_accepts_locked_shape() {
        let temp_dir = workspace_root();
        let family = FamilyId::parse("function.wrapper.pipeline.chain3.v1").unwrap();
        let paths = PacketPaths::new(temp_dir.path(), family.clone());
        scaffold::run(temp_dir.path(), family.as_str()).unwrap();
        seed_valid_manifest(&paths.manifest, family.as_str());
        seed_valid_cases(&paths);

        let manifest = parse_manifest_file(&paths.manifest, &family).unwrap();
        let layout = validate_packet_layout(&paths.root, &manifest).unwrap();

        assert_eq!(layout.case_filenames.len(), 4);
    }

    #[test]
    fn packet_layout_validation_rejects_non_unit_spec_files() {
        let temp_dir = workspace_root();
        let family = FamilyId::parse("function.wrapper.pipeline.chain3.v1").unwrap();
        let paths = PacketPaths::new(temp_dir.path(), family.clone());
        scaffold::run(temp_dir.path(), family.as_str()).unwrap();
        seed_valid_manifest(&paths.manifest, family.as_str());
        seed_valid_cases(&paths);
        write_string(
            &paths
                .fixtures
                .join("aligned/units/namespace/not-allowed.txt"),
            "bad",
        );

        let manifest = parse_manifest_file(&paths.manifest, &family).unwrap();
        let error = validate_packet_layout(&paths.root, &manifest).unwrap_err();
        assert!(matches!(error, XtaskError::InvalidInput(_)));
    }

    #[cfg(unix)]
    #[test]
    fn packet_layout_validation_rejects_symlinks_anywhere_under_fixtures() {
        use std::os::unix::fs::symlink;

        let temp_dir = workspace_root();
        let family = FamilyId::parse("function.wrapper.pipeline.chain3.v1").unwrap();
        let paths = PacketPaths::new(temp_dir.path(), family.clone());
        scaffold::run(temp_dir.path(), family.as_str()).unwrap();
        seed_valid_manifest(&paths.manifest, family.as_str());
        seed_valid_cases(&paths);
        let outside = TempDir::new().unwrap();
        symlink(outside.path(), paths.fixtures.join("drift/src/linked")).unwrap();

        let manifest = parse_manifest_file(&paths.manifest, &family).unwrap();
        let error = validate_packet_layout(&paths.root, &manifest).unwrap_err();
        assert!(matches!(error, XtaskError::InvalidInput(_)));
    }

    #[test]
    fn packet_layout_validation_rejects_duplicate_case_filenames() {
        let temp_dir = workspace_root();
        let family = FamilyId::parse("function.wrapper.pipeline.chain3.v1").unwrap();
        let paths = PacketPaths::new(temp_dir.path(), family.clone());
        scaffold::run(temp_dir.path(), family.as_str()).unwrap();
        seed_valid_manifest(&paths.manifest, family.as_str());
        seed_valid_cases(&paths);
        let original = paths
            .fixtures
            .join("aligned/units/namespace/checkout_chain3_aligned.unit.spec");
        let duplicate = paths
            .fixtures
            .join("drift/units/namespace/checkout_chain3_aligned.unit.spec");
        write_string(&duplicate, &fs::read_to_string(&original).unwrap());

        let manifest = parse_manifest_file(&paths.manifest, &family).unwrap();
        let error = validate_packet_layout(&paths.root, &manifest).unwrap_err();
        assert!(matches!(error, XtaskError::InvalidInput(_)));
    }

    #[test]
    fn family_prove_writes_locked_report() {
        let temp_dir = workspace_root();
        let family = FamilyId::parse("function.wrapper.pipeline.chain3.v1").unwrap();
        let paths = PacketPaths::new(temp_dir.path(), family.clone());
        scaffold::run(temp_dir.path(), family.as_str()).unwrap();
        seed_valid_manifest(&paths.manifest, family.as_str());
        seed_valid_cases(&paths);
        seed_suite_sources(temp_dir.path(), false);

        let runner = FakeRunner::new(&[
            command_output(&["git", "rev-parse", "HEAD"], 0, "abc123\n"),
            command_output(&["rustc", "--version"], 0, "rustc 1.89.0\n"),
            command_output(
                &["date", "-u", "+%Y-%m-%dT%H:%M:%SZ"],
                0,
                "2026-04-27T18:00:00Z\n",
            ),
            command_output(
                &[
                    "cargo",
                    "test",
                    "-p",
                    "spec-core",
                    "m21_chain3_classifier_",
                    "--",
                    "--nocapture",
                ],
                0,
                "",
            ),
            command_output(
                &[
                    "cargo",
                    "test",
                    "-p",
                    "spec-cli",
                    "--test",
                    "cli",
                    "m21_chain3_truth_surface_",
                    "--",
                    "--nocapture",
                ],
                0,
                "",
            ),
            command_output(
                &[
                    "cargo",
                    "test",
                    "-p",
                    "spec-cli",
                    "--test",
                    "m14_regressions",
                    "m21_chain3_corpus_",
                    "--",
                    "--nocapture",
                ],
                0,
                "",
            ),
        ]);

        prove::run_with_runner(temp_dir.path(), family.as_str(), &runner).unwrap();

        let report_path = paths.artifacts.join(PROVE_ARTIFACT_NAME);
        let report: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(report_path).unwrap()).unwrap();
        assert_eq!(report["overall_status"], "pass");
        assert_eq!(report["gates"]["gate_a"]["status"], "pass");
        assert_eq!(report["gates"]["gate_b"]["status"], "pass");
        assert_eq!(report["gates"]["gate_c"]["status"], "pass");
        assert_eq!(report["gates"]["gate_d"]["status"], "fail");
        assert_eq!(report["suites"].as_array().unwrap().len(), 3);
    }

    #[test]
    fn family_certify_writes_success_report_only_on_full_success() {
        let temp_dir = workspace_root();
        let family = FamilyId::parse("function.wrapper.pipeline.chain3.v1").unwrap();
        let paths = PacketPaths::new(temp_dir.path(), family.clone());
        scaffold::run(temp_dir.path(), family.as_str()).unwrap();
        seed_valid_manifest(&paths.manifest, family.as_str());
        seed_valid_cases(&paths);
        seed_suite_sources(temp_dir.path(), true);

        let runner = FakeRunner::new(&[
            command_output(&["git", "rev-parse", "HEAD"], 0, "abc123\n"),
            command_output(&["rustc", "--version"], 0, "rustc 1.89.0\n"),
            command_output(
                &["date", "-u", "+%Y-%m-%dT%H:%M:%SZ"],
                0,
                "2026-04-27T18:00:00Z\n",
            ),
            command_output(
                &[
                    "cargo",
                    "test",
                    "-p",
                    "spec-core",
                    "m21_chain3_classifier_",
                    "--",
                    "--nocapture",
                ],
                0,
                "",
            ),
            command_output(
                &[
                    "cargo",
                    "test",
                    "-p",
                    "spec-cli",
                    "--test",
                    "cli",
                    "m21_chain3_truth_surface_",
                    "--",
                    "--nocapture",
                ],
                0,
                "",
            ),
            command_output(
                &[
                    "cargo",
                    "test",
                    "-p",
                    "spec-cli",
                    "--test",
                    "m14_regressions",
                    "m21_chain3_corpus_",
                    "--",
                    "--nocapture",
                ],
                0,
                "",
            ),
            command_output(
                &["date", "-u", "+%Y-%m-%dT%H:%M:%SZ"],
                0,
                "2026-04-27T18:10:00Z\n",
            ),
            command_output(
                &[
                    "cargo",
                    "test",
                    "-p",
                    "spec-core",
                    "m21_chain3_regression_",
                    "--",
                    "--nocapture",
                ],
                0,
                "",
            ),
            command_output(
                &[
                    "cargo",
                    "test",
                    "-p",
                    "spec-cli",
                    "--test",
                    "m14_regressions",
                    "m21_chain3_regression_",
                    "--",
                    "--nocapture",
                ],
                0,
                "",
            ),
        ]);

        certify::run_with_runner(temp_dir.path(), family.as_str(), &runner).unwrap();

        let certification_report = certification_report_path(&paths);
        let report: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(certification_report).unwrap()).unwrap();
        assert_eq!(report["overall_status"], "pass");
        assert_eq!(report["gates"]["gate_d"]["status"], "pass");
        assert_eq!(report["suites"].as_array().unwrap().len(), 5);
    }

    #[test]
    fn family_certify_keeps_previous_success_report_on_failed_gate_d() {
        let temp_dir = workspace_root();
        let family = FamilyId::parse("function.wrapper.pipeline.chain3.v1").unwrap();
        let paths = PacketPaths::new(temp_dir.path(), family.clone());
        scaffold::run(temp_dir.path(), family.as_str()).unwrap();
        seed_valid_manifest(&paths.manifest, family.as_str());
        seed_valid_cases(&paths);
        seed_suite_sources(temp_dir.path(), false);

        let cert_report_path = certification_report_path(&paths);
        write_string(&cert_report_path, "{\"previous\":true}\n");

        let runner = FakeRunner::new(&[
            command_output(&["git", "rev-parse", "HEAD"], 0, "abc123\n"),
            command_output(&["rustc", "--version"], 0, "rustc 1.89.0\n"),
            command_output(
                &["date", "-u", "+%Y-%m-%dT%H:%M:%SZ"],
                0,
                "2026-04-27T18:00:00Z\n",
            ),
            command_output(
                &[
                    "cargo",
                    "test",
                    "-p",
                    "spec-core",
                    "m21_chain3_classifier_",
                    "--",
                    "--nocapture",
                ],
                0,
                "",
            ),
            command_output(
                &[
                    "cargo",
                    "test",
                    "-p",
                    "spec-cli",
                    "--test",
                    "cli",
                    "m21_chain3_truth_surface_",
                    "--",
                    "--nocapture",
                ],
                0,
                "",
            ),
            command_output(
                &[
                    "cargo",
                    "test",
                    "-p",
                    "spec-cli",
                    "--test",
                    "m14_regressions",
                    "m21_chain3_corpus_",
                    "--",
                    "--nocapture",
                ],
                0,
                "",
            ),
            command_output(
                &["date", "-u", "+%Y-%m-%dT%H:%M:%SZ"],
                0,
                "2026-04-27T18:20:00Z\n",
            ),
            command_output(
                &[
                    "cargo",
                    "test",
                    "-p",
                    "spec-core",
                    "m21_chain3_regression_",
                    "--",
                    "--nocapture",
                ],
                0,
                "",
            ),
            command_output(
                &[
                    "cargo",
                    "test",
                    "-p",
                    "spec-cli",
                    "--test",
                    "m14_regressions",
                    "m21_chain3_regression_",
                    "--",
                    "--nocapture",
                ],
                0,
                "",
            ),
        ]);

        let error =
            certify::run_with_runner(temp_dir.path(), family.as_str(), &runner).unwrap_err();
        assert!(matches!(error, XtaskError::CertifySuiteFailure(_)));
        assert_eq!(
            fs::read_to_string(cert_report_path).unwrap(),
            "{\"previous\":true}\n"
        );
        assert_eq!(fs::read_dir(&paths.artifacts).unwrap().count(), 3);
        assert!(
            !paths
                .artifacts
                .join(CERTIFY_ARTIFACT_NAME)
                .with_extension("tmp")
                .exists()
        );
    }

    fn workspace_root() -> TempDir {
        let temp_dir = TempDir::new().unwrap();
        fs::create_dir(temp_dir.path().join("semantic-families")).unwrap();
        temp_dir
    }

    fn write_string(path: &Path, contents: &str) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(path, contents).unwrap();
    }

    fn seed_valid_manifest(manifest_path: &Path, family: &str) {
        write_string(
            manifest_path,
            &format!(
                r#"schema_version = 1
family = "{family}"
kind = "function"
compatibility_key = "{family}"
summary = "Straight-line three-call wrapper pipeline over supported function deps."

[routing]
precedence = 1
must_not_shadow = [
  "function.wrapper.pipeline.v1",
  "function.arithmetic_leaf.monotone_down_nonnegative.v1",
  "function.arithmetic_leaf.monotone_up.v1",
]

[shape]
dep_count = 3
control_flow = "straight_line_only"
return_style = "let_then_return_or_direct_return"
loops = false
branching = false
requires_supported_function_deps = true

[args]
threading = "ordered_passthrough"
allow_nested_argument_expressions = false
allow_literal_only_extra_args = false

[corpus]
required_buckets = ["aligned", "drift", "under_specified", "unsupported_near_miss"]
min_cases_per_bucket = 1

[truth_surface]
requires_refresh_via = ["spec test"]
preserve_only_via = ["spec build", "spec generate", "spec status", "spec export"]
requires_stale_demote = true

[gates]
gate_a = true
gate_b = true
gate_c = true
gate_d = true
"#
            ),
        );
    }

    fn seed_valid_cases(paths: &PacketPaths) {
        let fixtures = [
            ("aligned", "checkout_chain3_aligned.unit.spec"),
            ("drift", "checkout_chain3_drift.unit.spec"),
            (
                "under_specified",
                "checkout_chain3_under_specified.unit.spec",
            ),
            (
                "unsupported_near_miss",
                "checkout_chain3_unsupported_near_miss.unit.spec",
            ),
        ];

        for (bucket, filename) in fixtures {
            write_string(
                &paths
                    .fixtures
                    .join(bucket)
                    .join("units/namespace")
                    .join(filename),
                "kind: function\n",
            );
        }
    }

    fn seed_suite_sources(workspace_root: &Path, include_cli_regression: bool) {
        write_string(
            &workspace_root.join("spec-core/src/semantic_review.rs"),
            "fn m21_chain3_classifier_alpha() {}\nfn m21_chain3_regression_alpha() {}\n",
        );
        write_string(
            &workspace_root.join("spec-cli/tests/cli.rs"),
            "fn m21_chain3_truth_surface_alpha() {}\n",
        );
        let cli_regression = if include_cli_regression {
            "fn m21_chain3_regression_alpha() {}\n"
        } else {
            ""
        };
        write_string(
            &workspace_root.join("spec-cli/tests/m14_regressions.rs"),
            &format!("fn m21_chain3_corpus_alpha() {{}}\n{cli_regression}"),
        );
    }

    fn command_output(command: &[&str], exit_code: i32, stdout: &str) -> (String, CommandOutput) {
        (
            command.join("\u{1f}"),
            CommandOutput {
                exit_code,
                stdout: stdout.to_string(),
                stderr: String::new(),
            },
        )
    }

    struct FakeRunner {
        outputs: RefCell<HashMap<String, VecDeque<CommandOutput>>>,
    }

    impl FakeRunner {
        fn new(entries: &[(String, CommandOutput)]) -> Self {
            let mut outputs = HashMap::<String, VecDeque<CommandOutput>>::new();
            for (command, output) in entries {
                outputs
                    .entry(command.clone())
                    .or_default()
                    .push_back(output.clone());
            }
            Self {
                outputs: RefCell::new(outputs),
            }
        }
    }

    impl CommandRunner for FakeRunner {
        fn run(&self, _cwd: &Path, command: &[String]) -> CommandOutput {
            let key = command.join("\u{1f}");
            self.outputs
                .borrow_mut()
                .get_mut(&key)
                .and_then(VecDeque::pop_front)
                .unwrap_or_else(|| panic!("unexpected command: {command:?}"))
        }
    }
}
