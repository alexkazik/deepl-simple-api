//! Blocking API.

use crate::api_key::ApiKey;
use crate::translate::{Translations, check_translate};
use crate::usage::Usage;
use crate::{Error, Options, Translation};
use reqwest::StatusCode;
use reqwest::blocking::{Client, RequestBuilder};
use serde::de::DeserializeOwned;

/// `DeepL` Client.
#[must_use]
pub struct DeepL {
    client: Client,
    api_key: ApiKey,
}

impl DeepL {
    /// Create a new `DeepL` API client.
    pub fn new<S>(api_key: S) -> Self
    where
        S: AsRef<str>,
    {
        Self {
            client: Client::new(),
            api_key: ApiKey::new(api_key),
        }
    }

    /// Call the translate API.
    ///
    /// # Errors
    ///
    /// Will return `Err` when there was a problem with the API call or the `texts` slice is empty.
    pub fn translate<S>(&self, options: &Options, texts: &[S]) -> Result<Vec<Translation>, Error>
    where
        S: AsRef<str>,
    {
        self.translate_opt_context::<&str, S>(options, &None, texts)
    }

    /// Call the translate API.
    ///
    /// # Errors
    ///
    /// Will return `Err` when there was a problem with the API call or the `texts` slice is empty.
    pub fn translate_with_context<C, S>(
        &self,
        options: &Options,
        context: &C,
        texts: &[S],
    ) -> Result<Vec<Translation>, Error>
    where
        C: AsRef<str>,
        S: AsRef<str>,
    {
        self.translate_opt_context(options, &Some(context), texts)
    }

    /// Call the translate API.
    ///
    /// # Errors
    ///
    /// Will return `Err` when there was a problem with the API call or the `texts` slice is empty.
    pub fn translate_opt_context<C, S>(
        &self,
        options: &Options,
        context: &Option<C>,
        texts: &[S],
    ) -> Result<Vec<Translation>, Error>
    where
        C: AsRef<str>,
        S: AsRef<str>,
    {
        check_translate(options, texts)?;

        Self::call_api(
            self.client
                .post(self.api_key.get_url_translate())
                .header("Authorization", &self.api_key)
                .json(&options.generate_json(context.as_ref(), texts)),
        )
        .map(|r: Translations| r.translations)
    }

    /// Call the usage API.
    ///
    /// # Errors
    ///
    /// Will return `Err` when there was a problem with the API call.
    pub fn usage(&self) -> Result<Usage, Error> {
        Self::call_api(
            self.client
                .get(self.api_key.get_url_usage())
                .header("Authorization", &self.api_key),
        )
    }

    fn call_api<T: DeserializeOwned>(request_builder: RequestBuilder) -> Result<T, Error> {
        let response = request_builder.send()?;

        match response.status() {
            StatusCode::OK => Ok(response.json()?),
            StatusCode::PAYLOAD_TOO_LARGE => Err(Error::RequestSizeExceedsTheLimit),
            StatusCode::TOO_MANY_REQUESTS => Err(Error::TooManyRequests),
            status if status.as_u16() == 456 => Err(Error::QuotaExceeded),
            status => Err(Error::UnknownStatus {
                status,
                text: response.text().ok(),
            }),
        }
    }
}
