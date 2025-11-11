use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::enums::{IPTCDigitalSourceEnumerationEnum, RestrictedDietEnum};
use crate::field::*;
use crate::helpers::one_or_many;
use crate::nutrition_information::NutritionInformation;
use crate::{Action, Comment, CreativeWork, Duration, Event, Place, Review, Thing};
use crate::aggregate_rating::AggregateRating;
use crate::alignment_object::AlignmentObject;
use crate::audience::Audience;
use crate::claim::Claim;
use crate::country::Country;
use crate::grant::Grant;
use crate::image_object::ImageObject;
use crate::interaction_counter::InteractionCounter;
use crate::item_list::ItemList;
use crate::media_object::MediaObject;
use crate::organization::Organization;
use crate::person::Person;
use crate::publication_event::PublicationEvent;

///<https://schema.org/dateCreated>
///<https://schema.org/Date>
///<https://schema.org/DateTime>
pub type RecipeDateCreatedFieldEnum = String;
///<https://schema.org/expires>
///<https://schema.org/Date>
///<https://schema.org/DateTime>
pub type RecipeExpiresFieldEnum = String;
///<https://schema.org/temporalCoverage>
///<https://schema.org/DateTime>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type RecipeTemporalCoverageFieldEnum = String;
///<https://schema.org/fileFormat>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type RecipeFileFormatFieldEnum = String;
///<https://schema.org/dateModified>
///<https://schema.org/Date>
///<https://schema.org/DateTime>
pub type RecipeDateModifiedFieldEnum = String;
///<https://schema.org/encodingFormat>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type RecipeEncodingFormatFieldEnum = String;
///<https://schema.org/temporal>
///<https://schema.org/DateTime>
///<https://schema.org/Text>
pub type RecipeTemporalFieldEnum = String;
///<https://schema.org/datePublished>
///<https://schema.org/Date>
///<https://schema.org/DateTime>
pub type RecipeDatePublishedFieldEnum = String;
///<https://schema.org/genre>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type RecipeGenreFieldEnum = String;
///<https://schema.org/editEIDR>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type RecipeEditEIDRFieldEnum = String;
///<https://schema.org/schemaVersion>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type RecipeSchemaVersionFieldEnum = String;
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type RecipeAdditionalTypeFieldEnum = String;

///<https://schema.org/Recipe>
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Recipe {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/nutrition>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub nutrition: Vec<NutritionInformation>,
    ///<https://schema.org/cookingMethod>
    #[serde(default, deserialize_with = "one_or_many")]
    pub cooking_method: Vec<String>,
    ///<https://schema.org/recipeYield>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub recipe_yield: Vec<RecipeRecipeYieldFieldEnum>,
    ///<https://schema.org/recipeCuisine>
    #[serde(rename = "recipeCuisine")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub recipe_cuisine: Vec<String>,
    ///<https://schema.org/ingredients>
    #[serde(rename = "ingredients")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub ingredients: Vec<String>,
    ///<https://schema.org/recipeIngredient>
    #[serde(rename = "recipeIngredient")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub recipe_ingredient: Vec<RecipeRecipeIngredientFieldEnum>,
    ///<https://schema.org/suitableForDiet>
    #[serde(rename = "suitableForDiet")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub suitable_for_diet: Vec<RestrictedDietEnum>,
    ///<https://schema.org/cookTime>
    #[serde(rename = "cookTime")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub cook_time: Vec<Duration>,
    ///<https://schema.org/recipeInstructions>
    #[serde(rename = "recipeInstructions")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub recipe_instructions: Vec<RecipeRecipeInstructionsFieldEnum>,
    ///<https://schema.org/recipeCategory>
    #[serde(rename = "recipeCategory")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub recipe_category: Vec<String>,
    ///<https://schema.org/steps>
    #[serde(rename = "steps")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub steps: Vec<RecipeStepsFieldEnum>,
    ///<https://schema.org/yield>
    #[serde(rename = "yield")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub _yield: Vec<RecipeYieldFieldEnum>,
    ///<https://schema.org/tool>
    #[serde(rename = "tool")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub tool: Vec<RecipeToolFieldEnum>,
    ///<https://schema.org/step>
    #[serde(rename = "step")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub step: Vec<RecipeStepFieldEnum>,
    ///<https://schema.org/prepTime>
    #[serde(rename = "prepTime")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub prep_time: Vec<Duration>,
    ///<https://schema.org/estimatedCost>
    #[serde(rename = "estimatedCost")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub estimated_cost: Vec<RecipeEstimatedCostFieldEnum>,
    ///<https://schema.org/totalTime>
    #[serde(rename = "totalTime")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub total_time: Vec<Duration>,
    ///<https://schema.org/performTime>
    #[serde(rename = "performTime")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub perform_time: Vec<Duration>,
    ///<https://schema.org/supply>
    #[serde(rename = "supply")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub supply: Vec<RecipeSupplyFieldEnum>,
    ///<https://schema.org/contentLocation>
    #[serde(rename = "contentLocation")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub content_location: Vec<Place>,
    ///<https://schema.org/recordedAt>
    #[serde(rename = "recordedAt")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub recorded_at: Vec<Event>,
    ///<https://schema.org/comment>
    #[serde(rename = "comment")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub comment: Vec<Comment>,
    ///<https://schema.org/isBasedOnUrl>
    #[serde(rename = "isBasedOnUrl")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub is_based_on_url: Vec<RecipeIsBasedOnUrlFieldEnum>,
    ///<https://schema.org/translationOfWork>
    #[serde(rename = "translationOfWork")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub translation_of_work: Vec<CreativeWork>,
    ///<https://schema.org/workTranslation>
    #[serde(rename = "workTranslation")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub work_translation: Vec<CreativeWork>,
    ///<https://schema.org/mentions>
    #[serde(rename = "mentions")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub mentions: Vec<Thing>,
    ///<https://schema.org/dateCreated>
    #[serde(rename = "dateCreated")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub date_created: Vec<RecipeDateCreatedFieldEnum>,
    ///<https://schema.org/wordCount>
    #[serde(rename = "wordCount")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub word_count: Vec<i32>,
    ///<https://schema.org/size>
    #[serde(rename = "size")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub size: Vec<RecipeSizeFieldEnum>,
    ///<https://schema.org/maintainer>
    #[serde(rename = "maintainer")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub maintainer: Vec<RecipeMaintainerFieldEnum>,
    ///<https://schema.org/license>
    #[serde(rename = "license")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub license: Vec<RecipeLicenseFieldEnum>,
    ///<https://schema.org/expires>
    #[serde(rename = "expires")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub expires: Vec<RecipeExpiresFieldEnum>,
    ///<https://schema.org/version>
    #[serde(rename = "version")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub version: Vec<RecipeVersionFieldEnum>,
    ///<https://schema.org/educationalLevel>
    #[serde(rename = "educationalLevel")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub educational_level: Vec<RecipeEducationalLevelFieldEnum>,
    ///<https://schema.org/commentCount>
    #[serde(rename = "commentCount")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub comment_count: Vec<i32>,
    ///<https://schema.org/offers>
    #[serde(rename = "offers")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub offers: Vec<RecipeOffersFieldEnum>,
    ///<https://schema.org/timeRequired>
    #[serde(rename = "timeRequired")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub time_required: Vec<Duration>,
    ///<https://schema.org/audience>
    #[serde(rename = "audience")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub audience: Vec<Audience>,
    ///<https://schema.org/review>
    #[serde(rename = "review")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub review: Vec<Review>,
    ///<https://schema.org/contributor>
    #[serde(rename = "contributor")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub contributor: Vec<RecipeContributorFieldEnum>,
    ///<https://schema.org/temporalCoverage>
    #[serde(rename = "temporalCoverage")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub temporal_coverage: Vec<RecipeTemporalCoverageFieldEnum>,
    ///<https://schema.org/interactionStatistic>
    #[serde(rename = "interactionStatistic")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub interaction_statistic: Vec<InteractionCounter>,
    ///<https://schema.org/mainEntity>
    #[serde(rename = "mainEntity")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub main_entity: Vec<Thing>,
    ///<https://schema.org/publisher>
    #[serde(rename = "publisher")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub publisher: Vec<RecipePublisherFieldEnum>,
    ///<https://schema.org/creditText>
    #[serde(rename = "creditText")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub credit_text: Vec<String>,
    ///<https://schema.org/character>
    #[serde(rename = "character")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub character: Vec<Person>,
    ///<https://schema.org/copyrightNotice>
    #[serde(rename = "copyrightNotice")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub copyright_notice: Vec<String>,
    ///<https://schema.org/headline>
    #[serde(rename = "headline")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub headline: Vec<String>,
    ///<https://schema.org/fileFormat>
    #[serde(rename = "fileFormat")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub file_format: Vec<RecipeFileFormatFieldEnum>,
    ///<https://schema.org/material>
    #[serde(rename = "material")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub material: Vec<RecipeMaterialFieldEnum>,
    ///<https://schema.org/hasPart>
    #[serde(rename = "hasPart")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub has_part: Vec<CreativeWork>,
    ///<https://schema.org/editor>
    #[serde(rename = "editor")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub editor: Vec<Person>,
    ///<https://schema.org/publication>
    #[serde(rename = "publication")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub publication: Vec<PublicationEvent>,
    ///<https://schema.org/accessibilityHazard>
    #[serde(rename = "accessibilityHazard")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub accessibility_hazard: Vec<String>,
    ///<https://schema.org/dateModified>
    #[serde(rename = "dateModified")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub date_modified: Vec<RecipeDateModifiedFieldEnum>,
    ///<https://schema.org/isAccessibleForFree>
    #[serde(rename = "isAccessibleForFree")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub is_accessible_for_free: Vec<String>,
    ///<https://schema.org/encodings>
    #[serde(rename = "encodings")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub encodings: Vec<MediaObject>,
    ///<https://schema.org/keywords>
    #[serde(rename = "keywords")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub keywords: Vec<RecipeKeywordsFieldEnum>,
    ///<https://schema.org/encodingFormat>
    #[serde(rename = "encodingFormat")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub encoding_format: Vec<RecipeEncodingFormatFieldEnum>,
    ///<https://schema.org/provider>
    #[serde(rename = "provider")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub provider: Vec<RecipeProviderFieldEnum>,
    ///<https://schema.org/creator>
    #[serde(rename = "creator")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub creator: Vec<RecipeCreatorFieldEnum>,
    ///<https://schema.org/accessMode>
    #[serde(rename = "accessMode")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub access_mode: Vec<String>,
    ///<https://schema.org/sdDatePublished>
    #[serde(rename = "sdDatePublished")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub sd_date_published: Vec<String>,
    ///<https://schema.org/exampleOfWork>
    #[serde(rename = "exampleOfWork")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub example_of_work: Vec<CreativeWork>,
    ///<https://schema.org/assesses>
    #[serde(rename = "assesses")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub assesses: Vec<RecipeAssessesFieldEnum>,
    ///<https://schema.org/contentReferenceTime>
    #[serde(rename = "contentReferenceTime")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub content_reference_time: Vec<String>,
    ///<https://schema.org/locationCreated>
    #[serde(rename = "locationCreated")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub location_created: Vec<Place>,
    ///<https://schema.org/teaches>
    #[serde(rename = "teaches")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub teaches: Vec<RecipeTeachesFieldEnum>,
    ///<https://schema.org/archivedAt>
    #[serde(rename = "archivedAt")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub archived_at: Vec<RecipeArchivedAtFieldEnum>,
    ///<https://schema.org/accessibilitySummary>
    #[serde(rename = "accessibilitySummary")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub accessibility_summary: Vec<String>,
    ///<https://schema.org/encoding>
    #[serde(rename = "encoding")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub encoding: Vec<MediaObject>,
    ///<https://schema.org/typicalAgeRange>
    #[serde(rename = "typicalAgeRange")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub typical_age_range: Vec<String>,
    ///<https://schema.org/interpretedAsClaim>
    #[serde(rename = "interpretedAsClaim")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub interpreted_as_claim: Vec<Claim>,
    ///<https://schema.org/publisherImprint>
    #[serde(rename = "publisherImprint")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub publisher_imprint: Vec<Organization>,
    ///<https://schema.org/discussionUrl>
    #[serde(rename = "discussionUrl")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub discussion_url: Vec<String>,
    ///<https://schema.org/contentRating>
    #[serde(rename = "contentRating")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub content_rating: Vec<RecipeContentRatingFieldEnum>,
    ///<https://schema.org/funder>
    #[serde(rename = "funder")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub funder: Vec<RecipeGenreFieldEnum>,
    ///<https://schema.org/countryOfOrigin>
    #[serde(rename = "countryOfOrigin")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub country_of_origin: Vec<Country>,
    ///<https://schema.org/text>
    #[serde(rename = "text")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub text: Vec<String>,
    ///<https://schema.org/accountablePerson>
    #[serde(rename = "accountablePerson")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub accountable_person: Vec<Person>,
    ///<https://schema.org/temporal>
    #[serde(rename = "temporal")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub temporal: Vec<RecipeTemporalFieldEnum>,
    ///<https://schema.org/associatedMedia>
    #[serde(rename = "associatedMedia")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub associated_media: Vec<MediaObject>,
    ///<https://schema.org/spatialCoverage>
    #[serde(rename = "spatialCoverage")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub spatial_coverage: Vec<Place>,
    ///<https://schema.org/award>
    #[serde(rename = "award")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub award: Vec<String>,
    ///<https://schema.org/isBasedOn>
    #[serde(rename = "isBasedOn")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub is_based_on: Vec<RecipeIsBasedOnFieldEnum>,
    ///<https://schema.org/pattern>
    #[serde(rename = "pattern")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub pattern: Vec<RecipePatternFieldEnum>,
    ///<https://schema.org/interactivityType>
    #[serde(rename = "interactivityType")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub interactivity_type: Vec<String>,
    ///<https://schema.org/abstract>
    #[serde(rename = "abstract")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub _abstract: Vec<String>,
    ///<https://schema.org/sponsor>
    #[serde(rename = "sponsor")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub sponsor: Vec<RecipeSponsorFieldEnum>,
    ///<https://schema.org/aggregateRating>
    #[serde(rename = "aggregateRating")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub aggregate_rating: Vec<AggregateRating>,
    ///<https://schema.org/correction>
    #[serde(rename = "correction")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub correction: Vec<RecipeCorrectionFieldEnum>,
    ///<https://schema.org/accessibilityAPI>
    #[serde(rename = "accessibilityAPI")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub accessibility_api: Vec<String>,
    ///<https://schema.org/copyrightHolder>
    #[serde(rename = "copyrightHolder")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub copyright_holder: Vec<RecipeCopyrightHolderFieldEnum>,
    ///<https://schema.org/inLanguage>
    #[serde(rename = "inLanguage")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub in_language: Vec<RecipeInLanguageFieldEnum>,
    ///<https://schema.org/usageInfo>
    #[serde(rename = "usageInfo")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub usage_info: Vec<RecipeUsageInfoFieldEnum>,
    ///<https://schema.org/datePublished>
    #[serde(rename = "datePublished")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub date_published: Vec<RecipeDatePublishedFieldEnum>,
    ///<https://schema.org/isFamilyFriendly>
    #[serde(rename = "isFamilyFriendly")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub is_family_friendly: Vec<String>,
    ///<https://schema.org/digitalSourceType>
    #[serde(rename = "digitalSourceType")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub digital_source_type: Vec<IPTCDigitalSourceEnumerationEnum>,
    ///<https://schema.org/sdPublisher>
    #[serde(rename = "sdPublisher")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub sd_publisher: Vec<RecipeSdPublisherFieldEnum>,
    ///<https://schema.org/genre>
    #[serde(rename = "genre")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub genre: Vec<RecipeGenreFieldEnum>,
    ///<https://schema.org/audio>
    #[serde(rename = "audio")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub audio: Vec<RecipeAudioFieldEnum>,
    ///<https://schema.org/creativeWorkStatus>
    #[serde(rename = "creativeWorkStatus")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub creative_work_status: Vec<RecipeCreativeWorkStatusFieldEnum>,
    ///<https://schema.org/alternativeHeadline>
    #[serde(rename = "alternativeHeadline")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub alternative_headline: Vec<String>,
    ///<https://schema.org/editEIDR>
    #[serde(rename = "editEIDR")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub edit_eidr: Vec<RecipeEditEIDRFieldEnum>,
    ///<https://schema.org/learningResourceType>
    #[serde(rename = "learningResourceType")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub learning_resource_type: Vec<RecipeLearningResourceTypeFieldEnum>,
    ///<https://schema.org/about>
    #[serde(rename = "about")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub about: Vec<Thing>,
    ///<https://schema.org/isPartOf>
    #[serde(rename = "isPartOf")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub is_part_of: Vec<RecipeIsPartOfFieldEnum>,
    ///<https://schema.org/funding>
    #[serde(rename = "funding")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub funding: Vec<Grant>,
    ///<https://schema.org/educationalAlignment>
    #[serde(rename = "educationalAlignment")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub educational_alignment: Vec<AlignmentObject>,
    ///<https://schema.org/accessModeSufficient>
    #[serde(rename = "accessModeSufficient")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub access_mode_sufficient: Vec<ItemList>,
    ///<https://schema.org/acquireLicensePage>
    #[serde(rename = "acquireLicensePage")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub acquire_license_page: Vec<RecipeAcquireLicensePageFieldEnum>,
    ///<https://schema.org/conditionsOfAccess>
    #[serde(rename = "conditionsOfAccess")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub conditions_of_access: Vec<String>,
    ///<https://schema.org/thumbnail>
    #[serde(rename = "thumbnail")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub thumbnail: Vec<ImageObject>,
    ///<https://schema.org/publishingPrinciples>
    #[serde(rename = "publishingPrinciples")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub publishing_principles: Vec<RecipePublishingPrinciplesFieldEnum>,
    ///<https://schema.org/thumbnailUrl>
    #[serde(rename = "thumbnailUrl")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub thumbnail_url: Vec<String>,
    ///<https://schema.org/copyrightYear>
    #[serde(rename = "copyrightYear")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub copyright_year: Vec<f32>,
    ///<https://schema.org/workExample>
    #[serde(rename = "workExample")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub work_example: Vec<CreativeWork>,
    ///<https://schema.org/accessibilityFeature>
    #[serde(rename = "accessibilityFeature")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub accessibility_feature: Vec<String>,
    ///<https://schema.org/citation>
    #[serde(rename = "citation")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub citation: Vec<RecipeCitationFieldEnum>,
    ///<https://schema.org/video>
    #[serde(rename = "video")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub video: Vec<RecipeVideoFieldEnum>,
    ///<https://schema.org/awards>
    #[serde(rename = "awards")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub awards: Vec<String>,
    ///<https://schema.org/spatial>
    #[serde(rename = "spatial")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub spatial: Vec<Place>,
    ///<https://schema.org/producer>
    #[serde(rename = "producer")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub producer: Vec<RecipeProducerFieldEnum>,
    ///<https://schema.org/schemaVersion>
    #[serde(rename = "schemaVersion")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub schema_version: Vec<RecipeSchemaVersionFieldEnum>,
    ///<https://schema.org/accessibilityControl>
    #[serde(rename = "accessibilityControl")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub accessibility_control: Vec<String>,
    ///<https://schema.org/author>
    #[serde(rename = "author")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub author: Vec<RecipeAuthorFieldEnum>,
    ///<https://schema.org/translator>
    #[serde(rename = "translator")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub translator: Vec<RecipeTranslatorFieldEnum>,
    ///<https://schema.org/materialExtent>
    #[serde(rename = "materialExtent")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub material_extent: Vec<RecipeMaterialExtentFieldEnum>,
    ///<https://schema.org/sourceOrganization>
    #[serde(rename = "sourceOrganization")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub source_organization: Vec<Organization>,
    ///<https://schema.org/position>
    #[serde(rename = "position")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub position: Vec<RecipePositionFieldEnum>,
    ///<https://schema.org/educationalUse>
    #[serde(rename = "educationalUse")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub educational_use: Vec<RecipeEducationalUseFieldEnum>,
    ///<https://schema.org/sdLicense>
    #[serde(rename = "sdLicense")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub sd_license: Vec<RecipeSdLicenseFieldEnum>,
    ///<https://schema.org/releasedEvent>
    #[serde(rename = "releasedEvent")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub released_event: Vec<PublicationEvent>,
    ///<https://schema.org/reviews>
    #[serde(rename = "reviews")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub reviews: Vec<Review>,
    ///<https://schema.org/disambiguatingDescription>
    #[serde(rename = "disambiguatingDescription")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub disambiguating_description: Vec<String>,
    ///<https://schema.org/potentialAction>
    #[serde(rename = "potentialAction")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub potential_action: Vec<Action>,
    ///<https://schema.org/additionalType>
    #[serde(rename = "additionalType")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub additional_type: Vec<RecipeAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(rename = "identifier")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub identifier: Vec<RecipeIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(rename = "image")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub image: Vec<RecipeImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(rename = "sameAs")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(rename = "description")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub description: Vec<RecipeDescriptionFieldEnum>,
    ///<https://schema.org/alternateName>
    #[serde(rename = "alternateName")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub alternate_name: Vec<String>,
    ///<https://schema.org/url>
    #[serde(rename = "url")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub url: Vec<String>,
    ///<https://schema.org/subjectOf>
    #[serde(rename = "subjectOf")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub subject_of: Vec<RecipeSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(rename = "name")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(rename = "mainEntityOfPage")]
    #[serde(default, deserialize_with = "one_or_many")]
    pub main_entity_of_page: Vec<RecipeMainEntityOfPageFieldEnum>,
}
