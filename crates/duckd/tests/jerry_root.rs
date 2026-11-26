//! Jerry Manager runs `duckd rkp-device ...` without a Duck ToolBox module, so a plain
//! data directory (no module.prop / bin / ui) must be accepted as DUCK_TOOLBOX_ROOT.

use std::{path::Path, process::Command};

use serde_json::Value;

fn run(root: Option<&Path>, data: &Path) -> Value {
    let mut command = Command::new(env!("CARGO_BIN_EXE_duckd"));
    command
        .args(["rkp-device", "status"])
        .env("DUCK_TOOLBOX_DATA_ROOT", data)
        .env_remove("DUCK_TOOLBOX_ROOT")
        .current_dir(data);
    if let Some(root) = root {
        command.env("DUCK_TOOLBOX_ROOT", root);
    }
    let output = command.output().expect("run duckd");
    serde_json::from_slice(&output.stdout).expect("one JSON envelope on stdout")
}

fn assert_booted(envelope: &Value) {
    assert_ne!(
        envelope["command"], "bootstrap",
        "root discovery failed: {envelope}"
    );
    assert_ne!(envelope["error"]["code"], "bootstrap_error", "{envelope}");
}

#[test]
fn bare_data_directory_is_a_valid_root() {
    let dir = tempfile::tempdir().unwrap();
    assert_booted(&run(Some(dir.path()), dir.path()));
}

#[test]
fn data_root_alone_is_enough_when_no_root_is_given() {
    let dir = tempfile::tempdir().unwrap();
    assert_booted(&run(None, dir.path()));
}

#[test]
fn a_run_leaves_no_var_logs_or_tmp_behind() {
    let data = tempfile::tempdir().unwrap();
    let scratch = tempfile::tempdir().unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_duckd"))
        .args(["rkp-device", "status"])
        .env("DUCK_TOOLBOX_ROOT", data.path())
        .env("DUCK_TOOLBOX_DATA_ROOT", data.path())
        .env("TMPDIR", scratch.path())
        .env_remove("DUCK_TOOLBOX_LOG")
        .output()
        .expect("run duckd");
    assert!(!output.stdout.is_empty());

    let left: Vec<_> = std::fs::read_dir(data.path())
        .unwrap()
        .map(|entry| entry.unwrap().file_name())
        .collect();
    assert!(left.is_empty(), "duckd left files behind: {left:?}");
}
