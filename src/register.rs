//! Registration of Telugu analysis components into the analysis factory.

use alloc::boxed::Box;
use alloc::vec;
use pizza_engine::analysis::AnalysisFactory;
use pizza_engine::analysis::Analyzer;
use pizza_engine::analysis::LowercaseTokenFilter;
use pizza_engine::analysis::StandardTokenizer;
use pizza_engine::analysis::TokenFilter;

use pizza_analysis_core::DecimalDigitTokenFilter;
use pizza_analysis_core::IndicNormalizationTokenFilter;
use pizza_analysis_core::TeluguStemTokenFilter;

use crate::normalize::TeluguNormalizationFilter;
use crate::stop::TeluguStopFilter;

/// Register all Telugu analysis components.
///
/// Registers:
/// - `"telugu"` analyzer (NEW: indic_norm → telugu_norm → lowercase → decimal_digit → stop → stem)
/// - `"telugu_normalization"` token filter
/// - `"telugu_stop"` token filter
pub fn register_all(factory: &mut AnalysisFactory) {
    factory.register_token_filter(
        "telugu_normalization",
        Box::new(TeluguNormalizationFilter::new()),
    );
    factory.register_token_filter("telugu_stop", Box::new(TeluguStopFilter::new()));

    let filters: Vec<Box<dyn TokenFilter>> = vec![
        Box::new(IndicNormalizationTokenFilter::new()),
        Box::new(TeluguNormalizationFilter::new()),
        Box::new(LowercaseTokenFilter::new()),
        Box::new(DecimalDigitTokenFilter::new()),
        Box::new(TeluguStopFilter::new()),
        Box::new(TeluguStemTokenFilter::new()),
    ];

    factory.register_analyzer(
        "telugu",
        Analyzer::new(vec![], Box::new(StandardTokenizer::new()), filters),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_all_no_panic() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
    }

    #[test]
    fn test_filters_registered() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
        assert!(factory.get_token_filter("telugu_normalization").is_some());
        assert!(factory.get_token_filter("telugu_stop").is_some());
    }

    #[test]
    fn test_analyzer_registered() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
        assert!(factory.get_analyzer("telugu").is_some());
    }

    #[test]
    fn test_end_to_end() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
        let analyzer = factory.get_analyzer("telugu").unwrap();
        let mut text = String::from("మరియు తెలుగు భాష");
        let tokens = analyzer.analyze_and_return_tokens(&mut text);
        let terms: Vec<&str> = tokens.iter().map(|t| t.term.as_ref()).collect();
        // "మరియు" is a stop word
        assert!(!terms.contains(&"మరియు"));
        assert!(!terms.is_empty());
    }
}
