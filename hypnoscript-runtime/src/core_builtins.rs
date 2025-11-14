use std::thread;
use std::time::Duration;
use crate::builtin_trait::BuiltinModule;
use crate::localization::LocalizedMessage;

/// Core I/O and hypnotic builtin functions
///
/// This module provides essential I/O operations and hypnotic-themed functions
/// with internationalization support.
pub struct CoreBuiltins;

impl BuiltinModule for CoreBuiltins {
    fn module_name() -> &'static str {
        "Core"
    }

    fn description() -> &'static str {
        "Core I/O, conversion, and hypnotic induction functions"
    }

    fn description_localized(locale: Option<&str>) -> String {
        let locale = crate::localization::detect_locale(locale);
        let msg = LocalizedMessage::new("Core I/O, conversion, and hypnotic induction functions")
            .with_translation("de", "Kern-I/O-, Konvertierungs- und hypnotische Induktionsfunktionen")
            .with_translation("fr", "Fonctions de base I/O, conversion et induction hypnotique")
            .with_translation("es", "Funciones básicas de I/O, conversión e inducción hipnótica");
        msg.resolve(&locale).to_string()
    }

    fn function_names() -> &'static [&'static str] {
        &[
            "observe", "whisper", "command", "drift",
            "DeepTrance", "HypnoticCountdown", "TranceInduction", "HypnoticVisualization",
            "ToInt", "ToDouble", "ToString", "ToBoolean",
        ]
    }
}

impl CoreBuiltins {
    /// Output a value with newline (observe)
    /// Standard output function in HypnoScript
    pub fn observe(value: &str) {
        println!("{}", value);
    }

    /// Output a value without newline (whisper)
    /// Used for continuous output without line breaks
    pub fn whisper(value: &str) {
        print!("{}", value);
        use std::io::{self, Write};
        let _ = io::stdout().flush();
    }

    /// Output a value in imperative/command style (command)
    /// Typically outputs in uppercase or emphasized format
    pub fn command(value: &str) {
        println!("{}", value.to_uppercase());
    }

    /// Wait for specified milliseconds (drift)
    pub fn drift(ms: u64) {
        thread::sleep(Duration::from_millis(ms));
    }

    /// Deep trance induction
    pub fn deep_trance(duration: u64) {
        Self::deep_trance_localized(duration, None);
    }

    /// Deep trance induction with locale support
    pub fn deep_trance_localized(duration: u64, locale: Option<&str>) {
        let locale = crate::localization::detect_locale(locale);

        let entering_msg = LocalizedMessage::new("Entering deep trance...")
            .with_translation("de", "Trete in tiefe Trance ein...")
            .with_translation("fr", "Entrer en transe profonde...")
            .with_translation("es", "Entrando en trance profundo...");

        let emerging_msg = LocalizedMessage::new("Emerging from trance...")
            .with_translation("de", "Aus der Trance auftauchen...")
            .with_translation("fr", "Émerger de la transe...")
            .with_translation("es", "Emergiendo del trance...");

        Self::observe(&entering_msg.resolve(&locale));
        Self::drift(duration);
        Self::observe(&emerging_msg.resolve(&locale));
    }

    /// Hypnotic countdown
    pub fn hypnotic_countdown(from: i64) {
        Self::hypnotic_countdown_localized(from, None);
    }

    /// Hypnotic countdown with locale support
    pub fn hypnotic_countdown_localized(from: i64, locale: Option<&str>) {
        let locale = crate::localization::detect_locale(locale);

        let sleepy_msg = LocalizedMessage::new("You are feeling very sleepy... {}")
            .with_translation("de", "Du fühlst dich sehr schläfrig... {}")
            .with_translation("fr", "Vous vous sentez très endormi... {}")
            .with_translation("es", "Te sientes muy somnoliento... {}");

        let trance_msg = LocalizedMessage::new("You are now in a deep hypnotic state.")
            .with_translation("de", "Du befindest dich jetzt in einem tiefen hypnotischen Zustand.")
            .with_translation("fr", "Vous êtes maintenant dans un état hypnotique profond.")
            .with_translation("es", "Ahora estás en un estado hipnótico profundo.");

        for i in (1..=from).rev() {
            let msg = sleepy_msg.resolve(&locale).replace("{}", &i.to_string());
            Self::observe(&msg);
            Self::drift(1000);
        }
        Self::observe(&trance_msg.resolve(&locale));
    }

    /// Trance induction
    pub fn trance_induction(subject_name: &str) {
        Self::trance_induction_localized(subject_name, None);
    }

    /// Trance induction with locale support
    pub fn trance_induction_localized(subject_name: &str, locale: Option<&str>) {
        let locale = crate::localization::detect_locale(locale);

        let welcome_msg = LocalizedMessage::new("Welcome {}, you are about to enter a deep trance...")
            .with_translation("de", "Willkommen {}, du wirst gleich in eine tiefe Trance eintreten...")
            .with_translation("fr", "Bienvenue {}, vous êtes sur le point d'entrer en transe profonde...")
            .with_translation("es", "Bienvenido {}, estás a punto de entrar en un trance profundo...");

        let breath_msg = LocalizedMessage::new("Take a deep breath and relax...")
            .with_translation("de", "Atme tief ein und entspanne dich...")
            .with_translation("fr", "Prenez une profonde inspiration et détendez-vous...")
            .with_translation("es", "Respira profundo y relájate...");

        let relaxed_msg = LocalizedMessage::new("With each breath, you feel more and more relaxed...")
            .with_translation("de", "Mit jedem Atemzug fühlst du dich mehr und mehr entspannt...")
            .with_translation("fr", "À chaque respiration, vous vous sentez de plus en plus détendu...")
            .with_translation("es", "Con cada respiración, te sientes más y más relajado...");

        let clear_msg = LocalizedMessage::new("Your mind is becoming clear and focused...")
            .with_translation("de", "Dein Geist wird klar und fokussiert...")
            .with_translation("fr", "Votre esprit devient clair et concentré...")
            .with_translation("es", "Tu mente se vuelve clara y enfocada...");

        Self::observe(&welcome_msg.resolve(&locale).replace("{}", subject_name));
        Self::drift(2000);
        Self::observe(&breath_msg.resolve(&locale));
        Self::drift(1500);
        Self::observe(&relaxed_msg.resolve(&locale));
        Self::drift(1500);
        Self::observe(&clear_msg.resolve(&locale));
        Self::drift(1000);
    }

    /// Hypnotic visualization
    pub fn hypnotic_visualization(scene: &str) {
        Self::hypnotic_visualization_localized(scene, None);
    }

    /// Hypnotic visualization with locale support
    pub fn hypnotic_visualization_localized(scene: &str, locale: Option<&str>) {
        let locale = crate::localization::detect_locale(locale);

        let imagine_msg = LocalizedMessage::new("Imagine yourself in {}...")
            .with_translation("de", "Stell dir vor, du bist in {}...")
            .with_translation("fr", "Imaginez-vous dans {}...")
            .with_translation("es", "Imagínate en {}...");

        let vivid_msg = LocalizedMessage::new("The colors are vivid, the sounds are clear...")
            .with_translation("de", "Die Farben sind lebendig, die Geräusche sind klar...")
            .with_translation("fr", "Les couleurs sont vives, les sons sont clairs...")
            .with_translation("es", "Los colores son vívidos, los sonidos son claros...");

        let peace_msg = LocalizedMessage::new("You feel completely at peace in this place...")
            .with_translation("de", "Du fühlst dich an diesem Ort vollkommen im Frieden...")
            .with_translation("fr", "Vous vous sentez complètement en paix dans cet endroit...")
            .with_translation("es", "Te sientes completamente en paz en este lugar...");

        Self::observe(&imagine_msg.resolve(&locale).replace("{}", scene));
        Self::drift(1500);
        Self::observe(&vivid_msg.resolve(&locale));
        Self::drift(1500);
        Self::observe(&peace_msg.resolve(&locale));
        Self::drift(1000);
    }

    /// Conversion functions
    pub fn to_int(value: f64) -> i64 {
        value as i64
    }

    pub fn to_double(value: &str) -> Result<f64, String> {
        value.parse::<f64>().map_err(|e| e.to_string())
    }

    pub fn to_string(value: &dyn std::fmt::Display) -> String {
        format!("{}", value)
    }

    pub fn to_boolean(value: &str) -> bool {
        matches!(value.to_lowercase().as_str(), "true" | "1" | "yes")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_int() {
        assert_eq!(CoreBuiltins::to_int(42.7), 42);
        assert_eq!(CoreBuiltins::to_int(-5.2), -5);
    }

    #[test]
    fn test_to_double() {
        // Test mit Werten, die nicht zu nahe an mathematischen Konstanten liegen
        assert_eq!(CoreBuiltins::to_double("42.75").unwrap(), 42.75);
        assert_eq!(CoreBuiltins::to_double("0.5").unwrap(), 0.5);
        assert!(CoreBuiltins::to_double("invalid").is_err());
    }

    #[test]
    fn test_to_boolean() {
        assert!(CoreBuiltins::to_boolean("true"));
        assert!(CoreBuiltins::to_boolean("True"));
        assert!(CoreBuiltins::to_boolean("1"));
        assert!(!CoreBuiltins::to_boolean("false"));
    }
}
