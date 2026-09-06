//! Telugu-specific text normalization.

use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;
use pizza_engine::analysis::Token;
use pizza_engine::analysis::TokenFilter;

/// Telugu-specific normalization filter.
///
/// - Normalizes Telugu digits ౦-౯ (U+0C66-U+0C6F) to ASCII 0-9
/// - Removes zero-width characters (U+200B, U+200C, U+200D)
#[derive(Clone, Debug, Default)]
pub struct TeluguNormalizationFilter;

impl TeluguNormalizationFilter {
    pub fn new() -> Self {
        Self
    }
}

impl TokenFilter for TeluguNormalizationFilter {
    fn filter<'a>(&self, token: &mut Token<'a>) -> (bool, Option<Vec<Token<'a>>>) {
        let text = token.term.as_ref();
        if text.is_empty() {
            return (false, None);
        }

        let needs_work = text.chars().any(|c| {
            let cp = c as u32;
            (0x0C66..=0x0C6F).contains(&cp) || (0x200B..=0x200D).contains(&cp)
        });

        if !needs_work {
            return (false, None);
        }

        let mut result = String::with_capacity(text.len());
        let mut changed = false;

        for c in text.chars() {
            let cp = c as u32;
            match cp {
                0x200B..=0x200D => {
                    changed = true;
                }
                0x0C66..=0x0C6F => {
                    result.push(char::from(b'0' + (cp - 0x0C66) as u8));
                    changed = true;
                }
                _ => {
                    result.push(c);
                }
            }
        }

        if changed {
            token.term = Cow::Owned(result);
        }
        (false, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_token(term: &str) -> Token {
        Token {
            term: Cow::Owned(term.to_string()),
            start_offset: 0,
            end_offset: term.len() as u32,
            position: 0,
        }
    }

    #[test]
    fn test_telugu_digit_normalization() {
        let filter = TeluguNormalizationFilter::new();
        let mut token = make_token("౧౨౩");
        filter.filter(&mut token);
        assert_eq!(token.term.as_ref(), "123");
    }

    #[test]
    fn test_zero_width_removal() {
        let filter = TeluguNormalizationFilter::new();
        let mut token = make_token("తెలు\u{200D}గు");
        filter.filter(&mut token);
        assert_eq!(token.term.as_ref(), "తెలుగు");
    }

    #[test]
    fn test_no_change_plain_telugu() {
        let filter = TeluguNormalizationFilter::new();
        let mut token = make_token("తెలుగు");
        filter.filter(&mut token);
        assert_eq!(token.term.as_ref(), "తెలుగు");
    }
}
