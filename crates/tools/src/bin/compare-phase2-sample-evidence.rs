use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};

const REQUIRED_SAMPLES: &[&str] = &["layer36-clock", "layer36-cat", "layer36-curl"];

fn main() -> Result<()> {
    let config = Config::parse(env::args().skip(1))?;
    let reports = config.load_reports()?;
    compare_reports(&reports, config.allow_blocked_curl)
}

#[derive(Debug, Clone)]
struct Config {
    linux: PathBuf,
    macos: PathBuf,
    windows: PathBuf,
    allow_blocked_curl: bool,
}

impl Config {
    fn parse(args: impl IntoIterator<Item = String>) -> Result<Self> {
        let mut linux = None;
        let mut macos = None;
        let mut windows = None;
        let mut allow_blocked_curl = false;

        let mut args = args.into_iter();
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--linux" => linux = Some(next_path(&mut args, "--linux")?),
                "--macos" => macos = Some(next_path(&mut args, "--macos")?),
                "--windows" => windows = Some(next_path(&mut args, "--windows")?),
                "--allow-blocked-curl" => allow_blocked_curl = true,
                "--help" | "-h" => {
                    println!(
                        "Usage: compare-phase2-sample-evidence --linux <md> --macos <md> --windows <md> [--allow-blocked-curl]"
                    );
                    std::process::exit(0);
                }
                _ => bail!("unknown argument `{arg}`"),
            }
        }

        let config = Self {
            linux: linux.context("--linux is required")?,
            macos: macos.context("--macos is required")?,
            windows: windows.context("--windows is required")?,
            allow_blocked_curl,
        };
        config.validate_paths()?;
        Ok(config)
    }

    fn validate_paths(&self) -> Result<()> {
        for (label, path) in [
            ("linux", &self.linux),
            ("macos", &self.macos),
            ("windows", &self.windows),
        ] {
            if !path.is_file() {
                bail!("{label} report path does not exist: {}", path.display());
            }
        }
        Ok(())
    }

    fn load_reports(&self) -> Result<Vec<HostReport>> {
        vec![
            HostReport::parse("linux", &self.linux),
            HostReport::parse("macos", &self.macos),
            HostReport::parse("windows", &self.windows),
        ]
        .into_iter()
        .collect()
    }
}

fn next_path(args: &mut impl Iterator<Item = String>, flag: &str) -> Result<PathBuf> {
    args.next()
        .map(PathBuf::from)
        .with_context(|| format!("{flag} requires a path"))
}

#[derive(Debug, Clone)]
struct HostReport {
    label: &'static str,
    source: PathBuf,
    host_line: String,
    sample_rows: BTreeMap<String, SampleRow>,
}

impl HostReport {
    fn parse(label: &'static str, path: &Path) -> Result<Self> {
        let source =
            fs::read_to_string(path).with_context(|| format!("read report {}", path.display()))?;
        let host_line = find_host_line(&source).context("missing host metadata line")?;
        let sample_rows = parse_sample_rows(&source)?;
        for required in REQUIRED_SAMPLES {
            if !sample_rows.contains_key(*required) {
                bail!(
                    "report {} is missing sample row `{required}`",
                    path.display()
                );
            }
        }
        Ok(Self {
            label,
            source: path.to_path_buf(),
            host_line,
            sample_rows,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SampleRow {
    sample: String,
    status: String,
    exit_code: Option<i32>,
    stdout_sha256: Option<String>,
}

fn find_host_line(source: &str) -> Option<String> {
    source
        .lines()
        .find(|line| line.trim_start().starts_with("- Host:"))
        .map(str::to_string)
}

fn parse_sample_rows(source: &str) -> Result<BTreeMap<String, SampleRow>> {
    let mut rows = BTreeMap::new();
    for line in source.lines() {
        let trimmed = line.trim();
        if !trimmed.starts_with("| layer36-") {
            continue;
        }
        let columns = trimmed
            .trim_start_matches('|')
            .trim_end_matches('|')
            .split('|')
            .map(|column| column.trim().to_string())
            .collect::<Vec<_>>();
        if columns.len() != 5 {
            bail!("invalid sample table row: `{line}`");
        }
        let sample = columns[0].clone();
        let status = columns[1].clone();
        let exit_code = parse_exit_code(&columns[2])?;
        let stdout_sha256 = parse_hash(&columns[3]);
        rows.insert(
            sample.clone(),
            SampleRow {
                sample,
                status,
                exit_code,
                stdout_sha256,
            },
        );
    }

    if rows.is_empty() {
        bail!("no sample rows found in evidence report");
    }
    Ok(rows)
}

fn parse_exit_code(value: &str) -> Result<Option<i32>> {
    if value == "n/a" {
        return Ok(None);
    }
    let parsed = value
        .parse::<i32>()
        .with_context(|| format!("invalid exit code `{value}`"))?;
    Ok(Some(parsed))
}

fn parse_hash(value: &str) -> Option<String> {
    let cleaned = value.trim_matches('`').trim();
    if cleaned == "n/a" || cleaned.is_empty() {
        None
    } else {
        Some(cleaned.to_string())
    }
}

fn compare_reports(reports: &[HostReport], allow_blocked_curl: bool) -> Result<()> {
    println!("Layer36 Phase 2 sample evidence comparison");
    for report in reports {
        println!(
            "- {}: {} ({})",
            report.label,
            report.source.display(),
            report.host_line
        );
    }

    for sample in REQUIRED_SAMPLES {
        compare_sample(reports, sample, allow_blocked_curl)?;
    }

    println!("comparison passed: sample stdout hashes are aligned");
    Ok(())
}

fn compare_sample(reports: &[HostReport], sample: &str, allow_blocked_curl: bool) -> Result<()> {
    let mut passed_hashes = Vec::new();
    let mut blocked_hosts = Vec::new();

    for report in reports {
        let row = report
            .sample_rows
            .get(sample)
            .with_context(|| format!("missing `{sample}` row in {}", report.source.display()))?;
        match row.status.as_str() {
            "passed" => {
                let exit = row.exit_code.unwrap_or(-1);
                if exit != 0 {
                    bail!(
                        "{} reports `{sample}` as passed but exit code is `{exit}`",
                        report.source.display()
                    );
                }
                let Some(hash) = &row.stdout_sha256 else {
                    bail!(
                        "{} reports `{sample}` as passed but stdout hash is missing",
                        report.source.display()
                    );
                };
                passed_hashes.push((report.label, hash.clone()));
            }
            "blocked" => blocked_hosts.push(report.label),
            other => {
                bail!(
                    "{} reports `{sample}` with unsupported status `{other}`",
                    report.source.display()
                )
            }
        }
    }

    if !blocked_hosts.is_empty() {
        if sample == "layer36-curl" && allow_blocked_curl {
            println!(
                "- {sample}: blocked on {} (allowed by flag)",
                blocked_hosts.join(", ")
            );
            return Ok(());
        }
        bail!(
            "{sample} is blocked on {}. Re-run on those hosts or pass --allow-blocked-curl for temporary curl-only exception.",
            blocked_hosts.join(", ")
        );
    }

    let first_hash = &passed_hashes[0].1;
    if passed_hashes.iter().any(|(_, hash)| hash != first_hash) {
        let detail = passed_hashes
            .iter()
            .map(|(host, hash)| format!("{host}={hash}"))
            .collect::<Vec<_>>()
            .join(", ");
        bail!("{sample} stdout hash mismatch across hosts: {detail}");
    }

    println!("- {sample}: match ({first_hash})");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_report(status_curl: &str, hash_suffix: &str) -> String {
        format!(
            r#"# Phase 2 Sample Evidence Run

## Host

- Host: `macos` / `aarch64`

## Results

| Sample | Status | Exit | Stdout SHA-256 | Stderr SHA-256 |
|---|---|---:|---|---|
| layer36-clock | passed | 0 | `clock{hash_suffix}` | `z` |
| layer36-cat | passed | 0 | `cat{hash_suffix}` | `z` |
| layer36-curl | {status_curl} | n/a | `n/a` | `n/a` |
"#
        )
    }

    #[test]
    fn parses_rows_from_markdown_table() {
        let rows = parse_sample_rows(&sample_report("blocked", "123")).expect("rows");
        assert_eq!(rows.len(), 3);
        assert_eq!(rows["layer36-clock"].status, "passed");
        assert_eq!(rows["layer36-curl"].status, "blocked");
    }

    #[test]
    fn compare_allows_blocked_curl_when_flag_set() {
        let make = |label: &'static str| HostReport {
            label,
            source: PathBuf::from(format!("{label}.md")),
            host_line: "- Host: `x` / `y`".to_string(),
            sample_rows: parse_sample_rows(&sample_report("blocked", "aaa")).expect("rows"),
        };
        compare_reports(&[make("linux"), make("macos"), make("windows")], true)
            .expect("comparison should pass");
    }

    #[test]
    fn compare_fails_when_hashes_differ() {
        let host = |label: &'static str, suffix: &str| HostReport {
            label,
            source: PathBuf::from(format!("{label}.md")),
            host_line: "- Host: `x` / `y`".to_string(),
            sample_rows: parse_sample_rows(&format!(
                r#"# Phase 2 Sample Evidence Run
## Host
- Host: `x` / `y`
## Results
| Sample | Status | Exit | Stdout SHA-256 | Stderr SHA-256 |
|---|---|---:|---|---|
| layer36-clock | passed | 0 | `clock{suffix}` | `z` |
| layer36-cat | passed | 0 | `cat{suffix}` | `z` |
| layer36-curl | passed | 0 | `curl{suffix}` | `z` |
"#
            ))
            .expect("rows"),
        };

        let result = compare_reports(
            &[host("linux", "1"), host("macos", "2"), host("windows", "1")],
            false,
        );
        assert!(result.is_err());
    }
}
