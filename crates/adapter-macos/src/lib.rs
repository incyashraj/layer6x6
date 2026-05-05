//! macOS host adapter surface for Layer36 Phase 2.
//!
//! This crate is the macOS ownership boundary. Shared behavior still comes from
//! `layer36-adapter-common`, while macOS-specific host wiring will land here.

use layer36_adapter_common::locale::HostLocale;

/// Host family handled by this adapter crate.
pub const HOST_FAMILY: &str = "macos";

/// Resolve locale and timezone for macOS host runs.
pub fn discover_locale(
    locale_override: Option<&str>,
    timezone_override: Option<&str>,
) -> HostLocale {
    HostLocale::from_env_with_overrides(locale_override, timezone_override)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn host_family_constant_matches_macos() {
        assert_eq!(HOST_FAMILY, "macos");
    }

    #[test]
    fn locale_discovery_applies_overrides() {
        let locale = discover_locale(Some("en-US"), Some("UTC"));
        assert_eq!(locale.current().bcp47, "en-US");
        assert_eq!(locale.timezone(), "UTC");
    }
}
