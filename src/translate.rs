use crate::parameter::ToStaticParam;
use crate::translate_parameter::NoTargetLanguage;
use crate::{Error, Options};
use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct Translations {
    pub(crate) translations: Vec<Translation>,
}

/// Response of the `translation` call.
#[derive(Deserialize)]
pub struct Translation {
    /// `billed_characters`, or None if not present in the result (use [`ShowBilledCharacters(true)`](crate::translate_parameter::ShowBilledCharacters) to enable it).
    pub billed_characters: Option<u64>,
    /// `detected_source_language`.
    pub detected_source_language: String,
    /// `text`.
    pub text: String,
}

pub(crate) fn check_translate<S>(options: &Options, texts: &[S]) -> Result<(), Error> {
    if !options.contains_key(NoTargetLanguage.to_static_param().0) {
        Err(Error::NoTargetLanguage)
    } else if texts.is_empty() {
        Err(Error::NoTexts)
    } else if options.contains_key("text") {
        Err(Error::OptionTextSet)
    } else if options.contains_key("context") {
        Err(Error::OptionContextSet)
    } else {
        Ok(())
    }
}
