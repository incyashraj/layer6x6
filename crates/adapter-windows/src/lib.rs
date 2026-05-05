//! Windows host adapter surface for Layer36 Phase 2.
//!
//! This crate is the Windows ownership boundary. Shared behavior still comes
//! from `layer36-adapter-common`, while Windows-specific host wiring will land
//! here.

use layer36_adapter_common::{
    locale::{DateStyle, HostLocale, LocaleId, NumberStyle},
    time::HostClock,
};

/// Host family handled by this adapter crate.
pub const HOST_FAMILY: &str = "windows";

/// Resolve locale and timezone for Windows host runs.
pub fn discover_locale(
    locale_override: Option<&str>,
    timezone_override: Option<&str>,
) -> HostLocale {
    HostLocale::from_env_with_overrides(locale_override, timezone_override)
}

/// Build the Windows host clock surface.
pub fn discover_clock(test_time_millis: Option<u64>) -> HostClock {
    HostClock::new(test_time_millis)
}

/// Sleep through the Windows host adapter path.
pub fn sleep_millis(millis: u32) {
    HostClock::sleep_millis(millis);
}

/// Read the current locale through the Windows adapter path.
pub fn current_locale(locale: &HostLocale) -> LocaleId {
    locale.current()
}

/// Read the current timezone through the Windows adapter path.
pub fn timezone(locale: &HostLocale) -> String {
    locale.timezone()
}

/// Format a date through the Windows adapter path.
pub fn format_date(millis: u64, timezone: &str, style: DateStyle, locale: &LocaleId) -> String {
    HostLocale::format_date(millis, timezone, style, locale)
}

/// Format a number through the Windows adapter path.
pub fn format_number(value: f64, style: NumberStyle, locale: &LocaleId) -> String {
    HostLocale::format_number(value, style, locale)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn host_family_constant_matches_windows() {
        assert_eq!(HOST_FAMILY, "windows");
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

    #[test]
    fn sleep_hook_accepts_zero_millis() {
        sleep_millis(0);
    }

    #[test]
    fn locale_format_helpers_return_stable_values() {
        let locale = discover_locale(Some("en-US"), Some("UTC"));
        let current = current_locale(&locale);
        let tz = timezone(&locale);
        let date = format_date(1_234, &tz, DateStyle::Medium, &current);
        let number = format_number(42.5, NumberStyle::Decimal, &current);

        assert_eq!(current.bcp47, "en-US");
        assert_eq!(tz, "UTC");
        assert_eq!(date, "1970-01-01 00:00");
        assert_eq!(number, "42.5");
    }
}
