//! Telugu stop word filter.

use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;
use hashbrown::HashSet;
use once_cell::sync::Lazy;
use pizza_engine::analysis::{Token, TokenFilter};

/// Standard Telugu stop words — pronouns, postpositions, conjunctions, auxiliaries.
static DEFAULT_STOP_WORDS: &[&str] = &[
    // Pronouns
    "నేను", "నీవు", "అతను", "ఆమె", "అది", "మేము", "మీరు", "వారు",
    "ఇది", "అవి", "ఇవి", "తాను", "ఆయన",
    // Demonstratives & question words
    "ఏమి", "ఎవరు", "ఎక్కడ", "ఎప్పుడు", "ఎలా", "ఎందుకు",
    // Postpositions / case markers
    "లో", "కు", "తో", "నుండి", "కి", "పై", "గురించి", "ద్వారా",
    "మీద", "కింద", "వరకు", "వల్ల", "కొరకు",
    // Conjunctions & connectors
    "మరియు", "లేదా", "కానీ", "కాబట్టి", "అందువల్ల", "ఎందుకంటే",
    "అయితే", "అయినప్పటికీ", "కాని",
    // Auxiliaries & copulas
    "ఉంది", "ఉన్నాయి", "ఉన్నారు", "ఉండేది", "ఉంటుంది", "అవుతుంది",
    "లేదు", "కాదు", "అవును",
    // Common verb forms
    "చేసి", "చేసిన", "చేయు", "వచ్చి", "వస్తుంది", "పోయి",
    "అని", "అన్న", "అనే", "అయిన", "అయి",
    // Particles & determiners
    "ఒక", "ఆ", "ఈ", "ఏ", "అన్ని", "కొన్ని", "చాలా",
    "ఇక్కడ", "అక్కడ", "ఇప్పుడు", "అప్పుడు",
    "మాత్రమే", "కూడా", "లాగా", "వంటి",
    // Numbers & quantifiers
    "అందరు", "ప్రతి", "కొంత", "ఎంత", "అంత", "ఇంత",
    // Common adverbs
    "మళ్ళీ", "ఇంకా", "అలా", "ఇలా",
    // Relative
    "అయిన", "అనే", "అన్ని", "వారి", "దాని",
];

static STOP_SET: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    DEFAULT_STOP_WORDS.iter().copied().collect()
});

/// Filters out common Telugu stop words.
#[derive(Clone, Debug)]
pub struct TeluguStopFilter {
    custom: Option<HashSet<String>>,
}

impl Default for TeluguStopFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl TeluguStopFilter {
    pub fn new() -> Self {
        Self { custom: None }
    }

    /// Create with a custom stop word list.
    pub fn with_words(words: &[&str]) -> Self {
        let set: HashSet<String> = words.iter().map(|w| w.to_string()).collect();
        Self { custom: Some(set) }
    }

    fn is_stop(&self, word: &str) -> bool {
        if let Some(ref custom) = self.custom {
            custom.contains(word)
        } else {
            STOP_SET.contains(word)
        }
    }
}

impl TokenFilter for TeluguStopFilter {
    fn filter<'a>(&self, token: &mut Token<'a>) -> (bool, Option<Vec<Token<'a>>>) {
        (self.is_stop(token.term.as_ref()), None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stop_word_detected() {
        let filter = TeluguStopFilter::new();
        let mut token = Token {
            term: Cow::Borrowed("మరియు"),
            start_offset: 0, end_offset: 0, position: 0,
        };
        let (removed, _) = filter.filter(&mut token);
        assert!(removed);
    }

    #[test]
    fn test_non_stop_word() {
        let filter = TeluguStopFilter::new();
        let mut token = Token {
            term: Cow::Borrowed("విశ్వవిద్యాలయం"),
            start_offset: 0, end_offset: 0, position: 0,
        };
        let (removed, _) = filter.filter(&mut token);
        assert!(!removed);
    }
}
