use crate::translate_parameter::{IntoParam, StaticValue, ToStaticParam, Value};
use std::borrow::Cow;
use std::collections::HashMap;

// Options

/// Options of a api call.
#[must_use]
#[derive(Default)]
pub struct Options {
    options: HashMap<&'static str, InnerValue>,
}

#[derive(serde::Serialize)]
#[serde(untagged)]
enum InnerValue {
    Bool(bool),
    Str(Cow<'static, str>),
    List(Vec<Cow<'static, str>>),
}

#[derive(serde::Serialize)]
pub(crate) struct JsonUpload<'a> {
    #[serde(flatten)]
    options: &'a HashMap<&'static str, InnerValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    context: Option<&'a str>,
    text: Vec<&'a str>,
}

impl Options {
    /// ```no_run
    /// # async fn test() -> Result<(), deepl_simple_api::Error> {
    /// # use deepl_simple_api::{Options, translate_parameter::*};
    /// # let deepl: deepl_simple_api::DeepL = todo!();
    /// let mut options = Options::new();
    /// options.params(&[&TargetLanguage("DE"), &SourceLanguage("EN"), &Formality::PreferLess]);
    /// let translated = deepl.translate(&options, &["Hello World"]).await?;
    /// # }
    /// ```
    #[inline]
    pub fn new() -> Self {
        Self {
            options: HashMap::default(),
        }
    }

    /// ```no_run
    /// # async fn test() -> Result<(), deepl_simple_api::Error> {
    /// # use deepl_simple_api::{Options, translate_parameter::*};
    /// # let deepl: deepl_simple_api::DeepL = todo!();
    /// let options = Options::builder()
    ///     .params(&[&TargetLanguage("DE"), &SourceLanguage("EN"), &Formality::PreferLess])
    ///     .build();
    /// let translated = deepl.translate(&options, &["Hello World"]).await?;
    /// # }
    /// ```
    #[inline]
    pub fn builder() -> OptionsBuilder {
        OptionsBuilder {
            inner: Options::new(),
        }
    }

    #[inline]
    pub(crate) fn generate_json<'a, C, S>(
        &'a self,
        context: Option<&'a C>,
        texts: &'a [S],
    ) -> JsonUpload<'a>
    where
        C: AsRef<str>,
        S: AsRef<str>,
    {
        JsonUpload {
            options: &self.options,
            context: context.map(AsRef::as_ref),
            text: texts.iter().map(AsRef::as_ref).collect(),
        }
    }

    /// Adds parameters to the options.
    ///
    /// This is like calling `param` for all entries in the slice.
    ///
    /// Though they all must be static parameters (unlike `param` which allows all parameters)
    pub fn params(&mut self, params: &[&dyn ToStaticParam]) {
        for param in params {
            let (k, v) = param.to_static_param();
            match v {
                None => {
                    self.options.remove(k);
                }
                Some(v) => {
                    self.options.insert(
                        k,
                        match v {
                            StaticValue::Bool(b) => InnerValue::Bool(b),
                            StaticValue::Str(s) => InnerValue::Str(Cow::Borrowed(s)),
                            StaticValue::List(l) => {
                                InnerValue::List(l.iter().map(|l| Cow::Borrowed(*l)).collect())
                            }
                        },
                    );
                }
            }
        }
    }

    /// Adds a single parameter to the options.
    pub fn param<T: IntoParam>(&mut self, param: T) {
        let (k, v) = param.into_param();
        match v {
            None => {
                self.options.remove(k);
            }
            Some(v) => {
                self.options.insert(
                    k,
                    match v {
                        Value::Bool(b) => InnerValue::Bool(b),
                        Value::Str(s) => InnerValue::Str(s),
                        Value::List(l) => InnerValue::List(l),
                    },
                );
            }
        }
    }

    pub(crate) fn contains_key(&self, key: &str) -> bool {
        self.options.contains_key(key)
    }
}

// OptionsBuilder

/// Builder for the `Options`.
#[must_use]
pub struct OptionsBuilder {
    inner: Options,
}

impl OptionsBuilder {
    /// Adds parameters to the options.
    ///
    /// This is like calling `param` for all entries in the slice.
    ///
    /// Though they all must be static parameters (unlike `param` which allows all parameters)
    #[inline]
    pub fn params(mut self, params: &[&dyn ToStaticParam]) -> Self {
        self.inner.params(params);

        self
    }

    /// Adds a single parameter to the options.
    #[inline]
    pub fn param<T: IntoParam>(mut self, param: T) -> Self {
        self.inner.param(param);

        self
    }

    /// Convert into Options.
    #[inline]
    pub fn build(self) -> Options {
        self.inner
    }
}
