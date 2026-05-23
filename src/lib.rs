//! Telugu text analysis plugin for INFINI Pizza.
//!
//! Provides a dedicated `"telugu"` analyzer with:
//!
//! ```text
//! StandardTokenizer → IndicNormalization → TeluguNormalization → Lowercase
//!     → DecimalDigit → Stop → TeluguStem
//! ```
//!
//! ## Components
//!
//! - **TeluguNormalizationFilter** — Telugu-specific normalizations:
//!   - Telugu digit normalization (౦-౯ → 0-9)
//!   - Zero-width character removal
//! - **TeluguStopFilter** — 90+ Telugu stop words with O(1) lookup
//! - Reuses `IndicNormalizationFilter`, `TeluguStemTokenFilter` from `analysis-core`

#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

mod normalize;
mod register;
mod stop;

pub use normalize::TeluguNormalizationFilter;
pub use register::register_all;
pub use stop::TeluguStopFilter;
