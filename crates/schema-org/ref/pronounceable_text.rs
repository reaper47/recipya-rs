use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::helpers::one_or_many;
use crate::field::*;
///<https://schema.org/PronounceableText>
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PronounceableText {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/speechToTextMarkup>
    #[serde(rename = "speechToTextMarkup")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub speech_to_text_markup: Vec<String>,
    ///<https://schema.org/textValue>
    #[serde(rename = "textValue")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub text_value: Vec<String>,
    ///<https://schema.org/phoneticText>
    #[serde(rename = "phoneticText")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub phonetic_text: Vec<String>,
    ///<https://schema.org/inLanguage>
    #[serde(rename = "inLanguage")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub in_language: Vec<PronounceableTextInLanguageFieldEnum>,
}
