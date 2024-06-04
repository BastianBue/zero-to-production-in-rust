use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_valid_subscriber_name_can_be_constructed() {
        let subscriber_name = SubscriberName::parse("Ursula".into());
        claims::assert_ok!(subscriber_name);
    }

    #[test]
    fn a_subscriber_with_an_invalid_character_may_not_be_constructed() {
        let subscriber_name = SubscriberName::parse("{Ursula".into());
        claims::assert_err!(subscriber_name);
    }

    #[test]
    fn a_subscriber_name_cannot_be_empty() {
        let subscriber_name = SubscriberName::parse("".into());
        claims::assert_err!(subscriber_name);
    }

    #[test]
    fn a_subscriber_name_cannot_be_too_long() {
        let long_name = "a".repeat(257);
        let subscriber_name = SubscriberName::parse(long_name);
        claims::assert_err!(subscriber_name);
    }
}
