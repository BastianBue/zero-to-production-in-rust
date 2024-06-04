use unicode_segmentation::UnicodeSegmentation;

pub struct SubscriberName(String);

impl SubscriberName {
    pub fn parse(s: String) -> Result<SubscriberName, String> {
        let is_empty_or_whitespace = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > 256;
        let forbidden_chars = ['/', '(', ')', '"', '<', '>', '\\', '{', '}', '[', ']'];
        let contains_forbidden_chars = s.chars().any(|c| forbidden_chars.contains(&c));
        if is_empty_or_whitespace || is_too_long || contains_forbidden_chars {
            Err(format!("Invalid subscriber name {}", s).into())
        } else {
            Ok(SubscriberName(s))
        }
    }
}

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
