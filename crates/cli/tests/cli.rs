use std::path::PathBuf;
use std::process::Command;

use sha2::{Digest, Sha256};

fn layer36() -> Command {
    Command::new(env!("CARGO_BIN_EXE_layer36"))
}

#[test]
fn help_lists_phase_1_commands() {
    let output = layer36().arg("--help").output().expect("run layer36 help");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Commands:"));
    assert!(stdout.contains("run"));
    assert!(stdout.contains("version"));
    assert!(stdout.contains("doctor"));
}

#[test]
fn version_prints_runtime_metadata() {
    let output = layer36()
        .arg("version")
        .output()
        .expect("run layer36 version");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("layer36"));
    assert!(stdout.contains("wasmtime  43.0.2"));
    assert!(stdout.contains("rustc"));
    assert!(stdout.contains("commit"));
}

#[test]
fn doctor_lists_phase_1_tooling() {
    let output = layer36()
        .arg("doctor")
        .output()
        .expect("run layer36 doctor");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Layer36 doctor"));
    assert!(stdout.contains("cargo-component"));
    assert!(stdout.contains("wasm32-wasip1"));
    assert!(stdout.contains("wasm32-wasip2"));
    assert!(stdout.contains("state dir"));
}

#[test]
fn missing_input_returns_clear_error() {
    let output = layer36()
        .args(["run", "/definitely/not/a/component.wasm"])
        .output()
        .expect("run layer36 with missing input");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("input file does not exist"));
}

#[test]
fn configured_hello_component_runs_and_matches_expected_fixture_hash() {
    let Some(path) = configured_hello_component() else {
        return;
    };

    let wasm = std::fs::read(&path).expect("read configured hello component");
    let actual_hash = sha256_hex(&wasm);
    eprintln!("hello component sha256: {actual_hash}");

    if let Some(expected_hash) = expected_hello_hash() {
        assert_eq!(
            actual_hash, expected_hash,
            "configured hello component hash does not match the expected shared fixture"
        );
    }

    let output = layer36()
        .args(["run"])
        .arg(path)
        .output()
        .expect("run layer36 hello component");

    assert!(
        output.status.success(),
        "layer36 run failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.lines().collect::<Vec<_>>(), ["Hello, Layer36!"]);
}

#[test]
fn fuel_limit_exits_with_limit_code() {
    let Some(path) = configured_hello_component() else {
        return;
    };

    let output = layer36()
        .args(["run", "--fuel", "1"])
        .arg(path)
        .output()
        .expect("run layer36 hello component with low fuel");

    assert_eq!(output.status.code(), Some(4));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("limit exceeded: fuel exhausted"));
}

#[test]
fn memory_limit_exits_with_limit_code() {
    let Some(path) = configured_hello_component() else {
        return;
    };

    let output = layer36()
        .args(["run", "--mem-limit", "0"])
        .arg(path)
        .output()
        .expect("run layer36 hello component with low memory");

    assert_eq!(output.status.code(), Some(4));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("limit exceeded: memory limit exceeded"));
}

fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    let mut hex = String::with_capacity(digest.len() * 2);
    for byte in digest {
        use std::fmt::Write as _;
        write!(&mut hex, "{byte:02x}").expect("write to string");
    }
    hex
}

fn workspace_path(path: PathBuf) -> PathBuf {
    if path.is_absolute() || path.exists() {
        return path;
    }

    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(path)
}

fn configured_hello_component() -> Option<PathBuf> {
    let Some(path) = std::env::var_os("LAYER36_HELLO_WASM") else {
        eprintln!("skipping hello component test: LAYER36_HELLO_WASM is not set");
        return None;
    };

    Some(workspace_path(PathBuf::from(path)))
}

fn expected_hello_hash() -> Option<String> {
    std::env::var("LAYER36_HELLO_SHA256")
        .ok()
        .map(|value| value.trim().to_ascii_lowercase())
        .filter(|value| !value.is_empty())
}
