use serde::{Deserialize, Serialize};

use crate::enums::RestrictedDietEnum;
use crate::field::{
    RecipeArchivedAtFieldEnum, RecipeAudioFieldEnum, RecipeAuthorFieldEnum,
    RecipeCitationFieldEnum, RecipeContentRatingFieldEnum, RecipeContributorFieldEnum,
    RecipeCreatorFieldEnum, RecipeDescriptionFieldEnum, RecipeEstimatedCostFieldEnum,
    RecipeImageFieldEnum, RecipeInLanguageFieldEnum, RecipeIsBasedOnFieldEnum,
    RecipeIsBasedOnUrlFieldEnum, RecipeIsPartOfFieldEnum, RecipeKeywordsFieldEnum,
    RecipeLicenseFieldEnum, RecipeMaintainerFieldEnum, RecipeProducerFieldEnum,
    RecipeProviderFieldEnum, RecipePublisherFieldEnum, RecipeRecipeIngredientFieldEnum,
    RecipeRecipeInstructionsFieldEnum, RecipeRecipeYieldFieldEnum, RecipeSdPublisherFieldEnum,
    RecipeSizeFieldEnum, RecipeStepFieldEnum, RecipeStepsFieldEnum, RecipeSubjectOfFieldEnum,
    RecipeSupplyFieldEnum, RecipeToolFieldEnum, RecipeTranslatorFieldEnum, RecipeVideoFieldEnum,
    RecipeYieldFieldEnum,
};
use crate::helpers::{deserialize_type, one_or_many, one_or_many_string_or_num};
use crate::{
    AggregateRating, Comment, Country, CreativeWork, Duration, DurationOrText, ImageObject,
    InteractionCounter, NutritionInformation, Person, Review, Thing,
};

///<https://schema.org/dateCreated>
///<https://schema.org/Date>
///<https://schema.org/DateTime>
pub type RecipeDateCreatedFieldEnum = String;
///<https://schema.org/expires>
///<https://schema.org/Date>
///<https://schema.org/DateTime>
pub type RecipeExpiresFieldEnum = String;
///<https://schema.org/dateModified>
///<https://schema.org/Date>
///<https://schema.org/DateTime>
pub type RecipeDateModifiedFieldEnum = String;
///<https://schema.org/datePublished>
///<https://schema.org/Date>
///<https://schema.org/DateTime>
pub type RecipeDatePublishedFieldEnum = String;
///<https://schema.org/schemaVersion>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type RecipeSchemaVersionFieldEnum = String;

/// Enumeration of possible values for the @graph field in JSON-LD used to group
/// multiple related entities in a single document.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum GraphObject {
    Recipe(Box<Recipe>),
    Unknown(serde_json::Value),
}

impl Default for GraphObject {
    fn default() -> Self {
        Self::Unknown(serde_json::Value::Null)
    }
}

///<https://schema.org/Recipe>
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct Recipe {
    #[serde(rename = "@type", deserialize_with = "deserialize_type")]
    pub r#type: Option<String>,
    #[serde(rename = "@graph", skip_serializing_if = "Option::is_none")]
    pub graph: Option<Vec<GraphObject>>,
    #[serde(rename = "@context")]
    pub context: Option<String>,
    ///<https://schema.org/nutrition>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub nutrition: Vec<NutritionInformation>,
    ///<https://schema.org/cookingMethod>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cooking_method: Vec<String>,
    ///<https://schema.org/recipeYield>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub recipe_yield: Vec<RecipeRecipeYieldFieldEnum>,
    ///<https://schema.org/recipeCuisine>
    #[serde(rename = "recipeCuisine")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub recipe_cuisine: Vec<String>,
    ///<https://schema.org/ingredients>
    #[serde(rename = "ingredients", default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ingredients: Vec<String>,
    ///<https://schema.org/recipeIngredient>
    #[serde(rename = "recipeIngredient")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub recipe_ingredient: Vec<RecipeRecipeIngredientFieldEnum>,
    ///<https://schema.org/suitableForDiet>
    #[serde(rename = "suitableForDiet", default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub suitable_for_diet: Vec<RestrictedDietEnum>,
    ///<https://schema.org/cookTime>
    #[serde(rename = "cookTime", default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cook_time: Vec<DurationOrText>,
    ///<https://schema.org/recipeInstructions>
    #[serde(rename = "recipeInstructions")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub recipe_instructions: Vec<RecipeRecipeInstructionsFieldEnum>,
    ///<https://schema.org/recipeCategory>
    #[serde(rename = "recipeCategory")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub recipe_category: Vec<String>,
    ///<https://schema.org/steps>
    #[serde(rename = "steps")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub steps: Vec<RecipeStepsFieldEnum>,
    ///<https://schema.org/yield>
    #[serde(rename = "yield")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub r#yield: Vec<RecipeYieldFieldEnum>,
    ///<https://schema.org/tool>
    #[serde(rename = "tool")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tool: Vec<RecipeToolFieldEnum>,
    ///<https://schema.org/step>
    #[serde(rename = "step")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub step: Vec<RecipeStepFieldEnum>,
    ///<https://schema.org/prepTime>
    #[serde(rename = "prepTime")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub prep_time: Vec<DurationOrText>,
    ///<https://schema.org/estimatedCost>
    #[serde(rename = "estimatedCost")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub estimated_cost: Vec<RecipeEstimatedCostFieldEnum>,
    ///<https://schema.org/totalTime>
    #[serde(rename = "totalTime")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub total_time: Vec<DurationOrText>,
    ///<https://schema.org/performTime>
    #[serde(rename = "performTime")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub perform_time: Vec<Duration>,
    ///<https://schema.org/supply>
    #[serde(rename = "supply")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub supply: Vec<RecipeSupplyFieldEnum>,
    ///<https://schema.org/comment>
    #[serde(rename = "comment")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<Comment>,
    ///<https://schema.org/isBasedOnUrl>
    #[serde(rename = "isBasedOnUrl")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub is_based_on_url: Vec<RecipeIsBasedOnUrlFieldEnum>,
    ///<https://schema.org/translationOfWork>
    #[serde(rename = "translationOfWork")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub translation_of_work: Vec<CreativeWork>,
    ///<https://schema.org/workTranslation>
    #[serde(rename = "workTranslation")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub work_translation: Vec<CreativeWork>,
    ///<https://schema.org/mentions>
    #[serde(rename = "mentions")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub mentions: Vec<Thing>,
    ///<https://schema.org/dateCreated>
    #[serde(rename = "dateCreated")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub date_created: Vec<RecipeDateCreatedFieldEnum>,
    ///<https://schema.org/wordCount>
    #[serde(rename = "wordCount")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub word_count: Vec<i32>,
    ///<https://schema.org/size>
    #[serde(rename = "size")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub size: Vec<RecipeSizeFieldEnum>,
    ///<https://schema.org/maintainer>
    #[serde(rename = "maintainer")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub maintainer: Vec<RecipeMaintainerFieldEnum>,
    ///<https://schema.org/license>
    #[serde(rename = "license")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub license: Vec<RecipeLicenseFieldEnum>,
    ///<https://schema.org/expires>
    #[serde(rename = "expires")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub expires: Vec<RecipeExpiresFieldEnum>,
    ///<https://schema.org/commentCount>
    #[serde(rename = "commentCount")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub comment_count: Vec<i32>,
    ///<https://schema.org/timeRequired>
    #[serde(rename = "timeRequired")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub time_required: Vec<Duration>,
    ///<https://schema.org/review>
    #[serde(rename = "review")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub review: Vec<Review>,
    ///<https://schema.org/contributor>
    #[serde(rename = "contributor")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub contributor: Vec<RecipeContributorFieldEnum>,
    ///<https://schema.org/interactionStatistic>
    #[serde(rename = "interactionStatistic")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub interaction_statistic: Vec<InteractionCounter>,
    ///<https://schema.org/publisher>
    #[serde(rename = "publisher")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub publisher: Vec<RecipePublisherFieldEnum>,
    ///<https://schema.org/creditText>
    #[serde(rename = "creditText")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub credit_text: Vec<String>,
    ///<https://schema.org/headline>
    #[serde(rename = "headline")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub headline: Vec<String>,
    ///<https://schema.org/editor>
    #[serde(rename = "editor")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub editor: Vec<Person>,
    ///<https://schema.org/dateModified>
    #[serde(rename = "dateModified")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub date_modified: Vec<RecipeDateModifiedFieldEnum>,
    ///<https://schema.org/isAccessibleForFree>
    #[serde(rename = "isAccessibleForFree")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub is_accessible_for_free: Vec<String>,
    ///<https://schema.org/keywords>
    #[serde(rename = "keywords")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub keywords: Vec<RecipeKeywordsFieldEnum>,
    ///<https://schema.org/provider>
    #[serde(rename = "provider")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub provider: Vec<RecipeProviderFieldEnum>,
    ///<https://schema.org/creator>
    #[serde(rename = "creator")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub creator: Vec<RecipeCreatorFieldEnum>,
    ///<https://schema.org/sdDatePublished>
    #[serde(rename = "sdDatePublished")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sd_date_published: Vec<String>,
    ///<https://schema.org/contentReferenceTime>
    #[serde(rename = "contentReferenceTime")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub content_reference_time: Vec<String>,
    ///<https://schema.org/archivedAt>
    #[serde(rename = "archivedAt")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub archived_at: Vec<RecipeArchivedAtFieldEnum>,
    ///<https://schema.org/discussionUrl>
    #[serde(rename = "discussionUrl")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub discussion_url: Vec<String>,
    ///<https://schema.org/contentRating>
    #[serde(rename = "contentRating")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub content_rating: Vec<RecipeContentRatingFieldEnum>,
    ///<https://schema.org/countryOfOrigin>
    #[serde(rename = "countryOfOrigin")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub country_of_origin: Vec<Country>,
    ///<https://schema.org/text>
    #[serde(rename = "text")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub text: Vec<String>,
    ///<https://schema.org/award>
    #[serde(rename = "award")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub award: Vec<String>,
    ///<https://schema.org/isBasedOn>
    #[serde(rename = "isBasedOn")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub is_based_on: Vec<RecipeIsBasedOnFieldEnum>,
    ///<https://schema.org/aggregateRating>
    #[serde(rename = "aggregateRating")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub aggregate_rating: Vec<AggregateRating>,
    ///<https://schema.org/inLanguage>
    #[serde(rename = "inLanguage")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub in_language: Vec<RecipeInLanguageFieldEnum>,
    ///<https://schema.org/datePublished>
    #[serde(rename = "datePublished")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub date_published: Vec<RecipeDatePublishedFieldEnum>,
    ///<https://schema.org/sdPublisher>
    #[serde(rename = "sdPublisher")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sd_publisher: Vec<RecipeSdPublisherFieldEnum>,
    ///<https://schema.org/audio>
    #[serde(rename = "audio")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub audio: Vec<RecipeAudioFieldEnum>,
    ///<https://schema.org/alternativeHeadline>
    #[serde(rename = "alternativeHeadline")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub alternative_headline: Vec<String>,
    ///<https://schema.org/about>
    #[serde(rename = "about")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub about: Vec<Thing>,
    ///<https://schema.org/isPartOf>
    #[serde(rename = "isPartOf")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub is_part_of: Vec<RecipeIsPartOfFieldEnum>,
    ///<https://schema.org/thumbnail>
    #[serde(rename = "thumbnail")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub thumbnail: Vec<ImageObject>,
    ///<https://schema.org/thumbnailUrl>
    #[serde(rename = "thumbnailUrl")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub thumbnail_url: Vec<String>,
    ///<https://schema.org/copyrightYear>
    #[serde(rename = "copyrightYear")]
    #[serde(default, deserialize_with = "one_or_many_string_or_num")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub copyright_year: Vec<i32>,
    ///<https://schema.org/workExample>
    #[serde(rename = "workExample")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub work_example: Vec<CreativeWork>,
    ///<https://schema.org/citation>
    #[serde(rename = "citation")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub citation: Vec<RecipeCitationFieldEnum>,
    ///<https://schema.org/video>
    #[serde(rename = "video")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub video: Vec<RecipeVideoFieldEnum>,
    ///<https://schema.org/awards>
    #[serde(rename = "awards")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub awards: Vec<String>,
    ///<https://schema.org/producer>
    #[serde(rename = "producer")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub producer: Vec<RecipeProducerFieldEnum>,
    ///<https://schema.org/schemaVersion>
    #[serde(rename = "schemaVersion")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub schema_version: Vec<RecipeSchemaVersionFieldEnum>,
    ///<https://schema.org/author>
    #[serde(rename = "author")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub author: Vec<RecipeAuthorFieldEnum>,
    ///<https://schema.org/translator>
    #[serde(rename = "translator")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub translator: Vec<RecipeTranslatorFieldEnum>,
    ///<https://schema.org/reviews>
    #[serde(rename = "reviews")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub reviews: Vec<Review>,
    ///<https://schema.org/disambiguatingDescription>
    #[serde(rename = "disambiguatingDescription")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub disambiguating_description: Vec<String>,
    ///<https://schema.org/image>
    #[serde(rename = "image")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub image: Vec<RecipeImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(rename = "sameAs")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(rename = "description")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub description: Vec<RecipeDescriptionFieldEnum>,
    ///<https://schema.org/alternateName>
    #[serde(rename = "alternateName")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub alternate_name: Vec<String>,
    ///<https://schema.org/url>
    #[serde(rename = "url")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub url: Vec<String>,
    ///<https://schema.org/subjectOf>
    #[serde(rename = "subjectOf")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub subject_of: Vec<RecipeSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(rename = "name")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub name: Vec<String>,
}

impl Recipe {
    /// Generates the schema definition of `RecipeSchema`.
    #[cfg(feature = "json-schema")]
    pub fn schema() -> String {
        let schema = schemars::schema_for!(Self);
        serde_json::to_string_pretty(&schema).unwrap_or_default()
    }
}
