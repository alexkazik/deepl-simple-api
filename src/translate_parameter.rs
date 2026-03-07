//! Parameter for the translate call.

pub use crate::parameter::{IntoParam, StaticValue, ToStaticParam, Value};
use paste::paste;
use std::borrow::Cow;

// helpers

macro_rules! param_no {
    ($name:ident, $param:expr) => {
        paste! {
            #[doc = concat!("Remove the parameter ", stringify!($name), ".")]
            #[derive(Copy, Clone, Eq, PartialEq)]
            pub struct [<No $name>];

            impl ToStaticParam for [<No $name>] {
                fn to_static_param(&self) -> (&'static str, Option<StaticValue<'_>>) {
                    (
                        $param, None
                    )
                }
            }
        }
    };
}

macro_rules! param_bool {
    ($name:ident, $param:expr) => {
        paste! {
            #[doc = concat!("Parameter ", stringify!($name), ".")]
            #[derive(Copy, Clone, Eq, PartialEq)]
            pub struct $name(pub bool);

            impl ToStaticParam for $name {
                fn to_static_param(&self) -> (&'static str, Option<StaticValue<'_>>) {
                    ($param, Some(StaticValue::Bool(self.0)))
                }
            }

            param_no!($name, $param);
        }
    };
}

macro_rules! param_str {
    ($name:ident, $param:expr, without_no) => {
        paste! {
            #[doc = concat!("Parameter ", stringify!($name), ", static.")]
            #[derive(Copy, Clone, Eq, PartialEq)]
            pub struct $name(pub &'static str);

            impl ToStaticParam for $name {
                fn to_static_param(&self) -> (&'static str, Option<StaticValue<'_>>) {
                    ($param, Some(StaticValue::Str(self.0)))
                }
            }

            #[doc = concat!("Parameter ", stringify!($name), ".")]
            #[derive(Clone, Eq, PartialEq)]
            pub struct [<$name Dyn>]<T: Into<Cow<'static, str>>>(pub T);

            impl<T: Into<Cow<'static, str>>> IntoParam for [<$name Dyn>]<T> {
                fn into_param(self) -> (&'static str, Option<Value>) {
                    ($param, Some(Value::Str(self.0.into())))
                }
            }
        }
    };
    ($name:ident, $param:expr) => {
        param_str!($name, $param, without_no);
        param_no!($name, $param);
    };
}

macro_rules! param_list {
    ($name:ident, $param:expr) => {
        paste! {
            #[doc = concat!("Parameter ", stringify!($name), ", static.")]
            #[derive(Copy, Clone, Eq, PartialEq)]
            pub struct $name<'a>(pub &'a [&'static str]);

            impl<'a> ToStaticParam for $name<'a> {
                fn to_static_param(&self) -> (&'static str, Option<StaticValue<'a>>) {
                    ($param, Some(StaticValue::List(self.0)))
                }
            }

            #[doc = concat!("Parameter ", stringify!($name), ".")]
            #[derive(Clone, Eq, PartialEq)]
            pub struct [<$name Dyn>]<T: Into<Cow<'static, str>>>(pub Vec<T>);

            impl<T: Into<Cow<'static, str>>> IntoParam for [<$name Dyn>]<T> {
                fn into_param(self) -> (&'static str, Option<Value>) {
                    (
                        $param,
                        Some(Value::List(self.0.into_iter().map(Into::into).collect())),
                    )
                }
            }
        }

        param_no!($name, $param);
    };
}

macro_rules! param_enum {
    ($name:ident, $param:expr, $(($field:ident, $value:expr),)+) => {
        paste! {
            #[doc = concat!("Parameter ", stringify!($name), ".")]
            #[non_exhaustive]
            #[derive(Copy, Clone, Eq, PartialEq)]
            #[expect(missing_docs)]
            pub enum $name {
                $($field,)+
            }

            impl ToStaticParam for $name {
                fn to_static_param(&self) -> (&'static str, Option<StaticValue<'_>>) {
                    (
                        $param,
                        match self {
                            $($name::$field => Some(StaticValue::Str($value)),)+
                        },
                    )
                }
            }
        }
    }
}

// all possible parameters, in order of the DeepL documentation
// (except text and context which are provided in the translation call)

param_str!(TargetLanguage, "target_lang");
param_str!(SourceLanguage, "source_lang");
param_bool!(ShowBilledCharacters, "show_billed_characters");
param_enum!(
    SplitSentences,
    "split_sentences",
    (Off, "0"),
    (All, "1"),
    (NoNewlines, "nonewlines"),
);
param_bool!(PreserveFormatting, "preserve_formatting");
param_enum!(
    Formality,
    "formality",
    (Default, "default"),
    (More, "more"),
    (Less, "less"),
    (PreferMore, "prefer_more"),
    (PreferLess, "prefer_less"),
);
param_enum!(
    ModelType,
    "model_type",
    (QualityOptimized, "quality_optimized"),
    (PreferQualityOptimized, "prefer_quality_optimized"),
    (LatencyOptimized, "latency_optimized"),
);
param_str!(GlossaryId, "glossary_id");
param_str!(StyleId, "style_id");
param_list!(CustomInstructions, "custom_instructions");
param_enum!(TagHandling, "tag_handling", (Xml, "xml"), (Html, "html"),);
param_enum!(
    TagHandlingVersion,
    "tag_handling_version",
    (V1, "v1"),
    (V2, "v2"),
);
param_bool!(OutlineDetection, "outline_detection");
param_bool!(EnableBetaLanguages, "enable_beta_languages");
param_list!(NonSplittingTags, "non_splitting_tags");
param_list!(SplittingTags, "splitting_tags");
param_list!(IgnoreTags, "ignore_tags");
