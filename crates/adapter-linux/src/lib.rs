//! Linux host adapter surface for Layer36 Phase 2.
//!
//! This crate is the Linux ownership boundary. Shared behavior still comes from
//! `layer36-adapter-common`, while Linux-specific host wiring will land here.

use layer36_adapter_common::{locale::HostLocale, time::HostClock};

/// Host family handled by this adapter crate.
pub const HOST_FAMILY: &str = "linux";

/// Resolve locale and timezone for Linux host runs.
pub fn discover_locale(
    locale_override: Option<&str>,
    timezone_override: Option<&str>,
) -> HostLocale {
    HostLocale::from_env_with_overrides(locale_override, timezone_override)
}

/// Build the Linux host clock surface.
pub fn discover_clock(test_time_millis: Option<u64>) -> HostClock {
    HostClock::new(test_time_millis)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn host_family_constant_matches_linux() {
        assert_eq!(HOST_FAMILY, "linux");
    }

    #[test]
    fn locale_discovery_applies_overrides() {
        let locale = discover_locale(Some("en-US"), Some("UTC"));
        assert_eq!(locale.current().bcp47, "en-US");
        assert_eq!(locale.timezone(), "UTC");
    }

    #[test]
    fn clock_discovery_applies_fixed_time_override() {
        let clock = discover_clock(Some(1_777));
        assert_eq!(clock.now_millis().expect("fixed clock"), 1_777);
    }
}
