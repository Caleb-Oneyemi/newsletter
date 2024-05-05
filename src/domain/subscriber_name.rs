use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct SubscriberName(String);

impl SubscriberName {
    /// Returns an instance of `SubscriberName` if the input satisfies all constraints.
    /// Panics otherwise.
    pub fn parse(s: String) -> Self {
        let is_empty: bool = s.trim().is_empty();
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_characters = s.chars().any(|c| forbidden_characters.contains(&c));

        // A grapheme is defined by the Unicode standard as a "user-perceived" character:
        // a character like `å` is a single grapheme, but it is actually composed of two characters.
        // .chars().count does not provide full unicode support
        let is_too_long = s.graphemes(true).count() > 256;

        if is_empty || contains_forbidden_characters || is_too_long {
            panic!("invalid name {}", s)
        } else {
            Self(s)
        }
    }
}

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
