use std::{collections::HashMap, fmt::Formatter, str::FromStr, vec::Vec};

use serde::{
    de,
    de::{Error, MapAccess, SeqAccess},
    Deserialize, Deserializer,
};
use url::Url;

use crate::schema::AtType;

#[derive(Debug, PartialEq)]
pub enum Action {
    Item(ActionType),
    Items(Vec<ActionType>),
}

/// An action performed by a direct agent and indirect participants upon a direct object. Optionally
/// happens at a location with the help of an inanimate instrument. The execution of the action
/// may produce a result. Specific action sub-type documentation specifies the exact expectation
/// of each argument/role.
#[derive(Debug, Default, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ActionType {
    #[serde(rename = "@type")]
    pub at_type: Option<String>,
    pub name: Option<String>,
    pub target: Vec<String>,
}

impl<'de> Deserialize<'de> for Action {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use crate::schema::common::Action::*;

        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Action;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("an Action or an array of Actions")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut vec: Vec<ActionType> = Vec::new();
                while let Some(action) = seq.next_element::<ActionType>()? {
                    vec.push(action);
                }
                Ok(Items(vec))
            }

            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let action = ActionType::deserialize(de::value::MapAccessDeserializer::new(map))?;
                Ok(Item(action))
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

/// The average rating based on multiple ratings or reviews.
#[derive(Debug, Default, Deserialize, PartialEq)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct AggregateRating {
    #[serde(rename = "@type", default = "set_aggregate_rating_type")]
    pub at_type: AtType,

    /// The rating for the content.
    //
    // Usage guidelines:
    // - Use values from 0123456789 (Unicode 'DIGIT ZERO' (U+0030) to 'DIGIT NINE' (U+0039)) rather
    //   than superficially similar Unicode symbols.
    // - Use '.' (Unicode 'FULL STOP' (U+002E)) rather than ',' to indicate a decimal point. Avoid
    //   using these symbols as a readability separator.
    pub rating_value: Option<NumberOrText>,

    /// The highest value allowed in this rating system.
    pub best_rating: Option<i64>,

    /// The count of total number of ratings.
    pub rating_count: Option<i64>,

    /// The count of total number of reviews.
    #[serde(default, deserialize_with = "deserialize_int64")]
    pub review_count: Option<i64>,
}

fn set_aggregate_rating_type() -> AtType {
    AtType::AggregateRating
}

fn deserialize_int64<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
where
    D: Deserializer<'de>,
{
    struct Visitor;

    impl<'de> serde::de::Visitor<'de> for Visitor {
        type Value = Option<i64>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an integer or a string that can be parsed into an integer")
        }

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Some(value))
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            match v.parse::<i64>() {
                Ok(num) => Ok(Some(num)),
                Err(_) => Ok(None),
            }
        }

        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            match v.parse::<i64>() {
                Ok(num) => Ok(Some(num)),
                Err(_) => Ok(None),
            }
        }
    }

    deserializer.deserialize_any(Visitor)
}

#[derive(Debug, PartialEq)]
pub enum AudioObjectOrClipOrMusicRecording {
    AudioObject(AudioObjectType),
    Clip(ClipType),
    MusicRecording(MusicRecordingType),
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct AudioObjectType {}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ClipType {}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct MusicRecordingType {}

impl<'de> Deserialize<'de> for AudioObjectOrClipOrMusicRecording {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use crate::schema::common::AudioObjectOrClipOrMusicRecording::*;

        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = AudioObjectOrClipOrMusicRecording;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("an AudioObject, CLip or MusicRecording object")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                if let Ok(a) =
                    AudioObjectType::deserialize(de::value::MapAccessDeserializer::new(&mut map))
                {
                    return Ok(AudioObject(a));
                }

                if let Ok(clip) =
                    ClipType::deserialize(de::value::MapAccessDeserializer::new(&mut map))
                {
                    return Ok(Clip(clip));
                }

                if let Ok(music) =
                    MusicRecordingType::deserialize(de::value::MapAccessDeserializer::new(&mut map))
                {
                    return Ok(MusicRecording(music));
                }

                Err(Error::invalid_value(de::Unexpected::Map, &self))
            }
        }

        deserializer.deserialize_map(Visitor)
    }
}

/// A comment on an item - for example, a comment on a blog post. The comment's content is expressed
/// via the text property, and its topic via about, properties shared with all CreativeWorks.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CommentType {}

#[derive(Debug, PartialEq)]
pub enum ClipOrVideoObject {
    Clip(ClipType),
    VideoObject(VideoObjectType),
}

impl<'de> Deserialize<'de> for ClipOrVideoObject {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use crate::schema::common::ClipOrVideoObject::*;

        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = ClipOrVideoObject;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("a Clip or VideoObject object")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                if let Ok(video) =
                    VideoObjectType::deserialize(de::value::MapAccessDeserializer::new(&mut map))
                {
                    return Ok(VideoObject(video));
                }

                if let Ok(clip) =
                    ClipType::deserialize(de::value::MapAccessDeserializer::new(&mut map))
                {
                    return Ok(Clip(clip));
                }

                Err(Error::invalid_value(de::Unexpected::Map, &self))
            }
        }

        deserializer.deserialize_map(Visitor)
    }
}

/// A country.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CountryType {}

#[derive(Debug, PartialEq)]
pub enum CreativeWorkOrHowToSectionOrHowToStepOrText {
    CreativeWork(CreativeWorkType),
    HowToSection(HowToSectionType),
    HowToStep(HowTo),
    Text(String),
}

impl<'de> Deserialize<'de> for CreativeWorkOrHowToSectionOrHowToStepOrText {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use crate::schema::common::CreativeWorkOrHowToSectionOrHowToStepOrText::*;

        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = CreativeWorkOrHowToSectionOrHowToStepOrText;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("a CreativeWork, HowToSection or HowToStep object or text")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Text(v.to_owned()))
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Text(v))
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                if let Ok(c) =
                    CreativeWorkType::deserialize(de::value::MapAccessDeserializer::new(&mut map))
                {
                    return Ok(CreativeWork(c));
                }

                if let Ok(section) =
                    HowToSectionType::deserialize(de::value::MapAccessDeserializer::new(&mut map))
                {
                    return Ok(HowToSection(section));
                }

                if let Ok(step) =
                    HowTo::deserialize(de::value::MapAccessDeserializer::new(&mut map))
                {
                    return Ok(HowToStep(step));
                }

                Err(Error::invalid_value(de::Unexpected::Map, &self))
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

#[derive(Debug, PartialEq)]
pub enum CreativeWorkOrItemListOrText {
    CreativeWork(CreativeWorkType),
    ItemList(Vec<HowTo>),
    Text(String),
}

impl Default for CreativeWorkOrItemListOrText {
    fn default() -> Self {
        Self::ItemList(Vec::new())
    }
}

impl<'de> Deserialize<'de> for CreativeWorkOrItemListOrText {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use crate::schema::common::CreativeWorkOrItemListOrText::*;

        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = CreativeWorkOrItemListOrText;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("a CreativeWork object, an array of HowTo objects or text.")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Text(v.to_owned()))
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Text(v))
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut vec: Vec<HowTo> = Vec::new();
                while let Some(mut object) = seq.next_element::<HowTo>()? {
                    if let Some(name) = object.name {
                        object.name = Some(deserialize_trim(name));
                    }
                    object.text = deserialize_trim(object.text);
                    vec.push(object);
                }
                Ok(ItemList(vec))
            }

            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let c = CreativeWorkType::deserialize(de::value::MapAccessDeserializer::new(map))?;
                Ok(CreativeWork(c))
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

fn deserialize_trim<'a>(mut s: String) -> String {
    let replace_map: HashMap<&str, &str> = HashMap::from_iter([("&nbsp;", " ")]);
    for (old, new) in replace_map.iter() {
        s = s.replace(old, new)
    }

    s.trim().to_string()
}

#[derive(Debug, PartialEq)]
pub enum CreativeWorkOrText {
    CreativeWork(CreativeWorkType),
    Text(String),
}

impl<'de> Deserialize<'de> for CreativeWorkOrText {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use crate::schema::common::CreativeWorkOrText::*;

        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = CreativeWorkOrText;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("a CreativeWork object or text")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Text(v.to_owned()))
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Text(v))
            }

            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let c = CreativeWorkType::deserialize(de::value::MapAccessDeserializer::new(map))?;
                Ok(CreativeWork(c))
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

#[derive(Debug, PartialEq)]
pub enum CreativeWorkOrUrl {
    Url(Url),
    CreativeWork(CreativeWorkType),
}

impl<'de> Deserialize<'de> for CreativeWorkOrUrl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use crate::schema::common::CreativeWorkOrUrl::*;

        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = CreativeWorkOrUrl;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("a URL or a CreativeWork struct")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                match url::Url::parse(v) {
                    Ok(url) => Ok(Url(url)),
                    Err(_) => Err(Error::invalid_value(de::Unexpected::Str(v), &self)),
                }
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: Error,
            {
                match url::Url::parse(&v) {
                    Ok(url) => Ok(Url(url)),
                    Err(_) => Err(Error::invalid_value(de::Unexpected::Str(&v), &self)),
                }
            }

            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let c = CreativeWorkType::deserialize(de::value::MapAccessDeserializer::new(map))?;
                Ok(CreativeWork(c))
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

/// The most generic kind of creative work, including books, movies, photographs, software programs, etc.
#[derive(Debug, Default, Deserialize, PartialEq)]
pub struct CreativeWorkType {
    #[serde(rename = "@id")]
    pub at_id: Option<Url>,

    #[serde(rename = "@type", default = "set_creative_work_type")]
    pub at_type: AtType,

    /// A description of the item.
    pub description: Option<TextOrTextObject>,

    /// An image of the item. This can be a URL or a fully described ImageObject.
    pub image: Option<ImageObjectOrUrl>,

    /// The name of the item.
    #[serde(alias = "Name")]
    pub name: Option<String>,
}

fn set_creative_work_type() -> AtType {
    AtType::CreativeWork
}

#[derive(Debug, PartialEq)]
pub enum DateOrDateTime {
    DateTime(iso8601::DateTime),
    Date(iso8601::Date),
}

impl<'de> Deserialize<'de> for DateOrDateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use crate::schema::common::DateOrDateTime::*;

        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = DateOrDateTime;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("a string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                if let Ok(dt) = iso8601::DateTime::from_str(&v) {
                    return Ok(DateTime(dt));
                }

                if let Ok(date) = iso8601::Date::from_str(&v) {
                    return Ok(Date(date));
                }

                Err(Error::invalid_value(de::Unexpected::Str(&v), &self))
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: Error,
            {
                if let Ok(dt) = iso8601::DateTime::from_str(&v) {
                    return Ok(DateTime(dt));
                }

                if let Ok(date) = iso8601::Date::from_str(&v) {
                    return Ok(Date(date));
                }

                Err(Error::invalid_value(de::Unexpected::Str(&v), &self))
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}

#[derive(Debug, PartialEq)]
pub enum DefinedTermOrTextOrUrl {
    DefinedTerm(DefinedTermType),
    Text(String),
    Url(Url),
}

/// A word, name, acronym, phrase, etc. with a formal definition. Often used in the context of
/// category or subject classification, glossaries or dictionaries, product or creative work types,
/// etc. Use the name property for the term being defined, use termCode if the term has an
/// alpha-numeric code allocated, use description to provide the definition of the term.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct DefinedTermType {}

impl<'de> Deserialize<'de> for DefinedTermOrTextOrUrl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use crate::schema::common::DefinedTermOrTextOrUrl::*;

        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = DefinedTermOrTextOrUrl;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("a DefinedTerm object, text or url")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                if let Ok(url) = url::Url::parse(v) {
                    return Ok(Url(url));
                }
                Ok(Text(v.to_owned()))
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: Error,
            {
                if let Ok(url) = url::Url::parse(&v) {
                    return Ok(Url(url));
                }
                Ok(Text(v))
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut vec: Vec<String> = Vec::new();
                while let Some(v) = seq.next_element()? {
                    vec.push(v)
                }
                Ok(Text(vec.join(",")))
            }

            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let d = DefinedTermType::deserialize(de::value::MapAccessDeserializer::new(map))?;
                Ok(DefinedTerm(d))
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

#[derive(Debug, PartialEq)]
pub enum DistanceOrQuantitativeValue {
    Distance(DistanceType),
    QuantitativeValue(QuantitativeValueType),
}

/// Properties that take Distances as values are of the form '<Number> <Length unit of measure>'. E.g., '7 ft'.
#[derive(Debug, Default, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct DistanceType {
    pub value: String,
}

impl<'de> Deserialize<'de> for DistanceOrQuantitativeValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use crate::schema::common::DistanceOrQuantitativeValue::*;

        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = DistanceOrQuantitativeValue;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("a distance or quantitative value")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Distance(DistanceType {
                    value: v.to_owned(),
                }))
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Distance(DistanceType { value: v }))
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(QuantitativeValue(QuantitativeValueType { value: v }))
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(QuantitativeValue(QuantitativeValueType { value: v as i64 }))
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

/// A sub-grouping of steps in the instructions for how to achieve a result (e.g. steps for
/// making a pie crust within a pie recipe).
#[derive(Debug, Default, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct HowToSectionType {}

/// Instructions that explain how to achieve a result by performing a sequence of steps.
#[derive(Debug, Default, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct HowTo {
    #[serde(rename = "@type")]
    pub at_type: AtType,
    pub name: Option<String>,
    pub text: String,
    pub url: Option<Url>,
    pub image: Option<ImageObjectOrUrl>,
}

#[derive(Debug, PartialEq)]
pub enum HowToSupplyOrText {
    HowToSupply(HowToSupplyType),
    Text(String),
}

/// A sub-property of instrument. A supply consumed when performing instructions or a direction.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct HowToSupplyType {}

impl<'de> Deserialize<'de> for HowToSupplyOrText {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use crate::schema::common::HowToSupplyOrText::*;

        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = HowToSupplyOrText;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("a HowToSupply object or text")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Text(v.to_owned()))
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Text(v))
            }

            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let how = HowToSupplyType::deserialize(de::value::MapAccessDeserializer::new(map))?;
                Ok(HowToSupply(how))
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

#[derive(Debug, PartialEq)]
pub enum HowToToolOrText {
    HowToTool(HowToToolType),
    Text(String),
}

/// A tool used (but not consumed) when performing instructions for how to achieve a result.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct HowToToolType {}

impl<'de> Deserialize<'de> for HowToToolOrText {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use crate::schema::common::HowToToolOrText::*;

        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = HowToToolOrText;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("a HowToSupply object or text")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Text(v.to_owned()))
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Text(v))
            }

            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let how = HowToToolType::deserialize(de::value::MapAccessDeserializer::new(map))?;
                Ok(HowToTool(how))
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

#[derive(Debug, PartialEq)]
pub enum ImageObjectOrUrl {
    Url(Url),
    ImageObject(ImageObjectType),
}

/// An image file.
#[derive(Debug, Default, Deserialize, PartialEq)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ImageObjectType {
    #[serde(rename = "@type", default = "set_image_object")]
    pub at_type: AtType,

    #[serde(rename = "@id")]
    pub at_id: Option<String>,

    /// The caption for this object. For downloadable machine formats (closed caption, subtitles etc.)
    /// use MediaObject and indicate the encodingFormat.
    pub caption: Option<MediaObjectOrText>,

    /// Actual bytes of the media object, for example the image file or video file.
    pub content_url: Option<Url>,

    /// The height of the item.
    pub height: Option<DistanceOrQuantitativeValue>,

    /// The language of the content or performance or used in an action. Please use one of the
    /// language codes from the IETF BCP 47 standard. See also availableLanguage. Supersedes language.
    pub in_language: Option<LanguageOrText>,

    /// URL of the item.
    pub url: Option<Url>,

    /// The width of the item.
    pub width: Option<DistanceOrQuantitativeValue>,
}

fn set_image_object() -> AtType {
    AtType::ImageObject
}

impl<'de> Deserialize<'de> for ImageObjectOrUrl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use crate::schema::common::ImageObjectOrUrl::*;

        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = ImageObjectOrUrl;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("an ImageObject object or URL")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                if v.is_empty() {
                    return Ok(ImageObject(ImageObjectType::default()));
                }

                if let Ok(url) = url::Url::parse(v) {
                    return Ok(Url(url));
                }
                Err(Error::invalid_value(de::Unexpected::Str(v), &self))
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: Error,
            {
                if v.is_empty() {
                    return Ok(ImageObject(ImageObjectType::default()));
                }

                if let Ok(url) = url::Url::parse(&v) {
                    return Ok(Url(url));
                }
                Err(Error::invalid_value(de::Unexpected::Str(&v), &self))
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut vec: Vec<String> = Vec::new();
                while let Some(s) = seq.next_element()? {
                    vec.push(s)
                }

                let v = match vec.pop() {
                    None => return Err(Error::custom("sequence is empty")),
                    Some(v) => v,
                };

                let url = url::Url::parse(&v).map_err(|ex| Error::custom(ex.to_string()))?;
                Ok(Url(url))
            }

            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let img = ImageObjectType::deserialize(de::value::MapAccessDeserializer::new(map))?;
                Ok(ImageObject(img))
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

#[derive(Debug, PartialEq)]
pub enum IntegerOrText {
    Integer(i64),
    Text(String),
}

impl<'de> Deserialize<'de> for IntegerOrText {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use crate::schema::common::IntegerOrText::*;

        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = IntegerOrText;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("an integer or text")
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Integer(v))
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Integer(v as i64))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Text(v.to_owned()))
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Text(v))
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

#[derive(Debug, PartialEq)]
pub enum ListItemOrTextOrThing {
    ListItem(Vec<ListItemType>),
    Text(String),
    Thing(ThingType),
}

#[derive(Debug, Default, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ListItemType {
    #[serde(rename = "@type", default = "set_list_item_type")]
    pub at_type: AtType,

    /// An entity represented by an entry in a list or data feed (e.g. an 'artist' in a list of 'artists').
    pub item: Option<ThingOrUrl>,

    /// The name of the item.
    pub name: Option<String>,

    /// The position of an item in a series or sequence of items.
    pub position: Option<IntegerOrText>,
}

fn set_list_item_type() -> AtType {
    AtType::ListItem
}

impl<'de> Deserialize<'de> for ListItemOrTextOrThing {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use crate::schema::common::ListItemOrTextOrThing::*;

        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = ListItemOrTextOrThing;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("a ListItem object, text or Thing object")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Text(v.to_owned()))
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Text(v))
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut vec: Vec<ListItemType> = Vec::new();
                while let Some(item) = seq.next_element::<ListItemType>()? {
                    vec.push(item);
                }
                Ok(ListItem(vec))
            }

            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let thing = ThingType::deserialize(de::value::MapAccessDeserializer::new(map))?;
                Ok(Thing(thing))
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

#[derive(Debug, PartialEq)]
pub enum LanguageOrText {
    Language(LanguageType),
    Text(String),
}

/// Natural languages such as Spanish, Tamil, Hindi, English, etc. Formal language code tags
/// expressed in BCP 47 can be used via the alternateName property. The Language type previously
/// also covered programming languages such as Scheme and Lisp, which are now best represented
/// using ComputerLanguage.
#[derive(Debug, Default, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct LanguageType {}

impl<'de> Deserialize<'de> for LanguageOrText {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use crate::schema::common::LanguageOrText::*;

        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = LanguageOrText;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("a Language object or text")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Text(v.to_owned()))
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Text(v))
            }

            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let lang = LanguageType::deserialize(de::value::MapAccessDeserializer::new(map))?;
                Ok(Language(lang))
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

#[derive(Debug, PartialEq)]
pub enum MediaObjectOrText {
    MediaObject(MediaObjectType),
    Text(String),
}

/// A media object, such as an image, video, audio, or text object embedded in a web page or a
/// downloadable dataset i.e. DataDownload. Note that a creative work may have many media objects
/// associated with it on the same web page. For example, a page about a single song (MusicRecording)
/// may have a music video (VideoObject), and a high and low bandwidth audio stream (2 AudioObject's).
#[derive(Debug, Default, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct MediaObjectType {}

impl<'de> Deserialize<'de> for MediaObjectOrText {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use crate::schema::common::MediaObjectOrText::*;

        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = MediaObjectOrText;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("a MediaObject object or text")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Text(v.to_owned()))
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Text(v))
            }

            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let media =
                    MediaObjectType::deserialize(de::value::MapAccessDeserializer::new(map))?;
                Ok(MediaObject(media))
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

#[derive(Debug, PartialEq)]
pub enum MonetaryAmountOrText {
    MonetaryAmount(MonetaryAmountType),
    Text(String),
}

/// A monetary value or range. This type can be used to describe an amount of money such as
/// $50 USD, or a range as in describing a bank account being suitable for a balance between
/// £1,000 and £1,000,000 GBP, or the value of a salary, etc. It is recommended to use PriceSpecification
/// Types to describe the price of an Offer, Invoice, etc.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct MonetaryAmountType {}

impl<'de> Deserialize<'de> for MonetaryAmountOrText {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use crate::schema::common::MonetaryAmountOrText::*;

        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = MonetaryAmountOrText;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("a MonetaryAmount object or a string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Text(v.to_owned()))
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Text(v))
            }

            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let m =
                    MonetaryAmountType::deserialize(de::value::MapAccessDeserializer::new(map))?;
                Ok(MonetaryAmount(m))
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

#[derive(Debug, PartialEq)]
pub enum NumberOrText {
    Number(i64),
    Text(String),
}

impl<'de> Deserialize<'de> for NumberOrText {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use crate::schema::common::NumberOrText::*;

        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = NumberOrText;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("a number or a string")
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Text(v))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Text(v.to_owned()))
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Number(v))
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Number(v as i64))
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

#[derive(Debug, PartialEq)]
pub enum OrganizationOrPerson {
    Organization(OrganizationType),
    Person(PersonType),
}

/// An organization such as a school, NGO, corporation, club, etc.
#[derive(Debug, Default, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct OrganizationType {
    #[serde(rename = "@id")]
    pub at_id: Option<Url>,
    #[serde(rename = "@type")]
    pub at_type: AtType,

    /// The name of the item.
    pub name: Option<String>,

    /// An associated logo.
    pub logo: Option<ImageObjectOrUrl>,

    /// URL of the item.
    pub url: Option<Url>,
}

/// A person (alive, dead, undead, or fictional).
#[derive(Debug, Default, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct PersonType {
    #[serde(rename = "@id")]
    pub at_id: Option<Url>,
    #[serde(rename = "@type")]
    pub at_type: Option<AtType>,
    pub name: Option<String>,
    pub url: Option<Url>,
}

impl<'de> Deserialize<'de> for OrganizationOrPerson {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use crate::schema::common::OrganizationOrPerson::*;

        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = OrganizationOrPerson;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("an Organization or Person object")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                if let Ok(org) =
                    OrganizationType::deserialize(de::value::MapAccessDeserializer::new(&mut map))
                {
                    return Ok(Organization(org));
                }

                if let Ok(person) =
                    PersonType::deserialize(de::value::MapAccessDeserializer::new(map))
                {
                    return Ok(Person(person));
                }

                Err(Error::invalid_value(de::Unexpected::Map, &self))
            }
        }

        deserializer.deserialize_map(Visitor)
    }
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct PlaceType {}

#[derive(Debug, PartialEq)]
pub enum PropertyValueOrTextOrUrl {
    PropertyValue(PropertyValueType),
    Text(String),
    Url(Url),
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct PropertyValueType {}

impl<'de> Deserialize<'de> for PropertyValueOrTextOrUrl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use crate::schema::common::PropertyValueOrTextOrUrl::*;

        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = PropertyValueOrTextOrUrl;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("a PropertyValue object, text or a URL")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                if let Ok(url) = url::Url::parse(&v) {
                    return Ok(Url(url));
                }
                Ok(Text(v.to_owned()))
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: Error,
            {
                if let Ok(url) = url::Url::parse(&v) {
                    return Ok(Url(url));
                }
                Ok(Text(v))
            }

            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let prop =
                    PropertyValueType::deserialize(de::value::MapAccessDeserializer::new(map))?;
                Ok(PropertyValue(prop))
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

#[derive(Debug, PartialEq)]
pub enum QuantitativeValueOrText {
    QuantitativeValue(QuantitativeValueType),
    Text(String),
}

impl Default for QuantitativeValueOrText {
    fn default() -> Self {
        QuantitativeValueOrText::QuantitativeValue(QuantitativeValueType { value: 1 })
    }
}

/// A point value or interval for product characteristics and other purposes.
#[derive(Debug, PartialEq)]
pub struct QuantitativeValueType {
    pub value: i64,
}

impl<'de> Deserialize<'de> for QuantitativeValueOrText {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use crate::schema::common::QuantitativeValueOrText::*;

        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = QuantitativeValueOrText;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("a quantitative value or text")
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(QuantitativeValue(QuantitativeValueType { value: v }))
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(QuantitativeValue(QuantitativeValueType { value: v as i64 }))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Text(v.to_owned()))
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Text(v))
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut vec: Vec<String> = Vec::new();
                while let Some(item) = seq.next_element::<String>()? {
                    vec.push(item);
                }

                let v = vec
                    .get(0)
                    .ok_or_else(|| Error::custom("sequence is empty"))?;
                let v: i64 = v.parse().map_err(|err| Error::custom(err))?;
                Ok(QuantitativeValue(QuantitativeValueType { value: v }))
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

#[derive(Debug, PartialEq)]
pub enum RatingOrText {
    Rating(RatingType),
    Text(String),
}

/// A rating is an evaluation on a numeric scale, such as 1 to 5 stars.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct RatingType {}

impl<'de> Deserialize<'de> for RatingOrText {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use crate::schema::common::RatingOrText::*;

        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = RatingOrText;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("a RatingOrText object or text")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Text(v.to_owned()))
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Text(v))
            }

            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let rating = RatingType::deserialize(de::value::MapAccessDeserializer::new(map))?;
                Ok(Rating(rating))
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct ReviewType {
    #[serde(rename = "@type")]
    pub at_type: AtType,
    #[serde(rename = "reviewRating")]
    pub review_rating: ReviewRating,
    pub author: OrganizationOrPerson,
    #[serde(rename = "datePublished")]
    pub date_published: DateOrDateTime,
    #[serde(rename = "reviewBody")]
    pub review_body: Option<String>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct ReviewRating {
    #[serde(rename = "@type")]
    pub at_type: AtType,
    #[serde(rename = "ratingValue")]
    pub rating_value: String,
}

#[derive(Debug, PartialEq)]
pub enum TextOrTextObject {
    Text(String),
    TextObject(TextObjectType),
}

/// A text file. The text can be unformatted or contain markup, html, etc.
#[derive(Debug, Default, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct TextObjectType {}

impl<'de> Deserialize<'de> for TextOrTextObject {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use crate::schema::common::TextOrTextObject::*;

        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = TextOrTextObject;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("text or a TextObject object")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Text(v.to_owned()))
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Text(v))
            }

            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let t = TextObjectType::deserialize(de::value::MapAccessDeserializer::new(map))?;
                Ok(TextObject(t))
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

#[derive(Debug, PartialEq)]
pub enum ThingOrUrl {
    Thing(ThingType),
    Url(Url),
}

/// The most generic type of item.
#[derive(Debug, Default, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ThingType {}

impl<'de> Deserialize<'de> for ThingOrUrl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use crate::schema::common::ThingOrUrl::*;

        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = ThingOrUrl;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("a Thing object or URL")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                match url::Url::parse(v) {
                    Ok(url) => Ok(Url(url)),
                    Err(_) => Err(Error::invalid_value(de::Unexpected::Str(v), &self)),
                }
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: Error,
            {
                match url::Url::parse(&v) {
                    Ok(url) => Ok(Url(url)),
                    Err(_) => Err(Error::invalid_value(de::Unexpected::Str(&v), &self)),
                }
            }

            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let t = ThingType::deserialize(de::value::MapAccessDeserializer::new(map))?;
                Ok(Thing(t))
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

#[derive(Debug, PartialEq)]
pub enum Video {
    VideoObject(VideoObjectType),
}

impl<'de> Deserialize<'de> for Video {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use crate::schema::common::Video::*;

        struct Visitor;

        impl<'d> de::Visitor<'d> for Visitor {
            type Value = Video;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("a VideoObject object")
            }

            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'d>,
            {
                let v = VideoObjectType::deserialize(de::value::MapAccessDeserializer::new(map))?;
                Ok(VideoObject(v))
            }
        }

        deserializer.deserialize_map(Visitor)
    }
}

/// A video file.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct VideoObjectType {
    #[serde(rename = "@type")]
    pub at_type: AtType,

    /// Actual bytes of the media object, for example the image file or video file.
    pub content_url: Url,

    /// A description of the item.
    pub description: String,

    /// Approximate or typical time it usually takes to work with or through the content of this
    /// work for the typical or target audience.
    #[serde(deserialize_with = "deserialize_duration")]
    pub duration: Option<iso8601::Duration>,

    /// A URL pointing to a player for a specific video. In general, this is the information in
    /// the src element of an embed tag and should not be the same as the content of the loc tag.
    pub embed_url: Url,

    /// The name of the item.
    pub name: String,

    /// A thumbnail image relevant to the Thing.
    pub thumbnail_url: Vec<Url>,

    /// Date (including time if available) when this media object was uploaded to this site.
    pub upload_date: Option<iso8601::DateTime>,
}

fn deserialize_duration<'de, D>(deserializer: D) -> Result<Option<iso8601::Duration>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(s) => {
            if s.is_empty() {
                return Ok(None);
            }
            let (_, dur) =
                iso8601::parsers::parse_duration(&s.as_bytes()).map_err(Error::custom)?;
            Ok(Some(dur))
        }
        None => Ok(None),
    }
}
