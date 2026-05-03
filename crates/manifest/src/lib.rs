//! Phase 2 sidecar manifest parser.
//!
//! A Layer36 Phase 2 app is still a plain `.wasm` component, but it may sit
//! next to a `manifest.toml` that declares identity, entry world, and requested
//! capabilities.

use std::{
    collections::BTreeSet,
    fmt,
    path::{Path, PathBuf},
    str::FromStr,
};

use serde::Deserialize;
use thiserror::Error;

pub const PHASE2_CLI_WORLD: &str = "layer36:app/cli@0.1.0";

const DEFAULT_GRANTED_CAPS: &[&str] = &[
    "io.stdout",
    "io.stderr",
    "io.args",
    "io.log",
    "time.clock",
    "time.monotonic",
    "time.sleep",
    "locale.info",
    "locale.format",
];

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    pub app: App,
    #[serde(default)]
    pub capabilities: Vec<CapabilityRequest>,
}

impl Manifest {
    pub fn parse(input: &str) -> Result<Self> {
        let manifest: Self =
            toml::from_str(input).map_err(|err| ManifestError::Toml(err.to_string()))?;
        manifest.validate()?;
        Ok(manifest)
    }

    pub fn parse_file(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let input = std::fs::read_to_string(path).map_err(|source| ManifestError::Io {
            path: path.to_path_buf(),
            source,
        })?;
        Self::parse(&input)
    }

    pub fn declared_capabilities(&self) -> Result<Vec<Capability>> {
        self.capabilities
            .iter()
            .map(|request| request.cap.parse())
            .collect()
    }

    pub fn required_capabilities(&self) -> Result<Vec<Capability>> {
        self.capabilities
            .iter()
            .filter(|request| request.required)
            .map(|request| request.cap.parse())
            .collect()
    }

    fn validate(&self) -> Result<()> {
        validate_app_id(&self.app.id)?;
        validate_required("app.name", &self.app.name)?;
        validate_required("app.version", &self.app.version)?;
        validate_required_path("app.entry", &self.app.entry)?;

        if self.app.world != PHASE2_CLI_WORLD {
            return Err(ManifestError::UnsupportedWorld {
                world: self.app.world.clone(),
            });
        }

        let mut seen = BTreeSet::new();
        for request in &self.capabilities {
            let cap: Capability = request.cap.parse()?;
            validate_required("capability.rationale", &request.rationale)?;
            if !seen.insert(cap.to_string()) {
                return Err(ManifestError::DuplicateCapability {
                    cap: request.cap.clone(),
                });
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct App {
    pub id: String,
    pub name: String,
    pub version: String,
    pub entry: PathBuf,
    pub world: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CapabilityRequest {
    pub cap: String,
    pub rationale: String,
    pub required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Capability {
    module: String,
    action: String,
    resource: Option<String>,
}

impl Capability {
    pub fn new(module: &str, action: &str, resource: Option<&str>) -> Result<Self> {
        validate_ident("capability module", module)?;
        validate_ident("capability action", action)?;

        let cap_name = format!("{module}.{action}");
        let resource_required = capability_resource_required(module, action).ok_or_else(|| {
            ManifestError::InvalidCapability {
                cap: cap_name.clone(),
                reason: "unknown Phase 2 capability".to_string(),
            }
        })?;
        let resource_was_present = resource.is_some();
        let resource = resource
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(ToOwned::to_owned);

        match (resource_required, resource.as_ref(), resource_was_present) {
            (true, None, _) => {
                return Err(ManifestError::InvalidCapability {
                    cap: cap_name,
                    reason: "this capability requires a resource after `:`".to_string(),
                });
            }
            (false, Some(_), _) | (false, None, true) => {
                return Err(ManifestError::InvalidCapability {
                    cap: cap_name,
                    reason: "this capability does not take a resource".to_string(),
                });
            }
            _ => {}
        }

        Ok(Self {
            module: module.to_owned(),
            action: action.to_owned(),
            resource,
        })
    }

    pub fn module(&self) -> &str {
        &self.module
    }

    pub fn action(&self) -> &str {
        &self.action
    }

    pub fn resource(&self) -> Option<&str> {
        self.resource.as_deref()
    }

    pub fn is_default_granted(&self) -> bool {
        default_granted_capabilities().contains(self)
    }
}

impl FromStr for Capability {
    type Err = ManifestError;

    fn from_str(input: &str) -> Result<Self> {
        let input = input.trim();
        let (module, rest) =
            input
                .split_once('.')
                .ok_or_else(|| ManifestError::InvalidCapability {
                    cap: input.to_owned(),
                    reason: "expected <module>.<action>[:resource]".to_string(),
                })?;
        let (action, resource) = match rest.split_once(':') {
            Some((action, resource)) => (action, Some(resource)),
            None => (rest, None),
        };

        Self::new(module, action, resource).map_err(|err| match err {
            ManifestError::InvalidIdentifier { field, reason } => {
                ManifestError::InvalidCapability {
                    cap: input.to_owned(),
                    reason: format!("{field}: {reason}"),
                }
            }
            other => other,
        })
    }
}

impl fmt::Display for Capability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.module, self.action)?;
        if let Some(resource) = &self.resource {
            write!(f, ":{resource}")?;
        }
        Ok(())
    }
}

pub fn default_granted_capabilities() -> BTreeSet<Capability> {
    DEFAULT_GRANTED_CAPS
        .iter()
        .map(|cap| cap.parse().expect("default capability strings are valid"))
        .collect()
}

#[derive(Debug, Error)]
pub enum ManifestError {
    #[error("failed to read manifest {path}: {source}")]
    Io {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("failed to parse manifest TOML: {0}")]
    Toml(String),
    #[error("missing required field `{0}`")]
    MissingField(&'static str),
    #[error("invalid app id `{id}`: {reason}")]
    InvalidAppId { id: String, reason: String },
    #[error("unsupported app world `{world}`")]
    UnsupportedWorld { world: String },
    #[error("invalid {field}: {reason}")]
    InvalidIdentifier { field: &'static str, reason: String },
    #[error("invalid capability `{cap}`: {reason}")]
    InvalidCapability { cap: String, reason: String },
    #[error("duplicate capability `{cap}`")]
    DuplicateCapability { cap: String },
}

pub type Result<T> = std::result::Result<T, ManifestError>;

fn validate_required(field: &'static str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        Err(ManifestError::MissingField(field))
    } else {
        Ok(())
    }
}

fn validate_required_path(field: &'static str, value: &Path) -> Result<()> {
    if value.as_os_str().is_empty() {
        Err(ManifestError::MissingField(field))
    } else {
        Ok(())
    }
}

fn validate_app_id(id: &str) -> Result<()> {
    validate_required("app.id", id)?;

    let parts = id.split('.').collect::<Vec<_>>();
    if parts.len() < 2 {
        return Err(ManifestError::InvalidAppId {
            id: id.to_owned(),
            reason: "use reverse-DNS form, for example com.example.app".to_string(),
        });
    }

    for part in parts {
        if part.is_empty()
            || !part
                .chars()
                .all(|ch| ch.is_ascii_alphanumeric() || ch == '-' || ch == '_')
        {
            return Err(ManifestError::InvalidAppId {
                id: id.to_owned(),
                reason: "segments may only contain ASCII letters, numbers, hyphen, or underscore"
                    .to_string(),
            });
        }
    }

    Ok(())
}

fn validate_ident(field: &'static str, value: &str) -> Result<()> {
    if value.is_empty() {
        return Err(ManifestError::InvalidIdentifier {
            field,
            reason: "value is empty".to_string(),
        });
    }

    if !value
        .chars()
        .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '-')
    {
        return Err(ManifestError::InvalidIdentifier {
            field,
            reason: "use lowercase ASCII letters, numbers, or hyphen".to_string(),
        });
    }

    Ok(())
}

fn capability_resource_required(module: &str, action: &str) -> Option<bool> {
    match (module, action) {
        ("io", "stdin" | "stdout" | "stderr" | "args" | "log") => Some(false),
        ("fs", "read" | "write" | "list" | "remove" | "mkdir") => Some(true),
        ("net", "connect") => Some(true),
        ("time", "clock" | "monotonic" | "sleep") => Some(false),
        ("locale", "info" | "format") => Some(false),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"
        [app]
        id = "com.example.hello"
        name = "Hello"
        version = "1.0.0"
        entry = "hello.wasm"
        world = "layer36:app/cli@0.1.0"

        [[capabilities]]
        cap = "fs.read:~/Documents/notes/**"
        rationale = "Read saved notes"
        required = true

        [[capabilities]]
        cap = "net.connect:api.example.com:443"
        rationale = "Sync to cloud"
        required = false
    "#;

    #[test]
    fn parses_phase_2_manifest_schema() {
        let manifest = Manifest::parse(EXAMPLE).expect("parse manifest");

        assert_eq!(manifest.app.id, "com.example.hello");
        assert_eq!(manifest.app.entry, PathBuf::from("hello.wasm"));
        assert_eq!(manifest.capabilities.len(), 2);

        let caps = manifest
            .declared_capabilities()
            .expect("declared capabilities");
        assert_eq!(caps[0].module(), "fs");
        assert_eq!(caps[0].action(), "read");
        assert_eq!(caps[0].resource(), Some("~/Documents/notes/**"));
    }

    #[test]
    fn rejects_unsupported_worlds() {
        let input = EXAMPLE.replace(PHASE2_CLI_WORLD, "layer36:app/gui@0.1.0");
        let err = Manifest::parse(&input).expect_err("reject unsupported world");

        assert!(matches!(err, ManifestError::UnsupportedWorld { .. }));
    }

    #[test]
    fn rejects_duplicate_capabilities() {
        let input = format!(
            "{EXAMPLE}\n[[capabilities]]\ncap = \"fs.read:~/Documents/notes/**\"\nrationale = \"again\"\nrequired = true\n"
        );
        let err = Manifest::parse(&input).expect_err("reject duplicate capability");

        assert!(matches!(err, ManifestError::DuplicateCapability { .. }));
    }

    #[test]
    fn parses_capability_parts() {
        let cap: Capability = "net.connect:api.example.com:443"
            .parse()
            .expect("parse cap");

        assert_eq!(cap.module(), "net");
        assert_eq!(cap.action(), "connect");
        assert_eq!(cap.resource(), Some("api.example.com:443"));
        assert_eq!(cap.to_string(), "net.connect:api.example.com:443");
    }

    #[test]
    fn rejects_unknown_capability_names() {
        let err = "net.listen:127.0.0.1:8080"
            .parse::<Capability>()
            .expect_err("reject unknown cap");

        assert!(matches!(err, ManifestError::InvalidCapability { .. }));
    }

    #[test]
    fn rejects_missing_required_resource() {
        let err = "fs.read"
            .parse::<Capability>()
            .expect_err("reject missing resource");

        assert!(matches!(err, ManifestError::InvalidCapability { .. }));
    }

    #[test]
    fn rejects_resource_on_resource_free_capability() {
        let err = "io.stdout:terminal"
            .parse::<Capability>()
            .expect_err("reject extra resource");

        assert!(matches!(err, ManifestError::InvalidCapability { .. }));
    }

    #[test]
    fn tracks_default_grants() {
        let stdout: Capability = "io.stdout".parse().expect("parse stdout cap");
        let fs_read: Capability = "fs.read:./data/**".parse().expect("parse fs cap");

        assert!(stdout.is_default_granted());
        assert!(!fs_read.is_default_granted());
    }
}
