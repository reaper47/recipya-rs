use std::fmt::Formatter;

use crate::schema::{
    common::{
        Action, AggregateRating, AudioObjectOrClipOrMusicRecording, ClipOrVideoObject, CommentType,
        CountryType, CreativeWorkOrHowToSectionOrHowToStepOrText, CreativeWorkOrItemListOrText,
        CreativeWorkOrText, CreativeWorkOrUrl, CreativeWorkType, DateOrDateTime,
        DefinedTermOrTextOrUrl, HowToSupplyOrText, HowToToolOrText, ImageObjectOrUrl,
        ImageObjectType, LanguageOrText, MonetaryAmountOrText, OrganizationOrPerson,
        OrganizationType, PlaceType, PropertyValueOrTextOrUrl, QuantitativeValueOrText,
        RatingOrText, ReviewType, TextOrTextObject,
    },
    nutrition::{NutritionInformationSchema, RestrictedDiet},
    AtContext, AtType,
};
use reqwest::Url;
use serde::{de, Deserialize, Deserializer};

#[derive(Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum GraphObject {
    Recipe(RecipeSchema),
    Unknown(UnknownType),
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct UnknownType {}

/// The recipe schema as described in the [schema](https://schema.org/Recipe).
#[derive(Debug, Default, Deserialize, PartialEq)]
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

    /// The overall rating, based on a collection of reviews or ratings, of the item.
    pub aggregate_rating: Option<AggregateRating>,

    /// An alias for the item.
    pub alternate_name: Option<String>,

    /// The actual body of the article.
    ///
    /// To ignore. It is not part of the Recipe schema but some websites fuse the
    /// Article and Recipe together.
    pub article_body: Option<String>,

    /// An embedded audio object.
    pub audio: Option<AudioObjectOrClipOrMusicRecording>,

    /// The author of this content or rating. Please note that author is special in that HTML 5
    /// provides a special mechanism for indicating authorship via the rel tag. That is equivalent
    /// to this and may be used interchangeably.
    pub author: Option<OrganizationType>,

    /// An award won by or for this item. Supersedes awards.
    pub award: Option<String>,

    /// A citation or reference to another creative work, such as another publication, web page,
    /// scholarly article, etc.
    pub citation: Option<CreativeWorkOrText>,

    /// Comments, typically from users.
    pub comment: Option<CommentType>,

    /// The number of comments this CreativeWork (e.g. Article, Question or Answer) has received.
    /// This is most applicable to works published in Web sites with commenting system; additional
    /// comments may exist elsewhere.
    pub comment_count: Option<i64>,

    /// Official rating of a piece of content—for example, 'MPAA PG-13'.
    #[serde(alias = "ContentRating")]
    pub content_rating: Option<RatingOrText>,

    /// A secondary contributor to the CreativeWork or Event.
    pub contributor: Option<OrganizationOrPerson>,

    /// The time it takes to actually cook the dish, in ISO 8601 duration format.
    #[serde(alias = "CookTime")]
    pub cook_time: Option<iso8601::Duration>,

    /// The method of cooking, such as Frying, Steaming, etc.
    #[serde(alias = "CookingMethod")]
    pub cooking_method: Option<String>,

    /// The location depicted or described in the content. For example, the location in a
    /// photograph or painting.
    pub content_location: Option<PlaceType>,

    /// The country of origin of something, including products as well as creative works such as
    /// movie and TV content.
    pub country_of_origin: Option<CountryType>,

    /// Text that can be used to credit person(s) and/or organization(s) associated with a
    /// published Creative Work.
    pub credit_text: Option<String>,

    /// The date on which the CreativeWork was created or the item was added to a DataFeed.
    pub date_created: Option<DateOrDateTime>,

    /// The date on which the CreativeWork was most recently modified or when the item's entry
    /// was modified within a DataFeed.
    pub date_modified: Option<DateOrDateTime>,

    /// Date of first publication or broadcast. For example the date a CreativeWork was broadcast
    /// or a Certification was issued.
    pub date_published: Option<DateOrDateTime>,

    /// A description of the item.
    pub description: Option<TextOrTextObject>,

    /// The estimated cost of the supply or supplies consumed when performing instructions.
    pub estimated_cost: Option<MonetaryAmountOrText>,

    /// Headline of the article.
    pub headline: Option<String>,

    /// The identifier property represents any kind of identifier for any kind of Thing, such as
    /// ISBNs, GTIN codes, UUIDs etc. Schema.org provides dedicated properties for representing
    /// many of these, either as textual strings or as URL (URI) links. See background notes for
    /// more details.
    pub identifier: Option<PropertyValueOrTextOrUrl>,

    /// An image of the item. This can be a URL or a fully described ImageObject.
    pub image: Option<ImageObjectOrUrl>,

    /// The language of the content or performance or used in an action. Please use one of the
    /// language codes from the IETF BCP 47 standard. See also availableLanguage. Supersedes language.
    pub in_language: Option<LanguageOrText>,

    /// A flag to signal that the item, event, or place is accessible for free. Supersedes free.
    #[serde(default, deserialize_with = "deserialize_bool")]
    pub is_accessible_for_free: bool,

    /// Indicates an item or CreativeWork that this item, or CreativeWork (in some sense), is part of.
    // Inverse property: hasPart
    pub is_part_of: Option<CreativeWorkOrUrl>,

    /// Keywords or tags used to describe some item. Multiple textual entries in a keywords list
    /// are typically delimited by commas, or by repeating the property.
    #[serde(alias = "Keywords")]
    pub keywords: Option<DefinedTermOrTextOrUrl>,

    /// The location where the CreativeWork was created, which may not be the same as the location
    /// depicted in the CreativeWork.
    pub location_created: Option<PlaceType>,

    /// Indicates a page (or other CreativeWork) for which this thing is the main entity being
    /// described. See background notes for details.
    // Inverse property: mainEntity
    pub main_entity_of_page: Option<CreativeWorkOrUrl>,

    /// The name of the item.
    pub name: Option<String>,

    /// Nutrition information about the recipe or menu item.
    pub nutrition: Option<NutritionInformationSchema>,

    /// The length of time it takes to perform instructions or a direction (not including time to
    /// prepare the supplies), in ISO 8601 duration format.
    pub perform_time: Option<iso8601::Duration>,

    /// Indicates a potential Action, which describes an idealized action in which this thing
    /// would play an 'object' role.
    pub potential_action: Option<Action>,

    /// The length of time it takes to prepare the items to be used in instructions or a
    /// direction, in ISO 8601 duration format.
    #[serde(alias = "PrepTime")]
    pub prep_time: Option<iso8601::Duration>,

    /// The publisher of the creative work.
    pub publisher: Option<OrganizationOrPerson>,

    /// The category of the recipe—for example, appetizer, entree, etc.
    #[serde(default)]
    pub recipe_category: RecipeCategory,

    /// The cuisine of the recipe (for example, French or Ethiopian).
    pub recipe_cuisine: Option<RecipeCuisine>,

    /// A step in making the recipe, in the form of a single item (document, video, etc.)
    /// or an ordered list with HowToStep and/or HowToSection items.
    pub recipe_ingredient: Option<Vec<String>>,

    /// A step in making the recipe, in the form of a single item (document, video, etc.) or an
    /// ordered list with HowToStep and/or HowToSection items.
    pub recipe_instructions: Option<CreativeWorkOrItemListOrText>,

    /// The quantity produced by the recipe (for example, number of people served, number of servings, etc).
    #[serde(default)]
    pub recipe_yield: QuantitativeValueOrText,

    /// A review of the item. Supersedes reviews.
    #[serde(alias = "Review")]
    pub review: Option<Vec<ReviewType>>,

    /// URL of a reference Web page that unambiguously indicates the item's identity. E.g. the URL
    /// of the item's Wikipedia page, Wikidata entry, or official website.
    pub same_as: Option<Url>,

    /// A single step item (as HowToStep, text, document, video, etc.) or a HowToSection.
    /// Supersedes steps.
    pub step: Option<CreativeWorkOrHowToSectionOrHowToStepOrText>,

    /// Indicates a dietary restriction or guideline for which this recipe or menu item
    /// is suitable, e.g. diabetic, halal etc.
    #[serde(default)]
    pub suitable_for_diet: RestrictedDiet,

    /// A sub-property of instrument. A supply consumed when performing instructions or a direction.
    pub supply: Option<HowToSupplyOrText>,

    /// The textual content of this CreativeWork.
    pub text: Option<String>,

    /// A sub property of instrument. An object used (but not consumed) when performing
    /// instructions or a direction.
    pub tool: Option<HowToToolOrText>,

    /// The total time required to perform instructions or a direction (including time to prepare
    /// the supplies), in ISO 8601 duration format.
    pub total_time: Option<iso8601::Duration>,

    /// The quantity that results by performing instructions. For example, a paper airplane,
    /// 10 personalized candles.
    #[serde(rename = "yield")]
    pub total_yield: Option<QuantitativeValueOrText>,

    /// Thumbnail image for an image or video.
    pub thumbnail: Option<ImageObjectType>,

    /// A thumbnail image relevant to the Thing.
    pub thumbnail_url: Option<Url>,

    /// The work that this work has been translated from. E.g. 物种起源 is a translationOf
    /// “On the Origin of Species”. Inverse property: workTranslation
    pub translation_of_work: Option<CreativeWorkType>,

    /// URL of the item.
    pub url: Option<Url>,

    /// An embedded video object.
    pub video: Option<ClipOrVideoObject>,

    /// Example/instance/realization/derivation of the concept of this creative work. E.g.
    /// the paperback edition, first edition, or e-book.
    pub work_example: Option<CreativeWorkType>,

    /// A work that is a translation of the content of this work. E.g. 西遊記 has an English workTranslation
    /// “Journey to the West”, a German workTranslation “Monkeys Pilgerfahrt” and a Vietnamese translation
    /// Tây du ký bình khảo. Inverse property: translationOfWork
    pub work_translation: Option<CreativeWorkType>,
}

fn deserialize_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    struct Visitor;

    impl<'de> serde::de::Visitor<'de> for Visitor {
        type Value = bool;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a bool or text")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(v.to_lowercase() == "true")
        }

        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(v.to_lowercase() == "true")
        }

        fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(v)
        }
    }

    deserializer.deserialize_any(Visitor)
}

#[derive(Debug, PartialEq)]
pub enum RecipeCategory {
    Text(String),
}

impl Default for RecipeCategory {
    fn default() -> Self {
        RecipeCategory::Text("uncategorized".to_string())
    }
}

impl<'de> Deserialize<'de> for RecipeCategory {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use crate::schema::recipe::RecipeCategory::Text;

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

#[derive(Debug, PartialEq)]
pub enum RecipeCuisine {
    Text(String),
}

impl<'de> Deserialize<'de> for RecipeCuisine {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use crate::schema::recipe::RecipeCuisine::Text;

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
