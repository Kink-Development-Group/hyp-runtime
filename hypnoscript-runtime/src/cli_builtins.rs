use std::collections::HashMap;
use std::io::{self, BufRead, Write};

use serde::{Deserialize, Serialize};

use crate::localization::{Locale, detect_locale};

/// Parsed command-line arguments (flags + positional values).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct ParsedArguments {
    /// Arguments that start with `-` or `--`.
    pub flags: HashMap<String, Option<String>>,
    /// Arguments without prefix or those following `--`.
    pub positional: Vec<String>,
}

/// Builtins tailored for CLI-style applications.
pub struct CliBuiltins;

impl CliBuiltins {
    /// Prompts the user for textual input.
    ///
    /// * `message` – Question displayed to the user.
    /// * `default` – Optional default value returned on empty input.
    /// * `allow_empty` – Whether empty input is accepted without a default.
    /// * `locale` – Optional locale hint (e.g., `"de"`, `"en-US"`).
    pub fn prompt(
        message: &str,
        default: Option<&str>,
        allow_empty: bool,
        locale: Option<&str>,
    ) -> io::Result<String> {
        let locale = detect_locale(locale);
        let mut stdout = io::stdout();
        let suffix = render_prompt_suffix(&locale, default);
        write!(stdout, "{}{}: ", message, suffix)?;
        stdout.flush()?;

        let stdin = io::stdin();
        let mut line = String::new();
        stdin.lock().read_line(&mut line)?;
        let trimmed = line.trim();

        if trimmed.is_empty() {
            if let Some(value) = default {
                return Ok(value.to_string());
            }
            if !allow_empty {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    render_empty_input_error(&locale),
                ));
            }
        }

        Ok(trimmed.to_string())
    }

    /// Prompts the user for a yes/no confirmation.
    pub fn confirm(message: &str, default: bool, locale: Option<&str>) -> io::Result<bool> {
        let locale = detect_locale(locale);
        let hint = yes_no_hint(&locale, default);
        let full_message = format!("{} {}", message, hint);

        loop {
            let answer = Self::prompt(&full_message, None, true, Some(locale.code()))?;
            if answer.trim().is_empty() {
                return Ok(default);
            }

            if is_yes(&answer, &locale) {
                return Ok(true);
            }
            if is_no(&answer, &locale) {
                return Ok(false);
            }

            println!("{}", invalid_confirmation_hint(&locale));
        }
    }

    /// Parses CLI-style arguments into flags + positional parts.
    pub fn parse_arguments(args: &[String]) -> ParsedArguments {
        let mut parsed = ParsedArguments::default();
        let mut iter = args.iter().peekable();
        let mut positional_mode = false;

        while let Some(arg) = iter.next() {
            if positional_mode {
                parsed.positional.push(arg.clone());
                continue;
            }

            if arg == "--" {
                positional_mode = true;
                continue;
            }

            if let Some(stripped) = arg.strip_prefix("--") {
                if let Some(value) = stripped.split_once('=') {
                    parsed
                        .flags
                        .insert(value.0.to_string(), Some(value.1.to_string()));
                } else if let Some(next) = iter.peek() {
                    if !next.starts_with('-') {
                        parsed
                            .flags
                            .insert(stripped.to_string(), Some(iter.next().unwrap().clone()));
                    } else {
                        parsed.flags.insert(stripped.to_string(), None);
                    }
                } else {
                    parsed.flags.insert(stripped.to_string(), None);
                }
                continue;
            }

            if let Some(stripped) = arg.strip_prefix('-') {
                if let Some((flag, value)) = stripped.split_once('=') {
                    parsed
                        .flags
                        .insert(flag.to_string(), Some(value.to_string()));
                    continue;
                }
                if stripped.len() > 1 {
                    for ch in stripped.chars() {
                        parsed.flags.insert(ch.to_string(), None);
                    }
                } else {
                    parsed.flags.insert(stripped.to_string(), None);
                }
                continue;
            }

            parsed.positional.push(arg.clone());
        }

        parsed
    }

    /// Returns whether the provided flag (without prefix) is set.
    pub fn has_flag(args: &[String], flag: &str) -> bool {
        Self::parse_arguments(args).flags.contains_key(flag)
    }

    /// Returns the value of the provided flag, if any.
    pub fn flag_value(args: &[String], flag: &str) -> Option<String> {
        Self::parse_arguments(args)
            .flags
            .get(flag)
            .cloned()
            .flatten()
    }
}

fn render_prompt_suffix(locale: &Locale, default: Option<&str>) -> String {
    match (locale.language(), default) {
        ("de", Some(value)) => format!(" (Standard: {value})"),
        (_, Some(value)) => format!(" (default: {value})"),
        _ => String::new(),
    }
}

fn render_empty_input_error(locale: &Locale) -> &'static str {
    match locale.language() {
        "de" => "Eingabe darf nicht leer sein.",
        _ => "Input cannot be empty.",
    }
}

fn yes_no_hint(locale: &Locale, default: bool) -> &'static str {
    match (locale.language(), default) {
        ("de", true) => "[J/n]",
        ("de", false) => "[j/N]",
        (_, true) => "[Y/n]",
        (_, false) => "[y/N]",
    }
}

fn invalid_confirmation_hint(locale: &Locale) -> &'static str {
    match locale.language() {
        "de" => "Bitte mit 'j' oder 'n' antworten.",
        _ => "Please answer with 'y' or 'n'.",
    }
}

fn is_yes(answer: &str, locale: &Locale) -> bool {
    let normalized = answer.trim().to_lowercase();
    match locale.language() {
        "de" => matches!(normalized.as_str(), "j" | "ja"),
        _ => matches!(normalized.as_str(), "y" | "yes"),
    }
}

fn is_no(answer: &str, locale: &Locale) -> bool {
    let normalized = answer.trim().to_lowercase();
    match locale.language() {
        "de" => matches!(normalized.as_str(), "n" | "nein"),
        _ => matches!(normalized.as_str(), "n" | "no"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_flags_and_positionals() {
        let args = vec![
            "--port".to_string(),
            "8080".to_string(),
            "-v".to_string(),
            "task".to_string(),
            "--feature=beta".to_string(),
        ];

        let parsed = CliBuiltins::parse_arguments(&args);
        assert_eq!(parsed.positional, vec!["task".to_string()]);
        assert_eq!(
            parsed.flags.get("port").cloned().flatten(),
            Some("8080".to_string())
        );
        assert!(parsed.flags.contains_key("v"));
        assert_eq!(
            parsed.flags.get("feature").cloned().flatten(),
            Some("beta".to_string())
        );
    }
}
