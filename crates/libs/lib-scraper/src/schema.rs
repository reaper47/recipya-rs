use iso8601::{DateTime, Duration};
use reqwest::Url;
use serde::{de, de::Unexpected, Deserialize, Deserializer};

#[derive(Debug, Deserialize, PartialEq)]
pub enum RestrictedDiet {
    DiabeticDiet,
    GlutenFreeDiet,
    HalalDiet,
    HinduDiet,
    KosherDiet,
    LowCalorieDiet,
    LowFatDiet,
    LowLactoseDiet,
    LowSaltDiet,
    VeganDiet,
    VegetarianDiet,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Container<T> {
    Item(T),
    Vec(Vec<T>),
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub enum AtType {
    AggregateRating,
    HowToStep,
    NewsArticle,
    Organization,
    Person,
    Recipe,
    VideoObject,
}

#[derive(Debug, PartialEq)]
pub enum AtContext {
    SchemaDotOrg,
}

impl<'de> Deserialize<'de> for AtContext {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "http://schema.org" => Ok(AtContext::SchemaDotOrg),
            "https://schema.org" => Ok(AtContext::SchemaDotOrg),
            _ => Err(de::Error::invalid_value(
                Unexpected::Str(&s),
                &"another context",
            )),
        }
    }
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Yield {
    Num(i32),
    VecStr(Vec<String>)
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Organization {
    #[serde(rename = "@type")]
    pub at_type: AtType,
    pub name: String,
    pub url: Option<Url>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct AggregateRating {
    #[serde(rename = "@type")]
    pub at_type: AtType,
    #[serde(rename = "ratingValue")]
    pub rating_value: i64,
    #[serde(rename = "bestRating")]
    pub best_rating: i64,
    #[serde(rename = "ratingCount")]
    pub rating_count: i64,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Instructions {
    Text(String),
    ItemList(Vec<HowTo>),
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct HowTo {
    #[serde(rename = "@type")]
    pub at_type: AtType,
    pub name: String,
    pub text: String,
    pub url: Url,
    #[serde(deserialize_with = "deserialize_string")]
    pub image: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Video {
    Object(VideoObject),
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct VideoObject {
    #[serde(rename = "@type")]
    pub at_type: AtType,
    pub name: String,
    pub description: String,
    #[serde(rename = "thumbnailUrl")]
    pub thumbnail_url: Vec<Url>,
    #[serde(rename = "contentUrl")]
    pub content_url: Url,
    #[serde(rename = "embedUrl")]
    pub embed_url: Url,
    #[serde(rename = "uploadDate")]
    pub upload_date: Option<DateTime>,
    #[serde(deserialize_with = "deserialize_duration")]
    pub duration: Option<Duration>,
}

fn deserialize_duration<'de, D>(deserializer: D) -> Result<Option<Duration>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(s) => {
            if s.is_empty() {
                return Ok(None)
            }
            let (_, dur) = iso8601::parsers::parse_duration(&s.as_bytes()).map_err(de::Error::custom)?;
            Ok(Some(dur))
        },
        None => Ok(None),
    }
}

fn deserialize_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(s) => {
            if s.is_empty() {
                return Ok(None)
            }
            Ok(Some(s))
        },
        None => Ok(None),
    }
}

/// The recipe schema as described in the [schema](https://schema.org/Recipe).
#[derive(Debug, Default, Deserialize, PartialEq)]
pub struct RecipeSchema {
    /// The context of the JSON.
    #[serde(rename = "@context")]
    pub at_context: Option<AtContext>, // TODO: enum for @context

    /// The type of schema object.
    #[serde(rename = "@type")]
    pub at_type: Option<Container<AtType>>, // TODO: enum for @type

    /// The time it takes to actually cook the dish, in ISO 8601 duration format.
    #[serde(alias = "CookTime", rename = "cookTime")]
    pub cook_time: Option<iso8601::Duration>,

    /// The method of cooking, such as Frying, Steaming, etc.
    #[serde(alias = "CookingMethod", rename = "cookingMethod")]
    pub cooking_method: Option<String>,

    /// Nutrition information about the recipe or menu item.
    pub nutrition: Option<NutritionInformationSchema>,

    /// The category of the recipe—for example, appetizer, entree, etc.
    #[serde(rename = "recipeCategory")]
    pub recipe_category: Option<Container<String>>,

    /// The cuisine of the recipe (for example, French or Ethiopian).
    #[serde(rename = "recipeCuisine")]
    pub recipe_cuisine: Option<Container<String>>,

    /// A step in making the recipe, in the form of a single item (document, video, etc.)
    /// or an ordered list with HowToStep and/or HowToSection items.
    #[serde(rename = "recipeIngredient")]
    pub recipe_ingredients: Option<Vec<String>>, // TODO: HowToStep

    /// A step in making the recipe, in the form of a single item (document, video, etc.) or an
    /// ordered list with HowToStep and/or HowToSection items.
    #[serde(rename = "recipeInstructions")]
    pub recipe_instructions: Option<Instructions>, // TODO: CreativeWork or ItemList or Text

    /// The quantity produced by the recipe (for example, number of people served, number of servings, etc).
    #[serde(rename = "recipeYield")]
    pub recipe_yield: Option<Yield>, // TODO: f32 or text

    /// Indicates a dietary restriction or guideline for which this recipe or menu item
    /// is suitable, e.g. diabetic, halal etc.
    #[serde(rename = "suitable_for_diet")]
    pub suitable_for_diet: Option<RestrictedDiet>,

    /// The estimated cost of the supply or supplies consumed when performing instructions.
    #[serde(rename = "estimatedCost")]
    pub estimated_cost: Option<String>, // TODO: Monetary amount or text

    /// The length of time it takes to perform instructions or a direction (not including time to
    /// prepare the supplies), in ISO 8601 duration format.
    #[serde(rename = "performTime")]
    pub perform_time: Option<iso8601::Duration>,

    /// The length of time it takes to prepare the items to be used in instructions or a
    /// direction, in ISO 8601 duration format.
    #[serde(alias = "PrepTime", rename = "prepTime")]
    pub prep_time: Option<iso8601::Duration>,

    /// A single step item (as HowToStep, text, document, video, etc.) or a HowToSection.
    /// Supersedes steps.
    pub step: Option<String>, // TODO: CreativeWork  or HowToSection  or HowToStep  or Text

    /// A sub-property of instrument. A supply consumed when performing instructions or a direction.
    pub supply: Option<String>, // TODO: HowToSupply  or Text

    /// A sub property of instrument. An object used (but not consumed) when performing
    /// instructions or a direction.
    pub tool: Option<String>, // TODO: HowToTool  or Text

    /// The total time required to perform instructions or a direction (including time to prepare
    /// the supplies), in ISO 8601 duration format.
    #[serde(rename = "totalTime")]
    pub total_time: Option<iso8601::Duration>,

    /// The quantity that results by performing instructions. For example, a paper airplane,
    /// 10 personalized candles.
    #[serde(rename = "yield")]
    pub total_yield: Option<i32>, // TODO: QuantitativeValue or Text

    /// Official rating of a piece of content—for example, 'MPAA PG-13'.
    #[serde(alias = "ContentRating", rename = "contentRating")]
    pub content_rating: Option<String>, // TODO: Rating

    /// The overall rating, based on a collection of reviews or ratings, of the item.
    #[serde(rename = "aggregateRating")]
    pub aggregate_rating: Option<AggregateRating>, // TODO: AggregateRating

    /// An embedded audio object.
    pub audio: Option<String>, // TODO: AudioObject  or Clip  or MusicRecording

    /// The author of this content or rating. Please note that author is special in that HTML 5
    /// provides a special mechanism for indicating authorship via the rel tag. That is equivalent
    /// to this and may be used interchangeably.
    pub author: Option<Organization>, // TODO: Organization  or Person

    /// An award won by or for this item. Supersedes awards.
    pub award: Option<String>,

    /// A citation or reference to another creative work, such as another publication, web page,
    /// scholarly article, etc.
    pub citation: Option<String>, // TODO: CreativeWork or Text

    /// Comments, typically from users.
    pub comment: Option<String>, // TODO: Comment

    /// The number of comments this CreativeWork (e.g. Article, Question or Answer) has received.
    /// This is most applicable to works published in Web sites with commenting system; additional
    /// comments may exist elsewhere.
    #[serde(rename = "commentCount")]
    pub comment_count: Option<i32>,

    /// The location depicted or described in the content. For example, the location in a
    /// photograph or painting.
    #[serde(rename = "contentLocation")]
    pub content_location: Option<String>, // TODO: Place

    /// A secondary contributor to the CreativeWork or Event.
    pub contributor: Option<String>, // TODO: Organization  or Person

    /// The country of origin of something, including products as well as creative works such as
    /// movie and TV content.
    #[serde(rename = "countryOfOrigin")]
    pub country_of_origin: Option<String>, // TODO: Country

    /// Text that can be used to credit person(s) and/or organization(s) associated with a
    /// published Creative Work.
    #[serde(rename = "creditText")]
    pub credit_text: Option<String>,

    /// The date on which the CreativeWork was created or the item was added to a DataFeed.
    #[serde(rename = "dateCreated")]
    pub date_created: Option<String>, // TODO: Date or DateTime

    /// The date on which the CreativeWork was most recently modified or when the item's entry
    /// was modified within a DataFeed.
    #[serde(rename = "dateModified")]
    pub date_modified: Option<DateTime>, // TODO: Date or DateTime

    /// Date of first publication or broadcast. For example the date a CreativeWork was broadcast
    /// or a Certification was issued.
    #[serde(rename = "datePublished")]
    pub date_published: Option<DateTime>, // TODO: Date or DateTime

    /// Headline of the article.
    pub headline: Option<String>,

    /// The language of the content or performance or used in an action. Please use one of the
    /// language codes from the IETF BCP 47 standard. See also availableLanguage. Supersedes language.
    #[serde(rename = "inLanguage")]
    pub in_language: Option<String>, // TODO: Language or Text

    /// Keywords or tags used to describe some item. Multiple textual entries in a keywords list
    /// are typically delimited by commas, or by repeating the property.
    #[serde(alias = "Keywords")]
    pub keywords: Option<Container<String>>, // TODO: DefinedTerm, Text or URL

    /// The location where the CreativeWork was created, which may not be the same as the location
    /// depicted in the CreativeWork.
    #[serde(rename = "locationCreated")]
    pub location_created: Option<String>, // TODO: Place

    /// A review of the item. Supersedes reviews.
    pub review: Option<String>, // TODO: Review

    /// The textual content of this CreativeWork.
    pub text: Option<String>,

    /// Thumbnail image for an image or video.
    pub thumbnail: Option<String>, // TODO: ImageObject

    /// A thumbnail image relevant to the Thing.
    #[serde(rename = "thumbnailUrl")]
    pub thumbnail_url: Option<Url>, // TODO: URL\

    /// The work that this work has been translated from. E.g. 物种起源 is a translationOf
    /// “On the Origin of Species”. Inverse property: workTranslation
    #[serde(rename = "translationOfWork")]
    pub translation_of_work: Option<String>, // TODO: CreativeWork

    /// An embedded video object.
    pub video: Option<VideoObject>, // TODO: Clip or VideoObject

    /// Example/instance/realization/derivation of the concept of this creative work. E.g.
    /// the paperback edition, first edition, or e-book.
    #[serde(rename = "workExample")]
    pub work_example: Option<String>, // TODO: CreativeWork

    /// A work that is a translation of the content of this work. E.g. 西遊記 has an English workTranslation
    /// “Journey to the West”, a German workTranslation “Monkeys Pilgerfahrt” and a Vietnamese translation
    /// Tây du ký bình khảo. Inverse property: translationOfWork
    #[serde(rename = "workTranslation")]
    pub work_translation: Option<String>, // TODO: CreativeWork

    /// An alias for the item.
    #[serde(rename = "alternateName")]
    pub alternate_name: Option<String>,

    /// A description of the item.
    pub description: Option<String>, // TODO: Text or TextObject

    /// The identifier property represents any kind of identifier for any kind of Thing, such as
    /// ISBNs, GTIN codes, UUIDs etc. Schema.org provides dedicated properties for representing
    /// many of these, either as textual strings or as URL (URI) links. See background notes for
    /// more details.
    pub identifier: Option<String>, // TODO: PropertyValue, Text or URL

    /// An image of the item. This can be a URL or a fully described ImageObject.
    pub image: Option<Container<Url>>, // TODO: ImageObject or URL

    /// The name of the item.
    pub name: Option<String>,

    /// URL of a reference Web page that unambiguously indicates the item's identity. E.g. the URL
    /// of the item's Wikipedia page, Wikidata entry, or official website.
    #[serde(rename = "sameAs")]
    pub same_as: Option<String>, // TODO: URL

    /// URL of the item.
    pub url: Option<String>, // TODO: URL
}

/// Nutritional information about the recipe as described in the [schema](https://schema.org/NutritionInformation).
#[derive(Debug, Default, Deserialize, PartialEq)]
pub struct NutritionInformationSchema {
    /// The number of calories (kcal).
    calories: Option<f32>,

    /// The number of grams of carbohydrates.
    #[serde(rename = "carbohydrateContent")]
    carbohydrate_content: Option<f32>,

    /// The number of milligrams of cholesterol.
    #[serde(rename = "cholesterolContent")]
    cholesterol_content: Option<f32>,

    /// The number of grams of fat.
    #[serde(rename = "fatContent")]
    fat_content: Option<f32>,

    /// The number of grams of fiber.
    #[serde(rename = "fiberContent")]
    fiber_content: Option<f32>,

    /// The number of grams of protein.
    #[serde(rename = "proteinContent")]
    protein_content: Option<f32>,

    /// The number of grams of saturated fat.
    #[serde(rename = "saturatedFatContent")]
    saturated_fat_content: Option<f32>,

    /// The serving size, in terms of the number of volume or mass.
    #[serde(rename = "servingSize")]
    serving_size: Option<String>,

    /// The number of milligrams of sodium.
    #[serde(rename = "sodiumContent")]
    sodium_content: Option<f32>,

    /// The number of grams of sugar.
    #[serde(rename = "sugarContent")]
    sugar_content: Option<f32>,

    /// The number of grams of trans fat.
    #[serde(rename = "transFatContent")]
    trans_fat_content: Option<f32>,

    /// The number of grams of unsaturated fat.
    #[serde(rename = "unsaturatedFatContent")]
    unsaturated_fat_content: Option<f32>,
}
