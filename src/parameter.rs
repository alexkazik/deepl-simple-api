//! Structs and traits for parameters.

use std::borrow::Cow;

// Value

/// Value of a `DeepL` parameter.
pub enum Value {
    /// `boolean`.
    Bool(bool),
    /// `string` or `enum<string>`.
    Str(Cow<'static, str>),
    /// `string[]`.
    List(Vec<Cow<'static, str>>),
}

/// Static value of a `DeepL` parameter.
///
/// The value must be `bool` or based on `&'static str` so that it can be copied.
pub enum StaticValue<'a> {
    /// `boolean`.
    Bool(bool),
    /// `string` or `enum<string>`.
    Str(&'static str),
    /// `string[]`.
    List(&'a [&'static str]),
}

// Param

/// Conversion into an parameter.
pub trait IntoParam {
    /// Create an parameter from a value.
    ///
    /// The return is
    /// * The name of the parameter
    /// * The value of the parameter, or None if the parameter should be omitted
    fn into_param(self) -> (&'static str, Option<Value>);
}

/// Conversion into an parameter.
pub trait ToStaticParam {
    /// Create an parameter from a value.
    ///
    /// The return is
    /// * The name of the parameter
    /// * The static value of the parameter, or None if the parameter should be omitted
    fn to_static_param(&self) -> (&'static str, Option<StaticValue<'_>>);
}

impl<T: ToStaticParam> IntoParam for T {
    fn into_param(self) -> (&'static str, Option<Value>) {
        match self.to_static_param() {
            (k, None) => (k, None),
            (k, Some(StaticValue::Bool(b))) => (k, Some(Value::Bool(b))),
            (k, Some(StaticValue::Str(s))) => (k, Some(Value::Str(Cow::Borrowed(s)))),
            (k, Some(StaticValue::List(l))) => (
                k,
                Some(Value::List(l.iter().map(|e| Cow::Borrowed(*e)).collect())),
            ),
        }
    }
}
