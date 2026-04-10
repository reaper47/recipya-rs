use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

use crate::enums::{GenderTypeEnum, ItemListOrderTypeEnum, MeasurementMethodEnumEnum};
use crate::{
    AtType, AudioObject, BreadcrumbList, Clip, Comment, CreativeWork, DefinedTerm, DefinedTermSet,
    Distance, Duration, EntryPoint, Enumeration, Event, HowToSection, HowToStep, HowToSupply,
    HowToTool, ImageObject, ItemList, Language, ListItem, Mass, MeasurementTypeEnumeration,
    MediaObject, MonetaryAmount, MusicRecording, Organization, Person, PostalAddress, Product,
    PropertyValue, QualitativeValue, QuantitativeValue, Rating, SizeSpecification, StructuredValue,
    TextObject, Thing, VideoObject, WebContent, WebPage,
};

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum FieldEnum2 {
    ///<https://schema.org/Integer>
    Integer(i32),
    ///<https://schema.org/Text>
    Text(String),
}
impl Default for FieldEnum2 {
    fn default() -> Self {
        Self::Integer(0)
    }
}
///<https://schema.org/clipNumber>
pub type ClipClipNumberFieldEnum = FieldEnum2;
///<https://schema.org/position>
pub type HowToToolPositionFieldEnum = FieldEnum2;
///<https://schema.org/position>
pub type HowToStepPositionFieldEnum = FieldEnum2;
///<https://schema.org/position>
pub type ListItemPositionFieldEnum = FieldEnum2;
///<https://schema.org/position>
pub type HowToSectionPositionFieldEnum = FieldEnum2;
///<https://schema.org/position>
pub type HowToSupplyPositionFieldEnum = FieldEnum2;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum FieldEnum4 {
    ///<https://schema.org/CreativeWork>
    CreativeWork(Box<CreativeWork>),
    ///<https://schema.org/Product>
    Product(Box<Product>),
    ///<https://schema.org/URL>
    URL(String),
}
impl Default for FieldEnum4 {
    fn default() -> Self {
        Self::CreativeWork(CreativeWork::default().into())
    }
}
impl FieldEnum4 {
    pub fn new_creative_work_text(s: &str) -> Self {
        Self::CreativeWork(Box::new(CreativeWork {
            r#type: AtType::CreativeWork.to_opt(),
            text: vec![s.to_string()],
            ..Default::default()
        }))
    }
}

///<https://schema.org/isBasedOnUrl>
pub type ClipIsBasedOnUrlFieldEnum = FieldEnum4;
///<https://schema.org/isBasedOn>
pub type ClipIsBasedOnFieldEnum = FieldEnum4;
///<https://schema.org/isBasedOnUrl>
pub type WebContentIsBasedOnUrlFieldEnum = FieldEnum4;
///<https://schema.org/isBasedOnUrl>
pub type MusicPlaylistIsBasedOnUrlFieldEnum = FieldEnum4;
///<https://schema.org/isBasedOn>
pub type MusicRecordingIsBasedOnFieldEnum = FieldEnum4;
///<https://schema.org/isBasedOn>
pub type WebContentIsBasedOnFieldEnum = FieldEnum4;
///<https://schema.org/isBasedOnUrl>
pub type ReviewIsBasedOnUrlFieldEnum = FieldEnum4;
///<https://schema.org/isBasedOn>
pub type ReviewIsBasedOnFieldEnum = FieldEnum4;
///<https://schema.org/isBasedOnUrl>
pub type WebPageElementIsBasedOnUrlFieldEnum = FieldEnum4;
///<https://schema.org/isBasedOn>
pub type WebPageElementIsBasedOnFieldEnum = FieldEnum4;
///<https://schema.org/isBasedOnUrl>
pub type WebPageIsBasedOnUrlFieldEnum = FieldEnum4;
///<https://schema.org/isBasedOn>
pub type WebPageIsBasedOnFieldEnum = FieldEnum4;
///<https://schema.org/isBasedOnUrl>
pub type AudioObjectIsBasedOnUrlFieldEnum = FieldEnum4;
///<https://schema.org/isBasedOn>
pub type AudioObjectIsBasedOnFieldEnum = FieldEnum4;
///<https://schema.org/isBasedOnUrl>
pub type MediaObjectIsBasedOnUrlFieldEnum = FieldEnum4;
///<https://schema.org/isBasedOnUrl>
pub type MusicCompositionIsBasedOnUrlFieldEnum = FieldEnum4;
///<https://schema.org/isBasedOn>
pub type MusicCompositionIsBasedOnFieldEnum = FieldEnum4;
///<https://schema.org/isBasedOnUrl>
pub type MusicReleaseIsBasedOnUrlFieldEnum = FieldEnum4;
///<https://schema.org/isBasedOnUrl>
pub type ImageObjectIsBasedOnUrlFieldEnum = FieldEnum4;
///<https://schema.org/isBasedOnUrl>
pub type HowToStepIsBasedOnUrlFieldEnum = FieldEnum4;
///<https://schema.org/isBasedOn>
pub type HowToStepIsBasedOnFieldEnum = FieldEnum4;
///<https://schema.org/isBasedOnUrl>
pub type HowToSectionIsBasedOnUrlFieldEnum = FieldEnum4;
///<https://schema.org/isBasedOn>
pub type HowToSectionIsBasedOnFieldEnum = FieldEnum4;
///<https://schema.org/isBasedOnUrl>
pub type RecipeIsBasedOnUrlFieldEnum = FieldEnum4;
///<https://schema.org/isBasedOn>
pub type RecipeIsBasedOnFieldEnum = FieldEnum4;
///<https://schema.org/isBasedOnUrl>
pub type MusicAlbumIsBasedOnUrlFieldEnum = FieldEnum4;
///<https://schema.org/isBasedOn>
pub type MusicAlbumIsBasedOnFieldEnum = FieldEnum4;
///<https://schema.org/isBasedOnUrl>
pub type CreativeWorkIsBasedOnUrlFieldEnum = FieldEnum4;
///<https://schema.org/isBasedOn>
pub type CreativeWorkIsBasedOnFieldEnum = FieldEnum4;
///<https://schema.org/isBasedOnUrl>
pub type HowToIsBasedOnUrlFieldEnum = FieldEnum4;
///<https://schema.org/isBasedOn>
pub type HowToIsBasedOnFieldEnum = FieldEnum4;
///<https://schema.org/isBasedOn>
pub type CommentIsBasedOnFieldEnum = FieldEnum4;
///<https://schema.org/isBasedOn>
pub type VideoObjectIsBasedOnFieldEnum = FieldEnum4;
///<https://schema.org/isBasedOnUrl>
pub type DefinedTermSetIsBasedOnUrlFieldEnum = FieldEnum4;
///<https://schema.org/isBasedOn>
pub type DefinedTermSetIsBasedOnFieldEnum = FieldEnum4;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum FieldEnum5 {
    ///<https://schema.org/DefinedTerm>
    DefinedTerm(DefinedTerm),
    ///<https://schema.org/QuantitativeValue>
    QuantitativeValue(QuantitativeValue),
    ///<https://schema.org/SizeSpecification>
    SizeSpecification(SizeSpecification),
    ///<https://schema.org/Text>
    Text(String),
}
impl Default for FieldEnum5 {
    fn default() -> Self {
        Self::Text(String::new())
    }
}
///<https://schema.org/size>
pub type ProductSizeFieldEnum = FieldEnum5;
///<https://schema.org/size>
pub type MediaObjectSizeFieldEnum = FieldEnum5;
///<https://schema.org/size>
pub type MusicCompositionSizeFieldEnum = FieldEnum5;
///<https://schema.org/size>
pub type ImageObjectSizeFieldEnum = FieldEnum5;
///<https://schema.org/size>
pub type HowToStepSizeFieldEnum = FieldEnum5;
///<https://schema.org/size>
pub type HowToSectionSizeFieldEnum = FieldEnum5;
///<https://schema.org/size>
pub type RecipeSizeFieldEnum = FieldEnum5;
///<https://schema.org/size>
pub type HowToSizeFieldEnum = FieldEnum5;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum FieldEnum6 {
    ///<https://schema.org/Organization>
    Organization(Organization),
    ///<https://schema.org/Person>
    Person(Person),
    Text(String),
}
impl Default for FieldEnum6 {
    fn default() -> Self {
        Self::Person(Person::default())
    }
}
impl FieldEnum6 {
    pub fn is_default(&self) -> bool {
        match self {
            Self::Organization(org) => org == &Organization::default(),
            Self::Person(person) => person == &Person::default(),
            Self::Text(text) => text.is_empty(),
        }
    }
}

///<https://schema.org/creator>
pub type ClipCreatorFieldEnum = FieldEnum6;
///<https://schema.org/author>
pub type ClipAuthorFieldEnum = FieldEnum6;
///<https://schema.org/author>
pub type WebContentAuthorFieldEnum = FieldEnum6;
///<https://schema.org/creator>
pub type MusicPlaylistCreatorFieldEnum = FieldEnum6;
///<https://schema.org/author>
pub type MusicPlaylistAuthorFieldEnum = FieldEnum6;
///<https://schema.org/creator>
pub type MusicRecordingCreatorFieldEnum = FieldEnum6;
///<https://schema.org/sdPublisher>
pub type MusicRecordingSdPublisherFieldEnum = FieldEnum6;
///<https://schema.org/author>
pub type MusicRecordingAuthorFieldEnum = FieldEnum6;
///<https://schema.org/contributor>
pub type ReviewContributorFieldEnum = FieldEnum6;
///<https://schema.org/publisher>
pub type ReviewPublisherFieldEnum = FieldEnum6;
///<https://schema.org/creator>
pub type ReviewCreatorFieldEnum = FieldEnum6;
///<https://schema.org/sdPublisher>
pub type ReviewSdPublisherFieldEnum = FieldEnum6;
///<https://schema.org/author>
pub type ReviewAuthorFieldEnum = FieldEnum6;
///<https://schema.org/translator>
pub type ReviewTranslatorFieldEnum = FieldEnum6;
///<https://schema.org/sdPublisher>
pub type WebPageElementSdPublisherFieldEnum = FieldEnum6;
///<https://schema.org/author>
pub type WebPageElementAuthorFieldEnum = FieldEnum6;
///<https://schema.org/reviewedBy>
pub type WebPageReviewedByFieldEnum = FieldEnum6;
///<https://schema.org/maintainer>
pub type WebPageMaintainerFieldEnum = FieldEnum6;
///<https://schema.org/contributor>
pub type WebPageContributorFieldEnum = FieldEnum6;
///<https://schema.org/creator>
pub type WebPageCreatorFieldEnum = FieldEnum6;
///<https://schema.org/author>
pub type WebPageAuthorFieldEnum = FieldEnum6;
///<https://schema.org/translator>
pub type WebPageTranslatorFieldEnum = FieldEnum6;
///<https://schema.org/creator>
pub type AudioObjectCreatorFieldEnum = FieldEnum6;
///<https://schema.org/sdPublisher>
pub type AudioObjectSdPublisherFieldEnum = FieldEnum6;
///<https://schema.org/author>
pub type AudioObjectAuthorFieldEnum = FieldEnum6;
///<https://schema.org/author>
pub type RatingAuthorFieldEnum = FieldEnum6;
///<https://schema.org/maintainer>
pub type MediaObjectMaintainerFieldEnum = FieldEnum6;
///<https://schema.org/contributor>
pub type MediaObjectContributorFieldEnum = FieldEnum6;
///<https://schema.org/publisher>
pub type MediaObjectPublisherFieldEnum = FieldEnum6;
///<https://schema.org/provider>
pub type MediaObjectProviderFieldEnum = FieldEnum6;
///<https://schema.org/creator>
pub type MediaObjectCreatorFieldEnum = FieldEnum6;
///<https://schema.org/sdPublisher>
pub type MediaObjectSdPublisherFieldEnum = FieldEnum6;
///<https://schema.org/producer>
pub type MediaObjectProducerFieldEnum = FieldEnum6;
///<https://schema.org/author>
pub type MediaObjectAuthorFieldEnum = FieldEnum6;
///<https://schema.org/translator>
pub type MediaObjectTranslatorFieldEnum = FieldEnum6;
///<https://schema.org/composer>
pub type MusicCompositionComposerFieldEnum = FieldEnum6;
///<https://schema.org/maintainer>
pub type MusicCompositionMaintainerFieldEnum = FieldEnum6;
///<https://schema.org/publisher>
pub type MusicCompositionPublisherFieldEnum = FieldEnum6;
///<https://schema.org/creator>
pub type MusicCompositionCreatorFieldEnum = FieldEnum6;
///<https://schema.org/sdPublisher>
pub type MusicCompositionSdPublisherFieldEnum = FieldEnum6;
///<https://schema.org/author>
pub type MusicCompositionAuthorFieldEnum = FieldEnum6;
///<https://schema.org/agent>
pub type ActionAgentFieldEnum = FieldEnum6;
///<https://schema.org/provider>
pub type ActionProviderFieldEnum = FieldEnum6;
///<https://schema.org/participant>
pub type ActionParticipantFieldEnum = FieldEnum6;
///<https://schema.org/creditedTo>
pub type MusicReleaseCreditedToFieldEnum = FieldEnum6;
///<https://schema.org/publisher>
pub type MusicReleasePublisherFieldEnum = FieldEnum6;
///<https://schema.org/creator>
pub type MusicReleaseCreatorFieldEnum = FieldEnum6;
///<https://schema.org/sdPublisher>
pub type MusicReleaseSdPublisherFieldEnum = FieldEnum6;
///<https://schema.org/author>
pub type MusicReleaseAuthorFieldEnum = FieldEnum6;
///<https://schema.org/author>
pub type AggregateRatingAuthorFieldEnum = FieldEnum6;
///<https://schema.org/contributor>
pub type ImageObjectContributorFieldEnum = FieldEnum6;
///<https://schema.org/publisher>
pub type ImageObjectPublisherFieldEnum = FieldEnum6;
///<https://schema.org/provider>
pub type ImageObjectProviderFieldEnum = FieldEnum6;
///<https://schema.org/creator>
pub type ImageObjectCreatorFieldEnum = FieldEnum6;
///<https://schema.org/sdPublisher>
pub type ImageObjectSdPublisherFieldEnum = FieldEnum6;
///<https://schema.org/author>
pub type ImageObjectAuthorFieldEnum = FieldEnum6;
///<https://schema.org/contributor>
pub type HowToStepContributorFieldEnum = FieldEnum6;
///<https://schema.org/creator>
pub type HowToStepCreatorFieldEnum = FieldEnum6;
///<https://schema.org/author>
pub type HowToStepAuthorFieldEnum = FieldEnum6;
///<https://schema.org/founder>
pub type OrganizationFounderFieldEnum = FieldEnum6;
///<https://schema.org/maintainer>
pub type RecipeMaintainerFieldEnum = FieldEnum6;
///<https://schema.org/contributor>
pub type RecipeContributorFieldEnum = FieldEnum6;
///<https://schema.org/publisher>
pub type RecipePublisherFieldEnum = FieldEnum6;
///<https://schema.org/provider>
pub type RecipeProviderFieldEnum = FieldEnum6;
///<https://schema.org/creator>
pub type RecipeCreatorFieldEnum = FieldEnum6;
///<https://schema.org/sdPublisher>
pub type RecipeSdPublisherFieldEnum = FieldEnum6;
///<https://schema.org/producer>
pub type RecipeProducerFieldEnum = FieldEnum6;
///<https://schema.org/author>
pub type RecipeAuthorFieldEnum = FieldEnum6;

impl RecipeAuthorFieldEnum {
    /// Creates a new person.
    pub fn new_person(name: &str) -> Self {
        Self::Person(Person {
            r#type: AtType::Person.to_opt(),
            name: vec![name.to_string()],
            ..Default::default()
        })
    }

    /// Creates a new organization.
    pub fn new_org(name: &str) -> Self {
        Self::Organization(Organization {
            r#type: AtType::Person.to_opt(),
            name: vec![name.to_string()],
            ..Default::default()
        })
    }
}
///<https://schema.org/translator>
pub type RecipeTranslatorFieldEnum = FieldEnum6;
///<https://schema.org/publisher>
pub type MusicAlbumPublisherFieldEnum = FieldEnum6;
///<https://schema.org/creator>
pub type MusicAlbumCreatorFieldEnum = FieldEnum6;
///<https://schema.org/sdPublisher>
pub type MusicAlbumSdPublisherFieldEnum = FieldEnum6;
///<https://schema.org/author>
pub type MusicAlbumAuthorFieldEnum = FieldEnum6;
///<https://schema.org/maintainer>
pub type CreativeWorkMaintainerFieldEnum = FieldEnum6;
///<https://schema.org/contributor>
pub type CreativeWorkContributorFieldEnum = FieldEnum6;
///<https://schema.org/author>
pub type CreativeWorkAuthorFieldEnum = FieldEnum6;
///<https://schema.org/translator>
pub type CreativeWorkTranslatorFieldEnum = FieldEnum6;
///<https://schema.org/contributor>
pub type TextObjectContributorFieldEnum = FieldEnum6;
///<https://schema.org/creator>
pub type TextObjectCreatorFieldEnum = FieldEnum6;
///<https://schema.org/author>
pub type TextObjectAuthorFieldEnum = FieldEnum6;
///<https://schema.org/translator>
pub type TextObjectTranslatorFieldEnum = FieldEnum6;
///<https://schema.org/author>
pub type HowToAuthorFieldEnum = FieldEnum6;
///<https://schema.org/translator>
pub type HowToTranslatorFieldEnum = FieldEnum6;
///<https://schema.org/contributor>
pub type CommentContributorFieldEnum = FieldEnum6;
///<https://schema.org/creator>
pub type CommentCreatorFieldEnum = FieldEnum6;
///<https://schema.org/author>
pub type CommentAuthorFieldEnum = FieldEnum6;
///<https://schema.org/creator>
pub type VideoObjectCreatorFieldEnum = FieldEnum6;
///<https://schema.org/sdPublisher>
pub type VideoObjectSdPublisherFieldEnum = FieldEnum6;
///<https://schema.org/organizer>
pub type EventOrganizerFieldEnum = FieldEnum6;
///<https://schema.org/contributor>
pub type DefinedTermSetContributorFieldEnum = FieldEnum6;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum FieldEnum7 {
    ///<https://schema.org/CreativeWork>
    CreativeWork(Box<CreativeWork>),
    ///<https://schema.org/URL>
    URL(String),
}
impl Default for FieldEnum7 {
    fn default() -> Self {
        Self::CreativeWork(CreativeWork::default().into())
    }
}
///<https://schema.org/acquireLicensePage>
pub type MusicRecordingAcquireLicensePageFieldEnum = FieldEnum7;
///<https://schema.org/publishingPrinciples>
pub type WebPagePublishingPrinciplesFieldEnum = FieldEnum7;
///<https://schema.org/isPartOf>
pub type AudioObjectIsPartOfFieldEnum = FieldEnum7;
///<https://schema.org/license>
pub type MediaObjectLicenseFieldEnum = FieldEnum7;
///<https://schema.org/license>
pub type MusicCompositionLicenseFieldEnum = FieldEnum7;
///<https://schema.org/usageInfo>
pub type MusicCompositionUsageInfoFieldEnum = FieldEnum7;
///<https://schema.org/isPartOf>
pub type ImageObjectIsPartOfFieldEnum = FieldEnum7;
///<https://schema.org/usageInfo>
pub type HowToSectionUsageInfoFieldEnum = FieldEnum7;
///<https://schema.org/license>
pub type RecipeLicenseFieldEnum = FieldEnum7;
///<https://schema.org/isPartOf>
pub type RecipeIsPartOfFieldEnum = FieldEnum7;
///<https://schema.org/isPartOf>
pub type MusicAlbumIsPartOfFieldEnum = FieldEnum7;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum FieldEnum8 {
    ///<https://schema.org/Number>
    Number(f32),
    ///<https://schema.org/Text>
    Text(String),
}
impl Default for FieldEnum8 {
    fn default() -> Self {
        Self::Number(0.)
    }
}
///<https://schema.org/worstRating>
pub type RatingWorstRatingFieldEnum = FieldEnum8;
///<https://schema.org/bestRating>
pub type RatingBestRatingFieldEnum = FieldEnum8;
///<https://schema.org/ratingValue>
pub type RatingRatingValueFieldEnum = FieldEnum8;
///<https://schema.org/latitude>
pub type PlaceLatitudeFieldEnum = FieldEnum8;
///<https://schema.org/longitude>
pub type PlaceLongitudeFieldEnum = FieldEnum8;
///<https://schema.org/worstRating>
pub type AggregateRatingWorstRatingFieldEnum = FieldEnum8;
///<https://schema.org/bestRating>
pub type AggregateRatingBestRatingFieldEnum = FieldEnum8;
///<https://schema.org/ratingValue>
pub type AggregateRatingRatingValueFieldEnum = FieldEnum8;
///<https://schema.org/latitude>
pub type CountryLatitudeFieldEnum = FieldEnum8;
///<https://schema.org/longitude>
pub type CountryLongitudeFieldEnum = FieldEnum8;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum FieldEnum9 {
    ///<https://schema.org/DefinedTerm>
    DefinedTerm(Box<DefinedTerm>),
    ///<https://schema.org/Text>
    ///<https://schema.org/URL>
    TextOrURL(String),
}
impl Default for FieldEnum9 {
    fn default() -> Self {
        Self::TextOrURL(String::new())
    }
}
///<https://schema.org/keywords>
pub type ClipKeywordsFieldEnum = FieldEnum9;
///<https://schema.org/keywords>
pub type MusicPlaylistKeywordsFieldEnum = FieldEnum9;
///<https://schema.org/keywords>
pub type MusicRecordingKeywordsFieldEnum = FieldEnum9;
///<https://schema.org/keywords>
pub type ProductKeywordsFieldEnum = FieldEnum9;
///<https://schema.org/keywords>
pub type ReviewKeywordsFieldEnum = FieldEnum9;
///<https://schema.org/keywords>
pub type WebPageElementKeywordsFieldEnum = FieldEnum9;
///<https://schema.org/keywords>
pub type WebPageKeywordsFieldEnum = FieldEnum9;
///<https://schema.org/keywords>
pub type AudioObjectKeywordsFieldEnum = FieldEnum9;
///<https://schema.org/keywords>
pub type MediaObjectKeywordsFieldEnum = FieldEnum9;
///<https://schema.org/keywords>
pub type MusicCompositionKeywordsFieldEnum = FieldEnum9;
///<https://schema.org/keywords>
pub type PlaceKeywordsFieldEnum = FieldEnum9;
///<https://schema.org/keywords>
pub type MusicReleaseKeywordsFieldEnum = FieldEnum9;
///<https://schema.org/keywords>
pub type CountryKeywordsFieldEnum = FieldEnum9;
///<https://schema.org/keywords>
pub type ImageObjectKeywordsFieldEnum = FieldEnum9;
///<https://schema.org/keywords>
pub type HowToStepKeywordsFieldEnum = FieldEnum9;
///<https://schema.org/keywords>
pub type HowToSectionKeywordsFieldEnum = FieldEnum9;
///<https://schema.org/keywords>
pub type OrganizationKeywordsFieldEnum = FieldEnum9;
///<https://schema.org/keywords>
pub type RecipeKeywordsFieldEnum = FieldEnum9;
///<https://schema.org/keywords>
pub type MusicAlbumKeywordsFieldEnum = FieldEnum9;
///<https://schema.org/keywords>
pub type CreativeWorkKeywordsFieldEnum = FieldEnum9;
///<https://schema.org/keywords>
pub type TextObjectKeywordsFieldEnum = FieldEnum9;
///<https://schema.org/keywords>
pub type HowToKeywordsFieldEnum = FieldEnum9;
///<https://schema.org/keywords>
pub type WebContentKeywordsFieldEnum = FieldEnum9;
///<https://schema.org/keywords>
pub type CommentKeywordsFieldEnum = FieldEnum9;
///<https://schema.org/keywords>
pub type VideoObjectKeywordsFieldEnum = FieldEnum9;
///<https://schema.org/keywords>
pub type EventKeywordsFieldEnum = FieldEnum9;
///<https://schema.org/keywords>
pub type DefinedTermSetKeywordsFieldEnum = FieldEnum9;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum FieldEnum11 {
    ///<https://schema.org/Product>
    Product(Box<Product>),
    ///<https://schema.org/Text>
    ///<https://schema.org/URL>
    TextOrURL(String),
}
impl Default for FieldEnum11 {
    fn default() -> Self {
        Self::TextOrURL(String::default())
    }
}
///<https://schema.org/material>
pub type ProductMaterialFieldEnum = FieldEnum11;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum FieldEnum12 {
    ///<https://schema.org/DefinedTerm>
    DefinedTerm(Box<DefinedTerm>),
    ///<https://schema.org/Text>
    Text(String),
}
impl Default for FieldEnum12 {
    fn default() -> Self {
        Self::Text(String::new())
    }
}
///<https://schema.org/jobTitle>
pub type PersonJobTitleFieldEnum = FieldEnum12;
///<https://schema.org/skills>
pub type PersonSkillsFieldEnum = FieldEnum12;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum FieldEnum13 {
    ///<https://schema.org/URL>
    URL(String),
    ///<https://schema.org/WebPage>
    WebPage(Box<WebPage>),
}
impl Default for FieldEnum13 {
    fn default() -> Self {
        Self::WebPage(WebPage::default().into())
    }
}
///<https://schema.org/archivedAt>
pub type ClipArchivedAtFieldEnum = FieldEnum13;
///<https://schema.org/archivedAt>
pub type MusicRecordingArchivedAtFieldEnum = FieldEnum13;
///<https://schema.org/archivedAt>
pub type ReviewArchivedAtFieldEnum = FieldEnum13;
///<https://schema.org/archivedAt>
pub type WebContentArchivedAtFieldEnum = FieldEnum13;
///<https://schema.org/archivedAt>
pub type WebPageElementArchivedAtFieldEnum = FieldEnum13;
///<https://schema.org/archivedAt>
pub type WebPageArchivedAtFieldEnum = FieldEnum13;
///<https://schema.org/archivedAt>
pub type AudioObjectArchivedAtFieldEnum = FieldEnum13;
///<https://schema.org/archivedAt>
pub type MediaObjectArchivedAtFieldEnum = FieldEnum13;
///<https://schema.org/archivedAt>
pub type MusicCompositionArchivedAtFieldEnum = FieldEnum13;
///<https://schema.org/archivedAt>
pub type ImageObjectArchivedAtFieldEnum = FieldEnum13;
///<https://schema.org/archivedAt>
pub type RecipeArchivedAtFieldEnum = FieldEnum13;
///<https://schema.org/archivedAt>
pub type MusicAlbumArchivedAtFieldEnum = FieldEnum13;
///<https://schema.org/archivedAt>
pub type TextObjectArchivedAtFieldEnum = FieldEnum13;
///<https://schema.org/archivedAt>
pub type HowToArchivedAtFieldEnum = FieldEnum13;
///<https://schema.org/archivedAt>
pub type VideoObjectArchivedAtFieldEnum = FieldEnum13;
///<https://schema.org/archivedAt>
pub type DefinedTermSetArchivedAtFieldEnum = FieldEnum13;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum FieldEnum14 {
    ///<https://schema.org/Rating>
    Rating(Box<Rating>),
    ///<https://schema.org/Text>
    Text(String),
}
impl Default for FieldEnum14 {
    fn default() -> Self {
        Self::Text(String::new())
    }
}
///<https://schema.org/contentRating>
pub type MusicRecordingContentRatingFieldEnum = FieldEnum14;
///<https://schema.org/contentRating>
pub type ReviewContentRatingFieldEnum = FieldEnum14;
///<https://schema.org/contentRating>
pub type AudioObjectContentRatingFieldEnum = FieldEnum14;
///<https://schema.org/contentRating>
pub type MediaObjectContentRatingFieldEnum = FieldEnum14;
///<https://schema.org/contentRating>
pub type RecipeContentRatingFieldEnum = FieldEnum14;

impl RecipeContentRatingFieldEnum {
    /// Creates a new `RecipeContentRatingFieldEnum` instance with the given rating value.
    pub fn new_rating(rating: f32) -> Self {
        Self::Rating(Box::new(Rating::new(rating)))
    }
}

///<https://schema.org/contentRating>
pub type TextObjectContentRatingFieldEnum = FieldEnum14;
///<https://schema.org/contentRating>
pub type HowToContentRatingFieldEnum = FieldEnum14;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum FieldEnum16 {
    ///<https://schema.org/Language>
    Language(Box<Language>),
    ///<https://schema.org/Text>
    Text(String),
}
impl Default for FieldEnum16 {
    fn default() -> Self {
        Self::Text(String::new())
    }
}
///<https://schema.org/inLanguage>
pub type ClipInLanguageFieldEnum = FieldEnum16;
///<https://schema.org/inLanguage>
pub type MusicRecordingInLanguageFieldEnum = FieldEnum16;
///<https://schema.org/inLanguage>
pub type WebContentInLanguageFieldEnum = FieldEnum16;
///<https://schema.org/inLanguage>
pub type ReviewInLanguageFieldEnum = FieldEnum16;
///<https://schema.org/inLanguage>
pub type WebPageElementInLanguageFieldEnum = FieldEnum16;
///<https://schema.org/knowsLanguage>
pub type PersonKnowsLanguageFieldEnum = FieldEnum16;
///<https://schema.org/inLanguage>
pub type WebPageInLanguageFieldEnum = FieldEnum16;
///<https://schema.org/inLanguage>
pub type AudioObjectInLanguageFieldEnum = FieldEnum16;
///<https://schema.org/inLanguage>
pub type MediaObjectInLanguageFieldEnum = FieldEnum16;
///<https://schema.org/inLanguage>
pub type HowToStepInLanguageFieldEnum = FieldEnum16;
///<https://schema.org/inLanguage>
pub type HowToSectionInLanguageFieldEnum = FieldEnum16;
///<https://schema.org/inLanguage>
pub type RecipeInLanguageFieldEnum = FieldEnum16;
///<https://schema.org/inLanguage>
pub type CreativeWorkInLanguageFieldEnum = FieldEnum16;
///<https://schema.org/inLanguage>
pub type TextObjectInLanguageFieldEnum = FieldEnum16;
///<https://schema.org/inLanguage>
pub type HowToInLanguageFieldEnum = FieldEnum16;
///<https://schema.org/inLanguage>
pub type CommentInLanguageFieldEnum = FieldEnum16;
///<https://schema.org/inLanguage>
pub type VideoObjectInLanguageFieldEnum = FieldEnum16;
///<https://schema.org/inLanguage>
pub type EventInLanguageFieldEnum = FieldEnum16;
///<https://schema.org/availableLanguage>
pub type PostalAddressAvailableLanguageFieldEnum = FieldEnum16;
///<https://schema.org/inLanguage>
pub type DefinedTermSetInLanguageFieldEnum = FieldEnum16;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum FieldEnum17 {
    ///<https://schema.org/AudioObject>
    AudioObject(AudioObject),
    ///<https://schema.org/Clip>
    Clip(Clip),
    ///<https://schema.org/MusicRecording>
    MusicRecording(MusicRecording),
}
impl Default for FieldEnum17 {
    fn default() -> Self {
        Self::AudioObject(AudioObject::default())
    }
}
///<https://schema.org/audio>
pub type ClipAudioFieldEnum = FieldEnum17;
///<https://schema.org/audio>
pub type MusicRecordingAudioFieldEnum = FieldEnum17;
///<https://schema.org/audio>
pub type ReviewAudioFieldEnum = FieldEnum17;
///<https://schema.org/audio>
pub type AudioObjectAudioFieldEnum = FieldEnum17;
///<https://schema.org/audio>
pub type MediaObjectAudioFieldEnum = FieldEnum17;
///<https://schema.org/audio>
pub type MusicReleaseAudioFieldEnum = FieldEnum17;
///<https://schema.org/audio>
pub type RecipeAudioFieldEnum = FieldEnum17;
///<https://schema.org/audio>
pub type MusicAlbumAudioFieldEnum = FieldEnum17;
///<https://schema.org/audio>
pub type HowToAudioFieldEnum = FieldEnum17;
///<https://schema.org/audio>
pub type CommentAudioFieldEnum = FieldEnum17;
///<https://schema.org/audio>
pub type VideoObjectAudioFieldEnum = FieldEnum17;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum FieldEnum18 {
    ///<https://schema.org/CreativeWork>
    CreativeWork(Box<CreativeWork>),
    ///<https://schema.org/Text>
    Text(String),
}
impl Default for FieldEnum18 {
    fn default() -> Self {
        Self::Text(String::new())
    }
}
///<https://schema.org/citation>
pub type WebPageCitationFieldEnum = FieldEnum18;
///<https://schema.org/citation>
pub type AudioObjectCitationFieldEnum = FieldEnum18;
///<https://schema.org/citation>
pub type MediaObjectCitationFieldEnum = FieldEnum18;
///<https://schema.org/citation>
pub type ImageObjectCitationFieldEnum = FieldEnum18;
///<https://schema.org/citation>
pub type RecipeCitationFieldEnum = FieldEnum18;
///<https://schema.org/citation>
pub type MusicAlbumCitationFieldEnum = FieldEnum18;
///<https://schema.org/citation>
pub type CreativeWorkCitationFieldEnum = FieldEnum18;
///<https://schema.org/citation>
pub type TextObjectCitationFieldEnum = FieldEnum18;
///<https://schema.org/citation>
pub type CommentCitationFieldEnum = FieldEnum18;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum FieldEnum19 {
    ///<https://schema.org/Clip>
    Clip(Box<Clip>),
    ///<https://schema.org/VideoObject>
    VideoObject(Box<VideoObject>),
}
impl Default for FieldEnum19 {
    fn default() -> Self {
        Self::Clip(Box::default())
    }
}
///<https://schema.org/video>
pub type ClipVideoFieldEnum = FieldEnum19;
///<https://schema.org/video>
pub type ReviewVideoFieldEnum = FieldEnum19;
///<https://schema.org/video>
pub type WebPageVideoFieldEnum = FieldEnum19;
///<https://schema.org/video>
pub type MediaObjectVideoFieldEnum = FieldEnum19;
///<https://schema.org/video>
pub type HowToStepVideoFieldEnum = FieldEnum19;
///<https://schema.org/video>
pub type HowToSectionVideoFieldEnum = FieldEnum19;
///<https://schema.org/video>
pub type RecipeVideoFieldEnum = FieldEnum19;
///<https://schema.org/video>
pub type CreativeWorkVideoFieldEnum = FieldEnum19;
///<https://schema.org/video>
pub type TextObjectVideoFieldEnum = FieldEnum19;
///<https://schema.org/video>
pub type HowToVideoFieldEnum = FieldEnum19;
///<https://schema.org/video>
pub type CommentVideoFieldEnum = FieldEnum19;
///<https://schema.org/video>
pub type VideoObjectVideoFieldEnum = FieldEnum19;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum FieldEnum20 {
    ///<https://schema.org/QuantitativeValue>
    QuantitativeValue(Box<QuantitativeValue>),
    Number(f64),
    ///<https://schema.org/Text>
    Text(String),
}
impl Default for FieldEnum20 {
    fn default() -> Self {
        Self::Text(String::new())
    }
}
impl FieldEnum20 {
    /// Creates a new `QuantitativeValue` field.
    pub fn new_quantitative_value(value: f64) -> Self {
        Self::QuantitativeValue(Box::new(QuantitativeValue::new(value)))
    }

    pub fn to_i16(&self) -> Option<i16> {
        match self {
            Self::QuantitativeValue(q) => q.value.first().map(|q| match q {
                QuantitativeValueValueFieldEnum::BooleanEnumOrText(s) => {
                    s.parse::<i16>().ok().unwrap_or_default()
                }
                QuantitativeValueValueFieldEnum::Number(n) => float_to_i16_safe(*n),
                QuantitativeValueValueFieldEnum::StructuredValue(v) => v
                    .name
                    .first()
                    .map(|v| v.parse::<i16>().unwrap_or_default())
                    .unwrap_or_default(),
                QuantitativeValueValueFieldEnum::QuantitativeValue(q) => q.to_number(),
            }),
            Self::Number(n) => Some(float_to_i16_safe(*n)),
            Self::Text(s) => s
                .split_whitespace()
                .find_map(|part| part.parse::<i16>().ok()),
        }
    }
}
///<https://schema.org/recipeYield>
pub type RecipeRecipeYieldFieldEnum = FieldEnum20;
///<https://schema.org/yield>
pub type RecipeYieldFieldEnum = FieldEnum20;
///<https://schema.org/yield>
pub type HowToYieldFieldEnum = FieldEnum20;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum FieldEnum21 {
    ///<https://schema.org/PropertyValue>
    PropertyValue(Box<PropertyValue>),
    ///<https://schema.org/Text>
    ///<https://schema.org/URL>
    TextOrURL(String),
}
impl Default for FieldEnum21 {
    fn default() -> Self {
        Self::TextOrURL(String::new())
    }
}
///<https://schema.org/identifier>
pub type RatingIdentifierFieldEnum = FieldEnum21;
///<https://schema.org/identifier>
pub type ActionIdentifierFieldEnum = FieldEnum21;
///<https://schema.org/identifier>
pub type MeasurementTypeEnumerationIdentifierFieldEnum = FieldEnum21;
///<https://schema.org/identifier>
pub type StructuredValueIdentifierFieldEnum = FieldEnum21;
///<https://schema.org/identifier>
pub type DefinedTermIdentifierFieldEnum = FieldEnum21;
///<https://schema.org/identifier>
pub type EnumerationIdentifierFieldEnum = FieldEnum21;
///<https://schema.org/identifier>
pub type LanguageIdentifierFieldEnum = FieldEnum21;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum FieldEnum22 {
    ///<https://schema.org/ImageObject>
    ImageObject(Box<ImageObject>),
    ///<https://schema.org/URL>
    URL(String),
}
impl Default for FieldEnum22 {
    fn default() -> Self {
        Self::ImageObject(ImageObject::default().into())
    }
}
///<https://schema.org/image>
pub type ClipImageFieldEnum = FieldEnum22;
///<https://schema.org/image>
pub type MusicPlaylistImageFieldEnum = FieldEnum22;
///<https://schema.org/image>
pub type HowToToolImageFieldEnum = FieldEnum22;
///<https://schema.org/image>
pub type MusicRecordingImageFieldEnum = FieldEnum22;
///<https://schema.org/image>
pub type WebContentImageFieldEnum = FieldEnum22;
///<https://schema.org/colorSwatch>
pub type ProductColorSwatchFieldEnum = FieldEnum22;
///<https://schema.org/logo>
pub type ProductLogoFieldEnum = FieldEnum22;
///<https://schema.org/image>
pub type ProductImageFieldEnum = FieldEnum22;
///<https://schema.org/image>
pub type DefinedTermSetImageFieldEnum = FieldEnum22;
///<https://schema.org/image>
pub type ReviewImageFieldEnum = FieldEnum22;
///<https://schema.org/image>
pub type ThingImageFieldEnum = FieldEnum22;
///<https://schema.org/image>
pub type WebPageElementImageFieldEnum = FieldEnum22;
///<https://schema.org/image>
pub type PersonImageFieldEnum = FieldEnum22;
///<https://schema.org/image>
pub type WebPageImageFieldEnum = FieldEnum22;
///<https://schema.org/image>
pub type AudioObjectImageFieldEnum = FieldEnum22;
///<https://schema.org/image>
pub type RatingImageFieldEnum = FieldEnum22;
///<https://schema.org/image>
pub type MediaObjectImageFieldEnum = FieldEnum22;
///<https://schema.org/image>
pub type MusicCompositionImageFieldEnum = FieldEnum22;
///<https://schema.org/image>
pub type ActionImageFieldEnum = FieldEnum22;
///<https://schema.org/logo>
pub type PlaceLogoFieldEnum = FieldEnum22;
///<https://schema.org/image>
pub type PlaceImageFieldEnum = FieldEnum22;
///<https://schema.org/image>
pub type ItemListImageFieldEnum = FieldEnum22;
///<https://schema.org/image>
pub type MassImageFieldEnum = FieldEnum22;
///<https://schema.org/image>
pub type MusicReleaseImageFieldEnum = FieldEnum22;
///<https://schema.org/image>
pub type AggregateRatingImageFieldEnum = FieldEnum22;
///<https://schema.org/image>
pub type CountryImageFieldEnum = FieldEnum22;
///<https://schema.org/image>
pub type ImageObjectImageFieldEnum = FieldEnum22;
///<https://schema.org/image>
pub type HowToStepImageFieldEnum = FieldEnum22;
///<https://schema.org/image>
pub type ListItemImageFieldEnum = FieldEnum22;
///<https://schema.org/image>
pub type HowToSectionImageFieldEnum = FieldEnum22;
///<https://schema.org/image>
pub type DefinedTermImageFieldEnum = FieldEnum22;
///<https://schema.org/logo>
pub type OrganizationLogoFieldEnum = FieldEnum22;
///<https://schema.org/image>
pub type OrganizationImageFieldEnum = FieldEnum22;
///<https://schema.org/image>
pub type RecipeImageFieldEnum = FieldEnum22;
///<https://schema.org/image>
pub type MusicAlbumImageFieldEnum = FieldEnum22;
///<https://schema.org/image>
pub type CreativeWorkImageFieldEnum = FieldEnum22;
///<https://schema.org/image>
pub type TextObjectImageFieldEnum = FieldEnum22;
///<https://schema.org/image>
pub type LanguageImageFieldEnum = FieldEnum22;
///<https://schema.org/image>
pub type HowToImageFieldEnum = FieldEnum22;
///<https://schema.org/image>
pub type CommentImageFieldEnum = FieldEnum22;
///<https://schema.org/image>
pub type PropertyValueImageFieldEnum = FieldEnum22;
///<https://schema.org/image>
pub type VideoObjectImageFieldEnum = FieldEnum22;
///<https://schema.org/image>
pub type EventImageFieldEnum = FieldEnum22;
///<https://schema.org/image>
pub type HowToSupplyImageFieldEnum = FieldEnum22;
///<https://schema.org/image>
pub type EntryPointImageFieldEnum = FieldEnum22;
///<https://schema.org/image>
pub type PostalAddressImageFieldEnum = FieldEnum22;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum FieldEnum23 {
    ///<https://schema.org/Text>
    Text(String),
    ///<https://schema.org/TextObject>
    TextObject(Box<TextObject>),
}
impl Default for FieldEnum23 {
    fn default() -> Self {
        Self::Text(String::new())
    }
}
/// [Schema Description](https://schema.org/description)
pub type ClipDescriptionFieldEnum = FieldEnum23;
/// [Schema Description](https://schema.org/description)
pub type WebContentDescriptionFieldEnum = FieldEnum23;
/// [Schema Description](https://schema.org/description)
pub type MeasurementTypeEnumerationDescriptionFieldEnum = FieldEnum23;
/// [Schema Description](https://schema.org/description)
pub type StructuredValueDescriptionFieldEnum = FieldEnum23;
/// [Schema Description](https://schema.org/description)
pub type MusicPlaylistDescriptionFieldEnum = FieldEnum23;
/// [Schema Description](https://schema.org/description)
pub type EnumerationDescriptionFieldEnum = FieldEnum23;
/// [Schema Description](https://schema.org/description)
pub type BreadcrumbListDescriptionFieldEnum = FieldEnum23;
/// [Schema Description](https://schema.org/description)
pub type HowToToolDescriptionFieldEnum = FieldEnum23;
/// [Schema Description](https://schema.org/description)
pub type QuantitativeValueDescriptionFieldEnum = FieldEnum23;
/// [Schema Description](https://schema.org/description)
pub type DistanceDescriptionFieldEnum = FieldEnum23;
/// [Schema Description](https://schema.org/description)
pub type MusicRecordingDescriptionFieldEnum = FieldEnum23;
/// [Schema Description](https://schema.org/description)
pub type ProductDescriptionFieldEnum = FieldEnum23;
/// [Schema Description](https://schema.org/description)
pub type ReviewDescriptionFieldEnum = FieldEnum23;
/// [Schema Description](https://schema.org/description)
pub type DefinedTermSetDescriptionFieldEnum = FieldEnum23;
/// [Schema Description](https://schema.org/description)
pub type ThingDescriptionFieldEnum = FieldEnum23;
/// [Schema Description](https://schema.org/description)
pub type WebPageElementDescriptionFieldEnum = FieldEnum23;
/// [Schema Description](https://schema.org/description)
pub type PersonDescriptionFieldEnum = FieldEnum23;
/// [Schema Description](https://schema.org/description)
pub type WebPageDescriptionFieldEnum = FieldEnum23;
/// [Schema Description](https://schema.org/description)
pub type AudioObjectDescriptionFieldEnum = FieldEnum23;
/// [Schema Description](https://schema.org/description)
pub type RatingDescriptionFieldEnum = FieldEnum23;
/// [Schema Description](https://schema.org/description)
pub type MediaObjectDescriptionFieldEnum = FieldEnum23;
/// [Schema Description](https://schema.org/description)
pub type MusicCompositionDescriptionFieldEnum = FieldEnum23;
/// [Schema Description](https://schema.org/description)
pub type ActionDescriptionFieldEnum = FieldEnum23;
/// [Schema Description](https://schema.org/description)
pub type SizeSpecificationDescriptionFieldEnum = FieldEnum23;
/// [Schema Description](https://schema.org/description)
pub type PlaceDescriptionFieldEnum = FieldEnum23;
/// [Schema Description](https://schema.org/description)
pub type ItemListDescriptionFieldEnum = FieldEnum23;
/// [Schema Description](https://schema.org/description)
pub type MassDescriptionFieldEnum = FieldEnum23;
/// [Schema Description](https://schema.org/description)
pub type InteractionCounterDescriptionFieldEnum = FieldEnum23;
/// [Schema Description](https://schema.org/description)
pub type MusicReleaseDescriptionFieldEnum = FieldEnum23;
/// [Schema Description](https://schema.org/description)
pub type AggregateRatingDescriptionFieldEnum = FieldEnum23;
/// [Schema Description](https://schema.org/description)
pub type CountryDescriptionFieldEnum = FieldEnum23;
/// [Schema Description](https://schema.org/description)
pub type QualitativeValueDescriptionFieldEnum = FieldEnum23;
/// [Schema Description](https://schema.org/description)
pub type ImageObjectDescriptionFieldEnum = FieldEnum23;
/// [Schema Description](https://schema.org/description)
pub type HowToStepDescriptionFieldEnum = FieldEnum23;
/// [Schema Description](https://schema.org/description)
pub type ListItemDescriptionFieldEnum = FieldEnum23;
/// [Schema Description](https://schema.org/description)
pub type HowToSectionDescriptionFieldEnum = FieldEnum23;
/// [Schema Description](https://schema.org/description)
pub type DefinedTermDescriptionFieldEnum = FieldEnum23;
/// [Schema Description](https://schema.org/description)
pub type OrganizationDescriptionFieldEnum = FieldEnum23;
/// [Schema Description](https://schema.org/description)
pub type RecipeDescriptionFieldEnum = FieldEnum23;
/// [Schema Description](https://schema.org/description)
pub type DurationDescriptionFieldEnum = FieldEnum23;
/// [Schema Description](https://schema.org/description)
pub type MusicAlbumDescriptionFieldEnum = FieldEnum23;
/// [Schema Description](https://schema.org/description)
pub type CreativeWorkDescriptionFieldEnum = FieldEnum23;
/// [Schema Description](https://schema.org/description)
pub type EnergyDescriptionFieldEnum = FieldEnum23;
/// [Schema Description](https://schema.org/description)
pub type MonetaryAmountDescriptionFieldEnum = FieldEnum23;
/// [Schema Description](https://schema.org/description)
pub type TextObjectDescriptionFieldEnum = FieldEnum23;
/// [Schema Description](https://schema.org/description)
pub type LanguageDescriptionFieldEnum = FieldEnum23;
/// [Schema Description](https://schema.org/description)
pub type HowToDescriptionFieldEnum = FieldEnum23;
/// [Schema Description](https://schema.org/description)
pub type CommentDescriptionFieldEnum = FieldEnum23;
/// [Schema Description](https://schema.org/description)
pub type PropertyValueDescriptionFieldEnum = FieldEnum23;
/// [Schema Description](https://schema.org/description)
pub type VideoObjectDescriptionFieldEnum = FieldEnum23;
/// [Schema Description](https://schema.org/description)
pub type EventDescriptionFieldEnum = FieldEnum23;
/// [Schema Description](https://schema.org/description)
pub type HowToSupplyDescriptionFieldEnum = FieldEnum23;
/// [Schema Description](https://schema.org/description)
pub type EntryPointDescriptionFieldEnum = FieldEnum23;
/// [Schema Description](https://schema.org/description)
pub type PostalAddressDescriptionFieldEnum = FieldEnum23;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum FieldEnum24 {
    ///<https://schema.org/CreativeWork>
    CreativeWork(Box<CreativeWork>),
    ///<https://schema.org/Event>
    Event(Box<Event>),
}
impl Default for FieldEnum24 {
    fn default() -> Self {
        Self::CreativeWork(Box::default())
    }
}
///<https://schema.org/subjectOf>
pub type ClipSubjectOfFieldEnum = FieldEnum24;
///<https://schema.org/subjectOf>
pub type MusicPlaylistSubjectOfFieldEnum = FieldEnum24;
///<https://schema.org/subjectOf>
pub type MeasurementTypeEnumerationSubjectOfFieldEnum = FieldEnum24;
///<https://schema.org/subjectOf>
pub type StructuredValueSubjectOfFieldEnum = FieldEnum24;
///<https://schema.org/subjectOf>
pub type HowToToolSubjectOfFieldEnum = FieldEnum24;
///<https://schema.org/subjectOf>
pub type EnumerationSubjectOfFieldEnum = FieldEnum24;
///<https://schema.org/subjectOf>
pub type BreadcrumbListSubjectOfFieldEnum = FieldEnum24;
///<https://schema.org/subjectOf>
pub type MusicRecordingSubjectOfFieldEnum = FieldEnum24;
///<https://schema.org/subjectOf>
pub type WebContentSubjectOfFieldEnum = FieldEnum24;
///<https://schema.org/subjectOf>
pub type ProductSubjectOfFieldEnum = FieldEnum24;
///<https://schema.org/subjectOf>
pub type DefinedTermSetSubjectOfFieldEnum = FieldEnum24;
///<https://schema.org/subjectOf>
pub type ReviewSubjectOfFieldEnum = FieldEnum24;
///<https://schema.org/subjectOf>
pub type ThingSubjectOfFieldEnum = FieldEnum24;
///<https://schema.org/subjectOf>
pub type WebPageElementSubjectOfFieldEnum = FieldEnum24;
///<https://schema.org/subjectOf>
pub type WebPageSubjectOfFieldEnum = FieldEnum24;
///<https://schema.org/subjectOf>
pub type AudioObjectSubjectOfFieldEnum = FieldEnum24;
///<https://schema.org/subjectOf>
pub type DistanceSubjectOfFieldEnum = FieldEnum24;
///<https://schema.org/subjectOf>
pub type RatingSubjectOfFieldEnum = FieldEnum24;
///<https://schema.org/subjectOf>
pub type MediaObjectSubjectOfFieldEnum = FieldEnum24;
///<https://schema.org/subjectOf>
pub type MusicCompositionSubjectOfFieldEnum = FieldEnum24;
///<https://schema.org/subjectOf>
pub type ActionSubjectOfFieldEnum = FieldEnum24;
///<https://schema.org/subjectOf>
pub type ItemListSubjectOfFieldEnum = FieldEnum24;
///<https://schema.org/subjectOf>
pub type MusicReleaseSubjectOfFieldEnum = FieldEnum24;
///<https://schema.org/subjectOf>
pub type AggregateRatingSubjectOfFieldEnum = FieldEnum24;
///<https://schema.org/subjectOf>
pub type CountrySubjectOfFieldEnum = FieldEnum24;
///<https://schema.org/subjectOf>
pub type QualitativeValueSubjectOfFieldEnum = FieldEnum24;
///<https://schema.org/subjectOf>
pub type ImageObjectSubjectOfFieldEnum = FieldEnum24;
///<https://schema.org/subjectOf>
pub type HowToStepSubjectOfFieldEnum = FieldEnum24;
///<https://schema.org/subjectOf>
pub type ListItemSubjectOfFieldEnum = FieldEnum24;
///<https://schema.org/subjectOf>
pub type HowToSectionSubjectOfFieldEnum = FieldEnum24;
///<https://schema.org/subjectOf>
pub type DefinedTermSubjectOfFieldEnum = FieldEnum24;
///<https://schema.org/subjectOf>
pub type OrganizationSubjectOfFieldEnum = FieldEnum24;
///<https://schema.org/subjectOf>
pub type RecipeSubjectOfFieldEnum = FieldEnum24;
///<https://schema.org/subjectOf>
pub type LanguageSubjectOfFieldEnum = FieldEnum24;
///<https://schema.org/subjectOf>
pub type HowToSubjectOfFieldEnum = FieldEnum24;
///<https://schema.org/subjectOf>
pub type CommentSubjectOfFieldEnum = FieldEnum24;
///<https://schema.org/subjectOf>
pub type VideoObjectSubjectOfFieldEnum = FieldEnum24;
///<https://schema.org/subjectOf>
pub type EventSubjectOfFieldEnum = FieldEnum24;
///<https://schema.org/subjectOf>
pub type EntryPointSubjectOfFieldEnum = FieldEnum24;
///<https://schema.org/subjectOf>
pub type PostalAddressSubjectOfFieldEnum = FieldEnum24;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum FieldEnum26 {
    ///<https://schema.org/EntryPoint>
    EntryPoint(Box<EntryPoint>),
    ///<https://schema.org/URL>
    URL(String),
}
impl Default for FieldEnum26 {
    fn default() -> Self {
        Self::EntryPoint(Box::default())
    }
}
///<https://schema.org/target>
pub type ActionTargetFieldEnum = FieldEnum26;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum FieldEnum35 {
    ///<https://schema.org/PostalAddress>
    PostalAddress(Box<PostalAddress>),
    ///<https://schema.org/Text>
    Text(String),
}
impl Default for FieldEnum35 {
    fn default() -> Self {
        Self::Text(String::default())
    }
}
///<https://schema.org/address>
pub type PlaceAddressFieldEnum = FieldEnum35;
///<https://schema.org/address>
pub type CountryAddressFieldEnum = FieldEnum35;
///<https://schema.org/address>
pub type OrganizationAddressFieldEnum = FieldEnum35;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum FieldEnum41 {
    ///<https://schema.org/ItemList>
    ItemList(Box<ItemList>),
    ///<https://schema.org/MusicRecording>
    MusicRecording(Box<MusicRecording>),
}
impl Default for FieldEnum41 {
    fn default() -> Self {
        Self::ItemList(Box::default())
    }
}
///<https://schema.org/track>
pub type MusicPlaylistTrackFieldEnum = FieldEnum41;
///<https://schema.org/track>
pub type MusicReleaseTrackFieldEnum = FieldEnum41;
///<https://schema.org/track>
pub type MusicAlbumTrackFieldEnum = FieldEnum41;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum FieldEnum44 {
    ///<https://schema.org/DefinedTermSet>
    DefinedTermSet(Box<DefinedTermSet>),
    ///<https://schema.org/URL>
    URL(String),
}
impl Default for FieldEnum44 {
    fn default() -> Self {
        Self::DefinedTermSet(Box::default())
    }
}
///<https://schema.org/inDefinedTermSet>
pub type DefinedTermInDefinedTermSetFieldEnum = FieldEnum44;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
pub enum FieldEnum48 {
    ///<https://schema.org/Distance>
    Distance(Distance),
    ///<https://schema.org/QuantitativeValue>
    QuantitativeValue(QuantitativeValue),
    Integer(i32),
}
impl Default for FieldEnum48 {
    fn default() -> Self {
        Self::Distance(Distance::default())
    }
}
impl<'de> Deserialize<'de> for FieldEnum48 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = Value::deserialize(deserializer)?;
        match &value {
            Value::String(s) => {
                if let Ok(n) = s.trim().parse::<i32>() {
                    return Ok(Self::Integer(n));
                }

                Err(serde::de::Error::custom(format!(
                    "cannot parse into i32: {s}"
                )))
            }
            Value::Number(n) => {
                if let Some(i) = n.as_i64().and_then(|i| i32::try_from(i).ok()) {
                    return Ok(Self::Integer(i));
                }

                Err(serde::de::Error::custom(format!(
                    "number out of i32 range: {n}"
                )))
            }
            _ => {
                if let Ok(v) = serde_json::from_value::<Distance>(value.clone()) {
                    return Ok(Self::Distance(v));
                }

                if let Ok(v) = serde_json::from_value::<QuantitativeValue>(value.clone()) {
                    return Ok(Self::QuantitativeValue(v));
                }

                Err(serde::de::Error::custom(format!(
                    "cannot deserialize FieldEnum48 from {value}"
                )))
            }
        }
    }
}
///<https://schema.org/height>
pub type PersonHeightFieldEnum = FieldEnum48;
///<https://schema.org/width>
pub type MediaObjectWidthFieldEnum = FieldEnum48;
///<https://schema.org/height>
pub type MediaObjectHeightFieldEnum = FieldEnum48;
///<https://schema.org/width>
pub type ImageObjectWidthFieldEnum = FieldEnum48;
///<https://schema.org/height>
pub type ImageObjectHeightFieldEnum = FieldEnum48;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum FieldEnum49 {
    ///<https://schema.org/ItemList>
    ItemList(ItemList),
    ///<https://schema.org/ListItem>
    ListItem(ListItem),
    ///<https://schema.org/Text>
    Text(String),
    ///<https://schema.org/WebContent>
    WebContent(Box<WebContent>),
}
impl Default for FieldEnum49 {
    fn default() -> Self {
        Self::Text(String::new())
    }
}
///<https://schema.org/negativeNotes>
pub type ProductNegativeNotesFieldEnum = FieldEnum49;
///<https://schema.org/positiveNotes>
pub type ProductPositiveNotesFieldEnum = FieldEnum49;
///<https://schema.org/negativeNotes>
pub type ReviewNegativeNotesFieldEnum = FieldEnum49;
///<https://schema.org/positiveNotes>
pub type ReviewPositiveNotesFieldEnum = FieldEnum49;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum FieldEnum52 {
    ///<https://schema.org/Mass>
    Mass(Mass),
    ///<https://schema.org/QuantitativeValue>
    QuantitativeValue(Box<QuantitativeValue>),
}
impl Default for FieldEnum52 {
    fn default() -> Self {
        Self::Mass(Mass::default())
    }
}
///<https://schema.org/weight>
pub type ProductWeightFieldEnum = FieldEnum52;
///<https://schema.org/weight>
pub type PersonWeightFieldEnum = FieldEnum52;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum FieldEnum54 {
    ///<https://schema.org/Duration>
    Duration(Duration),
    ///<https://schema.org/QuantitativeValue>
    QuantitativeValue(QuantitativeValue),
}
impl Default for FieldEnum54 {
    fn default() -> Self {
        Self::Duration(Duration::default())
    }
}
///<https://schema.org/duration>
pub type MusicRecordingDurationFieldEnum = FieldEnum54;
///<https://schema.org/duration>
pub type AudioObjectDurationFieldEnum = FieldEnum54;
///<https://schema.org/duration>
pub type MediaObjectDurationFieldEnum = FieldEnum54;
///<https://schema.org/duration>
pub type MusicReleaseDurationFieldEnum = FieldEnum54;
///<https://schema.org/duration>
pub type ImageObjectDurationFieldEnum = FieldEnum54;
///<https://schema.org/duration>
pub type VideoObjectDurationFieldEnum = FieldEnum54;
///<https://schema.org/duration>
pub type EventDurationFieldEnum = FieldEnum54;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum FieldEnum59 {
    ///<https://schema.org/BreadcrumbList>
    BreadcrumbList(Box<BreadcrumbList>),
    ///<https://schema.org/Text>
    Text(String),
}
impl Default for FieldEnum59 {
    fn default() -> Self {
        Self::Text(String::new())
    }
}
///<https://schema.org/breadcrumb>
pub type WebPageBreadcrumbFieldEnum = FieldEnum59;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum FieldEnum60 {
    ///<https://schema.org/Number>
    Number(f32),
    ///<https://schema.org/QuantitativeValue>
    QuantitativeValue(Box<QuantitativeValue>),
    ///<https://schema.org/Text>
    Text(String),
}
impl Default for FieldEnum60 {
    fn default() -> Self {
        Self::Text(String::default())
    }
}
impl FieldEnum60 {
    pub fn quantity(&self) -> i16 {
        match self {
            Self::Number(n) => float_to_i16_safe(*n),
            Self::QuantitativeValue(q) => q
                .value
                .first()
                .map(|q| match q {
                    QuantitativeValueValueFieldEnum::BooleanEnumOrText(t) => {
                        t.parse().ok().unwrap_or_default()
                    }
                    QuantitativeValueValueFieldEnum::Number(n) => float_to_i16_safe(*n),
                    QuantitativeValueValueFieldEnum::QuantitativeValue(q) => q.to_number(),
                    QuantitativeValueValueFieldEnum::StructuredValue(v) => v
                        .name
                        .first()
                        .map(|n| n.parse().ok().unwrap_or_default())
                        .unwrap_or_default(),
                })
                .unwrap_or_default(),
            Self::Text(s) => s.parse::<i16>().unwrap_or_default(),
        }
    }
}
///<https://schema.org/requiredQuantity>
pub type HowToToolRequiredQuantityFieldEnum = FieldEnum60;
///<https://schema.org/requiredQuantity>
pub type HowToSupplyRequiredQuantityFieldEnum = FieldEnum60;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum FieldEnum63 {
    ///<https://schema.org/DefinedTerm>
    DefinedTerm(DefinedTerm),
    ///<https://schema.org/Enumeration>
    Enumeration(Enumeration),
    ///<https://schema.org/MeasurementTypeEnumeration>
    MeasurementTypeEnumeration(MeasurementTypeEnumeration),
    ///<https://schema.org/PropertyValue>
    PropertyValue(PropertyValue),
    ///<https://schema.org/QualitativeValue>
    QualitativeValue(QualitativeValue),
    ///<https://schema.org/QuantitativeValue>
    QuantitativeValue(QuantitativeValue),
    ///<https://schema.org/StructuredValue>
    StructuredValue(StructuredValue),
    ///<https://schema.org/Text>
    Text(String),
}
impl Default for FieldEnum63 {
    fn default() -> Self {
        Self::Text(String::default())
    }
}
///<https://schema.org/valueReference>
pub type QuantitativeValueValueReferenceFieldEnum = FieldEnum63;
///<https://schema.org/valueReference>
pub type SizeSpecificationValueReferenceFieldEnum = FieldEnum63;
///<https://schema.org/valueReference>
pub type QualitativeValueValueReferenceFieldEnum = FieldEnum63;
///<https://schema.org/valueReference>
pub type PropertyValueValueReferenceFieldEnum = FieldEnum63;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum FieldEnum64 {
    ///<https://schema.org/Boolean>
    ///<https://schema.org/Text>
    BooleanEnumOrText(String),
    ///<https://schema.org/Number>
    Number(f64),
    ///<https://schema.org/StructuredValue>
    StructuredValue(Box<StructuredValue>),
    ///<https://schema.org/QuantitativeValue>
    QuantitativeValue(Box<QuantitativeValue>),
}
impl Default for FieldEnum64 {
    fn default() -> Self {
        Self::Number(Default::default())
    }
}

impl<'de> Deserialize<'de> for FieldEnum64 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;

        match &value {
            Value::String(s) => Ok(Self::BooleanEnumOrText(s.clone())),
            Value::Number(n) => Ok(Self::Number(n.as_f64().unwrap_or_default())),
            Value::Object(map) => match map.get("@type").and_then(|v| v.as_str()) {
                Some("StructuredValue") => serde_json::from_value(value)
                    .map(Self::StructuredValue)
                    .map_err(serde::de::Error::custom),
                Some("QuantitativeValue") => serde_json::from_value(value)
                    .map(Self::QuantitativeValue)
                    .map_err(serde::de::Error::custom),
                _ => Err(serde::de::Error::custom(
                    "Expected StructuredValue or QuantitativeValue object for FieldEnum64",
                )),
            },
            _ => Err(serde::de::Error::custom(
                "Expected string or object for FieldEnum64",
            )),
        }
    }
}

///<https://schema.org/value>
pub type QuantitativeValueValueFieldEnum = FieldEnum64;
///<https://schema.org/value>
pub type MonetaryAmountValueFieldEnum = FieldEnum64;
///<https://schema.org/value>
pub type PropertyValueValueFieldEnum = FieldEnum64;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum FieldEnum73 {
    ///<https://schema.org/Comment>
    Comment(Comment),
    ///<https://schema.org/CreativeWork>
    CreativeWork(CreativeWork),
}
impl Default for FieldEnum73 {
    fn default() -> Self {
        Self::Comment(Comment::default())
    }
}
///<https://schema.org/parentItem>
pub type CorrectionCommentParentItemFieldEnum = FieldEnum73;
///<https://schema.org/parentItem>
pub type CommentParentItemFieldEnum = FieldEnum73;

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum FieldEnum91 {
    ///<https://schema.org/GenderType>
    GenderTypeEnum(GenderTypeEnum),
    ///<https://schema.org/Text>
    Text(String),
}
impl Default for FieldEnum91 {
    fn default() -> Self {
        Self::Text(String::default())
    }
}
///<https://schema.org/gender>
pub type PersonGenderFieldEnum = FieldEnum91;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum FieldEnum96 {
    ///<https://schema.org/MediaObject>
    MediaObject(Box<MediaObject>),
    ///<https://schema.org/Text>
    Text(String),
}
impl Default for FieldEnum96 {
    fn default() -> Self {
        Self::Text(String::default())
    }
}
///<https://schema.org/caption>
pub type AudioObjectCaptionFieldEnum = FieldEnum96;
///<https://schema.org/caption>
pub type ImageObjectCaptionFieldEnum = FieldEnum96;
///<https://schema.org/caption>
pub type VideoObjectCaptionFieldEnum = FieldEnum96;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum FieldEnum102 {
    ///<https://schema.org/MonetaryAmount>
    MonetaryAmount(Box<MonetaryAmount>),
    ///<https://schema.org/Text>
    Text(String),
}
impl Default for FieldEnum102 {
    fn default() -> Self {
        Self::Text(String::default())
    }
}
///<https://schema.org/estimatedCost>
pub type RecipeEstimatedCostFieldEnum = FieldEnum102;
///<https://schema.org/estimatedCost>
pub type HowToEstimatedCostFieldEnum = FieldEnum102;
///<https://schema.org/estimatedCost>
pub type HowToSupplyEstimatedCostFieldEnum = FieldEnum102;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum FieldEnum113 {
    ///<https://schema.org/ListItem>
    ListItem(ListItem),
    ///<https://schema.org/Text>
    Text(String),
    ///<https://schema.org/Thing>
    Thing(Thing),
}
impl Default for FieldEnum113 {
    fn default() -> Self {
        Self::Text(String::default())
    }
}
///<https://schema.org/itemListElement>
pub type ItemListItemListElementFieldEnum = FieldEnum113;
///<https://schema.org/itemListElement>
pub type BreadcrumbListItemListElementFieldEnum = FieldEnum113;
///<https://schema.org/itemListElement>
pub type HowToStepItemListElementFieldEnum = FieldEnum113;
///<https://schema.org/itemListElement>
pub type HowToSectionItemListElementFieldEnum = FieldEnum113;

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum FieldEnum114 {
    ///<https://schema.org/ItemListOrderType>
    ItemListOrderTypeEnum(ItemListOrderTypeEnum),
    ///<https://schema.org/Text>
    Text(String),
}
impl Default for FieldEnum114 {
    fn default() -> Self {
        Self::Text(String::default())
    }
}
///<https://schema.org/itemListOrder>
pub type ItemListItemListOrderFieldEnum = FieldEnum114;
///<https://schema.org/itemListOrder>
pub type HowToStepItemListOrderFieldEnum = FieldEnum114;
///<https://schema.org/itemListOrder>
pub type BreadcrumbListItemListOrderFieldEnum = FieldEnum114;
///<https://schema.org/itemListOrder>
pub type HowToSectionItemListOrderFieldEnum = FieldEnum114;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum FieldEnum116 {
    ///<https://schema.org/DefinedTerm>
    DefinedTerm(Box<DefinedTerm>),
    ///<https://schema.org/MeasurementMethodEnum>
    MeasurementMethodEnumEnum(MeasurementMethodEnumEnum),
    ///<https://schema.org/Text>
    ///<https://schema.org/URL>
    TextOrURL(String),
}
impl Default for FieldEnum116 {
    fn default() -> Self {
        Self::TextOrURL(String::default())
    }
}
///<https://schema.org/measurementMethod>
pub type PropertyValueMeasurementMethodFieldEnum = FieldEnum116;
///<https://schema.org/measurementTechnique>
pub type PropertyValueMeasurementTechniqueFieldEnum = FieldEnum116;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum FieldEnum129 {
    ///<https://schema.org/PropertyValue>
    PropertyValue(Box<PropertyValue>),
    ///<https://schema.org/Text>
    Text(String),
}
impl Default for FieldEnum129 {
    fn default() -> Self {
        Self::Text(String::new())
    }
}
pub type ImageObjectExifDataFieldEnum = FieldEnum129;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
pub enum FieldEnum141 {
    ///<https://schema.org/CreativeWork>
    CreativeWork(Box<CreativeWork>),
    ///<https://schema.org/ItemList>
    ItemList(Box<ItemList>),
    ///<https://schema.org/HowToStep>
    HowToStep(Box<HowToStep>),
    ///<https://schema.org/Text>
    Text(String),
}
impl Default for FieldEnum141 {
    fn default() -> Self {
        Self::Text(String::default())
    }
}
impl<'de> Deserialize<'de> for FieldEnum141 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = Value::deserialize(deserializer)?;

        if let Some(t) = value.get("@type") {
            let types: Vec<&str> = match t {
                Value::String(s) => vec![s.as_str()],
                Value::Array(arr) => arr.iter().filter_map(|v| v.as_str()).collect(),
                _ => vec![],
            };

            for t in &types {
                match *t {
                    "HowToStep" => {
                        return serde_json::from_value::<HowToStep>(value)
                            .map(|v| Self::HowToStep(Box::new(v)))
                            .map_err(serde::de::Error::custom);
                    }
                    "ItemList" | "HowToSection" if value.get("itemListElement").is_some() => {
                        return serde_json::from_value::<ItemList>(value)
                            .map(|v| Self::ItemList(Box::new(v)))
                            .map_err(serde::de::Error::custom);
                    }
                    "CreativeWork" => {
                        return serde_json::from_value::<CreativeWork>(value)
                            .map(|v| Self::CreativeWork(Box::new(v)))
                            .map_err(serde::de::Error::custom);
                    }
                    _ => {}
                }
            }
        }

        if value.is_string() {
            return Ok(Self::Text(value.as_str().unwrap().to_string()));
        }

        if let Ok(v) = serde_json::from_value::<HowToStep>(value.clone()) {
            return Ok(Self::HowToStep(Box::new(v)));
        }
        if let Ok(v) = serde_json::from_value::<ItemList>(value.clone()) {
            return Ok(Self::ItemList(Box::new(v)));
        }
        if let Ok(v) = serde_json::from_value::<CreativeWork>(value) {
            return Ok(Self::CreativeWork(Box::new(v)));
        }

        Err(serde::de::Error::custom(
            "data did not match any variant of FieldEnum141",
        ))
    }
}
///<https://schema.org/steps>
pub type HowToSectionStepsFieldEnum = FieldEnum141;
///<https://schema.org/recipeInstructions>
pub type RecipeRecipeInstructionsFieldEnum = FieldEnum141;

impl RecipeRecipeInstructionsFieldEnum {
    /// Creates a new section.
    pub fn new_section<T: Into<String>>(name: &str, items: Vec<T>) -> Self {
        let num_items = i32::try_from(items.len()).unwrap_or_default();

        Self::ItemList(
            ItemList {
                item_list_element: items
                    .into_iter()
                    .map(|v| {
                        let s: String = v.into();
                        ItemListItemListElementFieldEnum::Text(s.trim().to_string())
                    })
                    .collect(),
                name: vec![name.into()],
                number_of_items: vec![num_items],
                ..Default::default()
            }
            .into(),
        )
    }

    /// Adds a new item to the section.
    pub fn push_item(&mut self, item: &str) {
        if let Self::ItemList(list) = self {
            list.item_list_element
                .push(ItemListItemListElementFieldEnum::Text(item.to_string()));
        }
    }

    /// Increments the number of items in the section.
    pub fn increment_items(&mut self) {
        if let Self::ItemList(list) = self
            && let Some(i) = list.number_of_items.first_mut()
        {
            *i += 1;
        }
    }

    /// Creates a new `CreativeWork` field with the given text and optional image.
    pub fn new_creative_work(
        text: &str,
        image: Option<&str>,
        name: Option<&str>,
        url: Option<&str>,
    ) -> Self {
        Self::CreativeWork(
            CreativeWork {
                r#type: AtType::HowToStep.to_opt(),
                text: vec![text.into()],
                image: image
                    .map(|u| {
                        vec![CreativeWorkImageFieldEnum::ImageObject(
                            ImageObject {
                                r#type: AtType::ImageObject.to_opt(),
                                url: vec![u.into()],
                                ..Default::default()
                            }
                            .into(),
                        )]
                    })
                    .unwrap_or_default(),
                name: name.map(|n| vec![n.into()]).unwrap_or_default(),
                url: url.map(|u| vec![u.into()]).unwrap_or_default(),
                ..Default::default()
            }
            .into(),
        )
    }
}

///<https://schema.org/steps>
pub type RecipeStepsFieldEnum = FieldEnum141;
///<https://schema.org/steps>
pub type HowToStepsFieldEnum = FieldEnum141;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
pub enum FieldEnum149 {
    ///<https://schema.org/PropertyValue>
    PropertyValue(PropertyValue),
    ///<https://schema.org/ItemList>
    ItemList(ItemList),
    ///<https://schema.org/Text>
    Text(String),
}

impl Default for FieldEnum149 {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl<'de> Deserialize<'de> for FieldEnum149 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;

        match &value {
            Value::String(s) => Ok(Self::Text(s.clone())),
            Value::Object(map) => match map.get("@type").and_then(|v| v.as_str()) {
                Some("PropertyValue") => serde_json::from_value(value)
                    .map(Self::PropertyValue)
                    .map_err(serde::de::Error::custom),
                Some("ItemList") => serde_json::from_value(value)
                    .map(Self::ItemList)
                    .map_err(serde::de::Error::custom),
                _ => serde_json::from_value::<PropertyValue>(value.clone())
                    .map(Self::PropertyValue)
                    .or_else(|_| serde_json::from_value(value).map(Self::ItemList))
                    .map_err(serde::de::Error::custom),
            },
            _ => Err(serde::de::Error::custom(
                "Expected string or object for FieldEnum149",
            )),
        }
    }
}

///<https://schema.org/recipeIngredient>
pub type RecipeRecipeIngredientFieldEnum = FieldEnum149;

impl RecipeRecipeIngredientFieldEnum {
    /// Creates a new `ItemList` that contains a title and a list of items.
    pub fn new_section(name: &str, items: &[&str]) -> Self {
        Self::ItemList(ItemList {
            r#type: AtType::ItemList.to_string(),
            item_list_element: items
                .iter()
                .map(|v| ItemListItemListElementFieldEnum::Text(v.to_string()))
                .collect(),
            name: vec![name.into()],
            number_of_items: vec![i32::try_from(items.len()).unwrap_or_default()],
            ..Default::default()
        })
    }

    /// Extracts all the items from the enum.
    pub fn item_names(&self) -> Vec<String> {
        match self {
            Self::ItemList(list) => list
                .item_list_element
                .iter()
                .flat_map(|v| match v {
                    ItemListItemListElementFieldEnum::ListItem(l) => l.name.clone(),
                    ItemListItemListElementFieldEnum::Text(s) => vec![s.clone()],
                    ItemListItemListElementFieldEnum::Thing(t) => t.name.clone(),
                })
                .collect::<Vec<_>>(),
            Self::PropertyValue(prop) => prop.name.clone(),
            Self::Text(s) => vec![s.clone()],
        }
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum FieldEnum150 {
    ///<https://schema.org/HowToTool>
    HowToTool(Box<HowToTool>),
    ///<https://schema.org/Text>
    Text(String),
}
impl Default for FieldEnum150 {
    fn default() -> Self {
        Self::Text(String::new())
    }
}
impl FieldEnum150 {
    pub fn new_tool(name: impl Into<String>, quantity: f32) -> Self {
        Self::HowToTool(Box::new(HowToTool {
            r#type: AtType::HowToTool.to_opt(),
            name: vec![name.into()],
            required_quantity: vec![HowToToolRequiredQuantityFieldEnum::Number(quantity)],
            ..Default::default()
        }))
    }
}
///<https://schema.org/tool>
pub type RecipeToolFieldEnum = FieldEnum150;
///<https://schema.org/tool>
pub type HowToToolFieldEnum = FieldEnum150;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum FieldEnum151 {
    ///<https://schema.org/CreativeWork>
    CreativeWork(CreativeWork),
    ///<https://schema.org/HowToSection>
    HowToSection(HowToSection),
    ///<https://schema.org/HowToStep>
    HowToStep(Box<HowToStep>),
    ///<https://schema.org/Text>
    Text(String),
}
impl Default for FieldEnum151 {
    fn default() -> Self {
        Self::Text(String::new())
    }
}
///<https://schema.org/step>
pub type RecipeStepFieldEnum = FieldEnum151;
///<https://schema.org/step>
pub type HowToStepFieldEnum = FieldEnum151;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum FieldEnum152 {
    ///<https://schema.org/HowToSupply>
    HowToSupply(Box<HowToSupply>),
    ///<https://schema.org/Text>
    Text(String),
}
impl Default for FieldEnum152 {
    fn default() -> Self {
        Self::Text(String::new())
    }
}
///<https://schema.org/supply>
pub type RecipeSupplyFieldEnum = FieldEnum152;
///<https://schema.org/supply>
pub type HowToSupplyFieldEnum = FieldEnum152;

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum IntegerOrText {
    Integer(i32),
    Text(String),
}
impl Default for IntegerOrText {
    fn default() -> Self {
        Self::Integer(0)
    }
}

pub(crate) fn float_to_i16_safe<F: Into<f64>>(n: F) -> i16 {
    let clamped = n
        .into()
        .round()
        .clamp(f64::from(i16::MIN), f64::from(i16::MAX));
    #[allow(clippy::cast_possible_truncation)]
    {
        clamped as i16
    }
}
