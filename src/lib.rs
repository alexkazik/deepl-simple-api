#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code)]
#![deny(unused_crate_dependencies)]
#![warn(missing_docs)]
#![allow(rustdoc::redundant_explicit_links)]

//! Simple `DeppL` API.
//!
//! Currently supported:
//! - translation
//! - usage
//!
//! Free and paid keys are supported.
//!
//! Uses [`reqwest`](::reqwest) as a http client, optionally supports blocking.
//!
//! ```no_run
//! # async fn test() -> Result<(), deepl_simple_api::Error> {
//! # use deepl_simple_api::{DeepL, Options, translate_parameter::*};
//! let deepl = DeepL::new("API-KEY-HERE");
//! let options = Options::builder()
//!     .params(&[&TargetLanguage("DE"), &SourceLanguage("EN"), &Formality::PreferLess])
//!     .build();
//! let translated = deepl.translate(&options, &["Hello World"]).await?;
//! # Ok(())
//! # }
//! ```

pub(crate) mod api_key;
pub(crate) mod async_impl;
#[cfg(feature = "blocking")]
pub mod blocking;
pub(crate) mod error;
pub(crate) mod options;
pub mod parameter;
pub(crate) mod translate;
pub mod translate_parameter;
pub(crate) mod usage;

pub use crate::async_impl::DeepL;
pub use crate::error::Error;
pub use crate::options::{Options, OptionsBuilder};
pub use crate::translate::Translation;
pub use crate::usage::Usage;
