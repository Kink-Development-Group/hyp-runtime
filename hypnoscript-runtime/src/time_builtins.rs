use chrono::{Datelike, Local, NaiveDate, Timelike};
use crate::builtin_trait::BuiltinModule;
use crate::localization::LocalizedMessage;

/// Time and date builtin functions
///
/// Provides comprehensive date/time operations including formatting,
/// calculations, and calendar functions.
pub struct TimeBuiltins;

impl BuiltinModule for TimeBuiltins {
    fn module_name() -> &'static str {
        "Time"
    }

    fn description() -> &'static str {
        "Date and time functions for timestamps, formatting, and calendar operations"
    }

    fn description_localized(locale: Option<&str>) -> String {
        let locale = crate::localization::detect_locale(locale);
        let msg = LocalizedMessage::new("Date and time functions for timestamps, formatting, and calendar operations")
            .with_translation("de", "Datums- und Zeitfunktionen für Zeitstempel, Formatierung und Kalenderoperationen")
            .with_translation("fr", "Fonctions de date et heure pour les horodatages, le formatage et les opérations calendaires")
            .with_translation("es", "Funciones de fecha y hora para marcas de tiempo, formato y operaciones de calendario");
        msg.resolve(&locale).to_string()
    }

    fn function_names() -> &'static [&'static str] {
        &[
            "GetCurrentTime", "GetCurrentDate", "GetCurrentTimeString", "GetCurrentDateTime",
            "FormatDateTime", "GetDayOfWeek", "GetDayOfYear", "IsLeapYear", "GetDaysInMonth",
            "GetYear", "GetMonth", "GetDay", "GetHour", "GetMinute", "GetSecond",
        ]
    }
}

impl TimeBuiltins {
    /// Get current Unix timestamp
    pub fn get_current_time() -> i64 {
        Local::now().timestamp()
    }

    /// Get current date as string
    pub fn get_current_date() -> String {
        Local::now().format("%Y-%m-%d").to_string()
    }

    /// Get current time as string
    pub fn get_current_time_string() -> String {
        Local::now().format("%H:%M:%S").to_string()
    }

    /// Get current date and time as string
    pub fn get_current_date_time() -> String {
        Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
    }

    /// Format current date time with custom format
    pub fn format_date_time(format: &str) -> String {
        Local::now().format(format).to_string()
    }

    /// Get day of week (0=Sunday, 6=Saturday)
    pub fn get_day_of_week() -> u32 {
        Local::now().weekday().num_days_from_sunday()
    }

    /// Get day of year
    pub fn get_day_of_year() -> u32 {
        Local::now().ordinal()
    }

    /// Check if year is leap year
    pub fn is_leap_year(year: i32) -> bool {
        NaiveDate::from_ymd_opt(year, 2, 29).is_some()
    }

    /// Get number of days in month
    pub fn get_days_in_month(year: i32, month: u32) -> Option<u32> {
        NaiveDate::from_ymd_opt(year, month, 1).and_then(|date| {
            if month == 12 {
                NaiveDate::from_ymd_opt(year + 1, 1, 1)
            } else {
                NaiveDate::from_ymd_opt(year, month + 1, 1)
            }
            .map(|next_month| (next_month - date).num_days() as u32)
        })
    }

    /// Get current year
    pub fn get_year() -> i32 {
        Local::now().year()
    }

    /// Get current month
    pub fn get_month() -> u32 {
        Local::now().month()
    }

    /// Get current day
    pub fn get_day() -> u32 {
        Local::now().day()
    }

    /// Get current hour
    pub fn get_hour() -> u32 {
        Local::now().hour()
    }

    /// Get current minute
    pub fn get_minute() -> u32 {
        Local::now().minute()
    }

    /// Get current second
    pub fn get_second() -> u32 {
        Local::now().second()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_leap_year() {
        assert!(TimeBuiltins::is_leap_year(2020));
        assert!(!TimeBuiltins::is_leap_year(2021));
        assert!(TimeBuiltins::is_leap_year(2000));
        assert!(!TimeBuiltins::is_leap_year(1900));
    }

    #[test]
    fn test_days_in_month() {
        assert_eq!(TimeBuiltins::get_days_in_month(2020, 2), Some(29)); // Leap year
        assert_eq!(TimeBuiltins::get_days_in_month(2021, 2), Some(28)); // Not leap year
        assert_eq!(TimeBuiltins::get_days_in_month(2021, 1), Some(31));
        assert_eq!(TimeBuiltins::get_days_in_month(2021, 4), Some(30));
    }
}
