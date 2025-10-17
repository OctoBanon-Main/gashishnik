use once_cell::sync::Lazy;
use regex::Regex;

pub static ANSI_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\x1B(?:[@-Z\\-_]|\[[0-?]*[ -/]*[@-~])").expect("Invalid ANSI_REGEX")
});

pub static CONTROL_CHARS_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"[\x00-\x1F\x7F]").expect("Invalid CONTROL_CHARS_REGEX")
});

pub fn sanitize_message(input: &str) -> String {
    let no_ansi = ANSI_REGEX.replace_all(input, "");
    let clean = CONTROL_CHARS_REGEX.replace_all(&no_ansi, "");
    clean.to_string()
}