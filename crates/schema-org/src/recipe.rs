use std::fmt::Formatter;

use super::components::*;
use super::nutrition::NutritionInformationSchema;
use super::{AtContext, AtType, Diets, components};
use crate::data_type::text::URL;
use reqwest::Url;
use schemars::{JsonSchema, schema_for};
use serde::{Deserialize, Deserializer, de};
use tracing::warn;

/// Enumeration of possible values for the @graph field in JSON-LD used to group
/// multiple related entities in a single document.
#[derive(Debug, Deserialize, PartialEq, JsonSchema)]
#[serde(untagged)]
pub enum GraphObject {
    Recipe(Box<RecipeSchema>),
    Unknown(UnknownType),
}

#[derive(Debug, Deserialize, PartialEq, JsonSchema)]
pub struct UnknownType {}

/// The recipe schema as described in the [schema](https://schema.org/Recipe).
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct RecipeSchema {
    /// The context of the JSON.
    #[serde(rename = "@context", default)]
    pub at_context: AtContext,

    /// The type of schema object.
    #[serde(rename = "@type")]
    pub at_type: Option<AtType>,

    #[serde(rename = "@graph")]
    pub at_graph: Option<Vec<GraphObject>>,

    #[serde(rename = "@id")]
    pub at_id: Option<String>,

    /// The time it takes to actually cook the dish, in ISO 8601 duration format.
    #[serde(alias = "CookTime")]
    #[schemars(with = "String", description = "ISO 8601 duration, e.g. PT20M")]
    pub cook_time: Option<iso8601::Duration>,

    /// The method of cooking, such as Frying, Steaming, etc.
    #[serde(alias = "CookingMethod")]
    pub cooking_method: Option<String>,

    /// Nutrition information about the recipe or menu item.
    pub nutrition: Option<NutritionInformationSchema>,

    /// The category of the recipe—for example, appetizer, entrée, etc.
    #[serde(default)]
    pub recipe_category: RecipeCategory,

    /// The cuisine of the recipe (for example, French or Ethiopian).
    pub recipe_cuisine: Option<RecipeCuisine>,

    /// A step in making the recipe, in the form of a single item (document, video, etc.)
    /// or an ordered list with HowToStep and/or HowToSection items.
    pub recipe_ingredient: Option<Vec<ItemListOrPropertyValueOrText>>,

    /// A step in making the recipe, in the form of a single item (document, video, etc.) or an
    /// ordered list with HowToStep and/or HowToSection items.
    pub recipe_instructions: Option<CreativeWorkOrItemListOrText>,

    /// The quantity produced by the recipe (for example, number of people served, number of servings, etc).
    #[serde(default)]
    pub recipe_yield: QuantitativeValueOrText,

    /// Indicates a dietary restriction or guideline for which this recipe or menu item
    /// is suitable, e.g. diabetic, halal etc.
    #[serde(default)]
    pub suitable_for_diet: Option<Diets>,

    #[serde(flatten)]
    pub how_to: HowTo,

    /// An alias for the item.
    pub alternate_name: Option<String>,

    /// The actual body of the article.
    ///
    /// To ignore. It is not part of the Recipe schema but some websites fuse the
    /// Article and Recipe together.
    pub article_body: Option<String>,

    /// Official rating of a piece of content—for example, 'MPAA PG-13'.
    #[serde(alias = "ContentRating")]
    pub content_rating: Option<RatingOrText>,

    /// Text that can be used to credit person(s) and/or organization(s) associated with a
    /// published Creative Work.
    pub credit_text: Option<String>,

    /// A description of the item.
    pub description: Option<TextOrTextObject>,

    /// An image of the item. This can be a URL or a fully described ImageObject.
    pub image: Option<ImageObjectOrUrl>,

    /// The name of the item.
    #[serde(alias = "Name")]
    pub name: Option<String>,

    /// A description of the item.
    pub description: Option<TextOrTextObject>,

    /// The estimated cost of the supply or supplies consumed when performing instructions.
    pub estimated_cost: Option<MonetaryAmountOrText>,

    /// The identifier property represents any kind of identifier for any kind of Thing, such as
    /// ISBNs, GTIN codes, UUIDs etc. Schema.org provides dedicated properties for representing
    /// many of these, either as textual strings or as URL (URI) links. See background notes for
    /// more details.
    pub identifier: Option<PropertyValueOrTextOrURL>,

    /// An image of the item. This can be a URL or a fully described ImageObject.
    pub image: Option<ImageObjectOrUrl>,

    /// Indicates a page (or other CreativeWork) for which this thing is the main entity being
    /// described. See background notes for details.
    // Inverse property: mainEntity
    pub main_entity_of_page: Option<CreativeWorkOrURL>,

    /// The name of the item.
    pub name: Option<String>,

    /// The length of time it takes to perform instructions or a direction (not including time to
    /// prepare the supplies), in ISO 8601 duration format.
    #[schemars(with = "String", description = "ISO 8601 duration, e.g. PT20M")]
    pub perform_time: Option<iso8601::Duration>,

    /// Indicates a potential Action, which describes an idealized action in which this thing
    /// would play an 'object' role.
    pub potential_action: Option<ActionItemOrItems>,

    /// The length of time it takes to prepare the items to be used in instructions or a
    /// direction, in ISO 8601 duration format.
    #[serde(alias = "PrepTime", deserialize_with = "deserialize_iso8601_duration")]
    #[schemars(with = "String", description = "ISO 8601 duration, e.g. PT20M")]
    pub prep_time: Option<iso8601::Duration>,

    /// A review of the item. Supersedes reviews.
    #[serde(alias = "Review")]
    pub review: Option<Vec<Review>>,

    /// URL of a reference Web page that unambiguously indicates the item's identity. E.g. the URL
    /// of the item's Wikipedia page, Wikidata entry, or official website.
    pub same_as: Option<URL>,

    /// A single step item (as HowToStep, text, document, video, etc.) or a HowToSection.
    /// Supersedes steps.
    pub step: Option<CreativeWorkOrHowToSectionOrHowToStepOrText>,

    /// A sub-property of instrument. A supply consumed when performing instructions or a direction.
    pub supply: Option<HowToSupplyOrText>,

    /// A sub property of instrument. An object used (but not consumed) when performing
    /// instructions or a direction.
    pub tool: Option<Vec<HowToToolOrText>>,

    /// The total time required to perform instructions or a direction (including time to prepare
    /// the supplies), in ISO 8601 duration format.
    #[serde(default, deserialize_with = "deserialize_iso8601_duration")]
    #[schemars(with = "String", description = "ISO 8601 duration, e.g. PT20M")]
    pub total_time: Option<iso8601::Duration>,

    /// The quantity that results by performing instructions. For example, a paper airplane,
    /// 10 personalized candles.
    #[serde(rename = "yield")]
    pub total_yield: Option<QuantitativeValueOrText>,

    /// URL of the item.
    pub url: Option<URL>,
}

impl RecipeSchema {
    /// Extracts image URLs from image objects.
    pub fn extract_image_urls(&self) -> Option<Vec<Url>> {
        self.image
            .clone()
            .map(|obj| match obj {
                ImageObjectOrUrl::Urls(url) => Some(url),
                ImageObjectOrUrl::ImageObject(obj) => obj.url.map(|v| vec![v]),
                ImageObjectOrUrl::ImageObjects(objs) => {
                    objs.into_iter().map(|obj| obj.url).collect()
                }
                ImageObjectOrUrl::Text(_) => None,
            })
            .unwrap_or_default()
    }

    /// Extracts content URLs from video objects, filtering out clip objects.
    pub fn extract_video_content_urls(&self) -> Option<Vec<Url>> {
        self.video.as_ref().map(|videos| {
            videos
                .iter()
                .filter_map(|video| match video {
                    ClipOrVideoObject::Clip(clip) => {
                        warn!("Ignoring clip object in video URL extraction: {clip:?}");
                        None
                    }
                    ClipOrVideoObject::VideoObject(video_obj) => {
                        Some(video_obj.content_url.clone())
                    }
                })
                .collect()
        })
    }

    /// Extracts embedded URLs from video objects, filtering out clip objects.
    pub fn extract_video_embed_urls(&self) -> Option<Vec<Url>> {
        self.video.as_ref().map(|videos| {
            videos
                .iter()
                .filter_map(|video| match video {
                    ClipOrVideoObject::Clip(clip) => {
                        warn!("Ignoring clip object in video URL extraction: {clip:?}");
                        None
                    }
                    ClipOrVideoObject::VideoObject(video_obj) => Some(video_obj.embed_url.clone()),
                })
                .collect()
        })
    }

    /// Generates the schema definition of RecipeSchema.
    pub fn schema() -> String {
        let schema = schema_for!(RecipeSchema);
        serde_json::to_string_pretty(&schema).unwrap_or_default()
    }
}

/// Enumeration of different containers used to store a category.
#[derive(Clone, Debug, PartialEq, JsonSchema)]
pub enum RecipeCategory {
    Text(String),
}

impl Default for RecipeCategory {
    fn default() -> Self {
        RecipeCategory::Text("uncategorized".to_string())
    }
}

impl TryFrom<RecipeCategory> for String {
    type Error = String;

    fn try_from(value: RecipeCategory) -> Result<Self, Self::Error> {
        match value {
            RecipeCategory::Text(text) => Ok(text),
        }
    }
}

impl<'de> Deserialize<'de> for RecipeCategory {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use super::recipe::RecipeCategory::Text;

        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = RecipeCategory;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("string or vector of strings")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Text(v.to_owned()))
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Text(v))
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                let mut vec: Vec<String> = Vec::new();
                while let Some(item) = seq.next_element::<String>()? {
                    vec.push(item);
                }

                let v = vec
                    .first()
                    .ok_or_else(|| de::Error::custom("sequence is empty"))?;

                Ok(Text(v.to_string()))
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

/// Enumeration of different containers used to store a cuisine.
#[derive(Debug, PartialEq, Clone, JsonSchema)]
pub enum RecipeCuisine {
    Text(String),
}

impl From<RecipeCuisine> for String {
    fn from(s: RecipeCuisine) -> String {
        match s {
            RecipeCuisine::Text(text) => text,
        }
    }
}

impl<'de> Deserialize<'de> for RecipeCuisine {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use super::recipe::RecipeCuisine::Text;

        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = RecipeCuisine;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("string or vector of strings")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Text(v.to_owned()))
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Text(v))
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                let mut vec: Vec<String> = Vec::new();
                while let Some(item) = seq.next_element::<String>()? {
                    vec.push(item);
                }

                let v = vec
                    .first()
                    .ok_or_else(|| de::Error::custom("sequence is empty"))?;

                Ok(Text(v.to_string()))
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

fn deserialize_iso8601_duration<'de, D>(
    deserializer: D,
) -> Result<Option<iso8601::Duration>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt: Option<String> = Option::<String>::deserialize(deserializer)?;
    match opt.as_deref().map(str::trim) {
        None | Some("") => Ok(None),
        Some(s) => iso8601::duration(s).map(Some).map_err(de::Error::custom),
    }
}
