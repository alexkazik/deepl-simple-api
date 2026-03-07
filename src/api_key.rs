use reqwest::header::{HeaderValue, InvalidHeaderValue};

#[must_use]
pub(crate) struct ApiKey {
    api_key: String,
}

impl ApiKey {
    #[inline]
    pub(crate) fn new<S>(api_key: S) -> Self
    where
        S: AsRef<str>,
    {
        Self {
            api_key: format!("DeepL-Auth-Key {}", api_key.as_ref()),
        }
    }

    #[inline]
    pub(crate) fn get_url_translate(&self) -> &'static str {
        if self.api_key.ends_with(":fx") {
            "https://api-free.deepl.com/v2/translate"
        } else {
            "https://api.deepl.com/v2/translate"
        }
    }

    #[inline]
    pub(crate) fn get_url_usage(&self) -> &'static str {
        if self.api_key.ends_with(":fx") {
            "https://api-free.deepl.com/v2/usage"
        } else {
            "https://api.deepl.com/v2/usage"
        }
    }
}

impl TryFrom<&ApiKey> for HeaderValue {
    type Error = InvalidHeaderValue;

    #[inline]
    fn try_from(value: &ApiKey) -> Result<Self, Self::Error> {
        HeaderValue::try_from(&value.api_key)
    }
}
