use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::helpers::one_or_many;
use crate::{Comment, Duration, Event, Place, Thing};
use crate::enums::IPTCDigitalSourceEnumerationEnum;
use crate::field::*;

///<https://schema.org/dateCreated>
///<https://schema.org/Date>
///<https://schema.org/DateTime>
pub type CreativeWorkDateCreatedFieldEnum = String;
///<https://schema.org/expires>
///<https://schema.org/Date>
///<https://schema.org/DateTime>
pub type CreativeWorkExpiresFieldEnum = String;
///<https://schema.org/temporalCoverage>
///<https://schema.org/DateTime>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type CreativeWorkTemporalCoverageFieldEnum = String;
///<https://schema.org/fileFormat>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type CreativeWorkFileFormatFieldEnum = String;
///<https://schema.org/dateModified>
///<https://schema.org/Date>
///<https://schema.org/DateTime>
pub type CreativeWorkDateModifiedFieldEnum = String;
///<https://schema.org/encodingFormat>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type CreativeWorkEncodingFormatFieldEnum = String;
///<https://schema.org/temporal>
///<https://schema.org/DateTime>
///<https://schema.org/Text>
pub type CreativeWorkTemporalFieldEnum = String;
///<https://schema.org/datePublished>
///<https://schema.org/Date>
///<https://schema.org/DateTime>
pub type CreativeWorkDatePublishedFieldEnum = String;
///<https://schema.org/genre>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type CreativeWorkGenreFieldEnum = String;
///<https://schema.org/editEIDR>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type CreativeWorkEditEIDRFieldEnum = String;
///<https://schema.org/schemaVersion>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type CreativeWorkSchemaVersionFieldEnum = String;
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type CreativeWorkAdditionalTypeFieldEnum = String;

///<https://schema.org/CreativeWork>
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CreativeWork {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/contentLocation>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub content_location: Vec<Place>,
    ///<https://schema.org/recordedAt>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub recorded_at: Vec<Event>,
    ///<https://schema.org/comment>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<Comment>,
    ///<https://schema.org/isBasedOnUrl>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub is_based_on_url: Vec<CreativeWorkIsBasedOnUrlFieldEnum>,
    ///<https://schema.org/translationOfWork>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub translation_of_work: Vec<CreativeWork>,
    ///<https://schema.org/workTranslation>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub work_translation: Vec<CreativeWork>,
    ///<https://schema.org/mentions>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub mentions: Vec<Thing>,
    ///<https://schema.org/dateCreated>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub date_created: Vec<CreativeWorkDateCreatedFieldEnum>,
    ///<https://schema.org/wordCount>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub word_count: Vec<i32>,
    ///<https://schema.org/size>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub size: Vec<CreativeWorkSizeFieldEnum>,
    ///<https://schema.org/maintainer>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub maintainer: Vec<CreativeWorkMaintainerFieldEnum>,
    ///<https://schema.org/license>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub license: Vec<CreativeWorkLicenseFieldEnum>,
    ///<https://schema.org/expires>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub expires: Vec<CreativeWorkExpiresFieldEnum>,
    ///<https://schema.org/version>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub version: Vec<CreativeWorkVersionFieldEnum>,
    ///<https://schema.org/educationalLevel>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub educational_level: Vec<CreativeWorkEducationalLevelFieldEnum>,
    ///<https://schema.org/commentCount>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub comment_count: Vec<i32>,
    ///<https://schema.org/offers>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub offers: Vec<CreativeWorkOffersFieldEnum>,
    ///<https://schema.org/timeRequired>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub time_required: Vec<Duration>,
    ///<https://schema.org/audience>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub audience: Vec<Audience>,
    ///<https://schema.org/review>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub review: Vec<Review>,
    ///<https://schema.org/contributor>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub contributor: Vec<CreativeWorkContributorFieldEnum>,
    ///<https://schema.org/temporalCoverage>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub temporal_coverage: Vec<CreativeWorkTemporalCoverageFieldEnum>,
    ///<https://schema.org/interactionStatistic>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub interaction_statistic: Vec<InteractionCounter>,
    ///<https://schema.org/mainEntity>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub main_entity: Vec<Thing>,
    ///<https://schema.org/publisher>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub publisher: Vec<CreativeWorkPublisherFieldEnum>,
    ///<https://schema.org/creditText>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub credit_text: Vec<String>,
    ///<https://schema.org/character>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub character: Vec<Person>,
    ///<https://schema.org/copyrightNotice>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub copyright_notice: Vec<String>,
    ///<https://schema.org/headline>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub headline: Vec<String>,
    ///<https://schema.org/fileFormat>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub file_format: Vec<CreativeWorkFileFormatFieldEnum>,
    ///<https://schema.org/material>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub material: Vec<CreativeWorkMaterialFieldEnum>,
    ///<https://schema.org/hasPart>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub has_part: Vec<CreativeWork>,
    ///<https://schema.org/editor>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub editor: Vec<Person>,
    ///<https://schema.org/publication>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub publication: Vec<PublicationEvent>,
    ///<https://schema.org/accessibilityHazard>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub accessibility_hazard: Vec<String>,
    ///<https://schema.org/dateModified>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub date_modified: Vec<CreativeWorkDateModifiedFieldEnum>,
    ///<https://schema.org/isAccessibleForFree>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub is_accessible_for_free: Vec<String>,
    ///<https://schema.org/encodings>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub encodings: Vec<MediaObject>,
    ///<https://schema.org/keywords>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub keywords: Vec<CreativeWorkKeywordsFieldEnum>,
    ///<https://schema.org/encodingFormat>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub encoding_format: Vec<CreativeWorkEncodingFormatFieldEnum>,
    ///<https://schema.org/provider>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub provider: Vec<CreativeWorkProviderFieldEnum>,
    ///<https://schema.org/creator>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub creator: Vec<CreativeWorkCreatorFieldEnum>,
    ///<https://schema.org/accessMode>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub access_mode: Vec<String>,
    ///<https://schema.org/sdDatePublished>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sd_date_published: Vec<String>,
    ///<https://schema.org/exampleOfWork>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub example_of_work: Vec<CreativeWork>,
    ///<https://schema.org/assesses>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub assesses: Vec<CreativeWorkAssessesFieldEnum>,
    ///<https://schema.org/contentReferenceTime>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub content_reference_time: Vec<String>,
    ///<https://schema.org/locationCreated>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub location_created: Vec<Place>,
    ///<https://schema.org/teaches>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub teaches: Vec<CreativeWorkTeachesFieldEnum>,
    ///<https://schema.org/archivedAt>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub archived_at: Vec<CreativeWorkArchivedAtFieldEnum>,
    ///<https://schema.org/accessibilitySummary>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub accessibility_summary: Vec<String>,
    ///<https://schema.org/encoding>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub encoding: Vec<MediaObject>,
    ///<https://schema.org/typicalAgeRange>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub typical_age_range: Vec<String>,
    ///<https://schema.org/interpretedAsClaim>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub interpreted_as_claim: Vec<Claim>,
    ///<https://schema.org/publisherImprint>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub publisher_imprint: Vec<Organization>,
    ///<https://schema.org/discussionUrl>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub discussion_url: Vec<String>,
    ///<https://schema.org/contentRating>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub content_rating: Vec<CreativeWorkContentRatingFieldEnum>,
    ///<https://schema.org/funder>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub funder: Vec<CreativeWorkFunderFieldEnum>,
    ///<https://schema.org/countryOfOrigin>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub country_of_origin: Vec<Country>,
    ///<https://schema.org/text>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub text: Vec<String>,
    ///<https://schema.org/accountablePerson>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub accountable_person: Vec<Person>,
    ///<https://schema.org/temporal>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub temporal: Vec<CreativeWorkTemporalFieldEnum>,
    ///<https://schema.org/associatedMedia>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub associated_media: Vec<MediaObject>,
    ///<https://schema.org/spatialCoverage>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub spatial_coverage: Vec<Place>,
    ///<https://schema.org/award>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub award: Vec<String>,
    ///<https://schema.org/isBasedOn>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub is_based_on: Vec<CreativeWorkIsBasedOnFieldEnum>,
    ///<https://schema.org/pattern>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub pattern: Vec<CreativeWorkPatternFieldEnum>,
    ///<https://schema.org/interactivityType>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub interactivity_type: Vec<String>,
    ///<https://schema.org/abstract>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub _abstract: Vec<String>,
    ///<https://schema.org/sponsor>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sponsor: Vec<CreativeWorkSponsorFieldEnum>,
    ///<https://schema.org/aggregateRating>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub aggregate_rating: Vec<AggregateRating>,
    ///<https://schema.org/correction>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub correction: Vec<CreativeWorkCorrectionFieldEnum>,
    ///<https://schema.org/accessibilityAPI>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub accessibility_api: Vec<String>,
    ///<https://schema.org/copyrightHolder>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub copyright_holder: Vec<CreativeWorkCopyrightHolderFieldEnum>,
    ///<https://schema.org/inLanguage>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub in_language: Vec<CreativeWorkInLanguageFieldEnum>,
    ///<https://schema.org/usageInfo>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub usage_info: Vec<CreativeWorkUsageInfoFieldEnum>,
    ///<https://schema.org/datePublished>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub date_published: Vec<CreativeWorkDatePublishedFieldEnum>,
    ///<https://schema.org/isFamilyFriendly>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub is_family_friendly: Vec<String>,
    ///<https://schema.org/digitalSourceType>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub digital_source_type: Vec<IPTCDigitalSourceEnumerationEnum>,
    ///<https://schema.org/sdPublisher>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sd_publisher: Vec<CreativeWorkSdPublisherFieldEnum>,
    ///<https://schema.org/genre>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub genre: Vec<CreativeWorkGenreFieldEnum>,
    ///<https://schema.org/audio>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub audio: Vec<CreativeWorkAudioFieldEnum>,
    ///<https://schema.org/creativeWorkStatus>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub creative_work_status: Vec<CreativeWorkCreativeWorkStatusFieldEnum>,
    ///<https://schema.org/alternativeHeadline>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub alternative_headline: Vec<String>,
    ///<https://schema.org/editEIDR>
    #[serde(rename = "editEIDR")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub edit_eidr: Vec<CreativeWorkEditEIDRFieldEnum>,
    ///<https://schema.org/learningResourceType>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub learning_resource_type: Vec<CreativeWorkLearningResourceTypeFieldEnum>,
    ///<https://schema.org/about>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub about: Vec<Thing>,
    ///<https://schema.org/isPartOf>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub is_part_of: Vec<CreativeWorkIsPartOfFieldEnum>,
    ///<https://schema.org/funding>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub funding: Vec<Grant>,
    ///<https://schema.org/educationalAlignment>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub educational_alignment: Vec<AlignmentObject>,
    ///<https://schema.org/accessModeSufficient>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub access_mode_sufficient: Vec<ItemList>,
    ///<https://schema.org/acquireLicensePage>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub acquire_license_page: Vec<CreativeWorkAcquireLicensePageFieldEnum>,
    ///<https://schema.org/conditionsOfAccess>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub conditions_of_access: Vec<String>,
    ///<https://schema.org/thumbnail>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub thumbnail: Vec<ImageObject>,
    ///<https://schema.org/publishingPrinciples>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub publishing_principles: Vec<CreativeWorkPublishingPrinciplesFieldEnum>,
    ///<https://schema.org/thumbnailUrl>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub thumbnail_url: Vec<String>,
    ///<https://schema.org/copyrightYear>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub copyright_year: Vec<f32>,
    ///<https://schema.org/workExample>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub work_example: Vec<CreativeWork>,
    ///<https://schema.org/accessibilityFeature>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub accessibility_feature: Vec<String>,
    ///<https://schema.org/citation>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub citation: Vec<CreativeWorkCitationFieldEnum>,
    ///<https://schema.org/video>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub video: Vec<CreativeWorkVideoFieldEnum>,
    ///<https://schema.org/awards>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub awards: Vec<String>,
    ///<https://schema.org/spatial>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub spatial: Vec<Place>,
    ///<https://schema.org/producer>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub producer: Vec<CreativeWorkProducerFieldEnum>,
    ///<https://schema.org/schemaVersion>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub schema_version: Vec<CreativeWorkSchemaVersionFieldEnum>,
    ///<https://schema.org/accessibilityControl>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub accessibility_control: Vec<String>,
    ///<https://schema.org/author>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub author: Vec<CreativeWorkAuthorFieldEnum>,
    ///<https://schema.org/translator>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub translator: Vec<CreativeWorkTranslatorFieldEnum>,
    ///<https://schema.org/materialExtent>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub material_extent: Vec<CreativeWorkMaterialExtentFieldEnum>,
    ///<https://schema.org/sourceOrganization>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub source_organization: Vec<Organization>,
    ///<https://schema.org/position>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub position: Vec<CreativeWorkPositionFieldEnum>,
    ///<https://schema.org/educationalUse>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub educational_use: Vec<CreativeWorkEducationalUseFieldEnum>,
    ///<https://schema.org/sdLicense>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sd_license: Vec<CreativeWorkSdLicenseFieldEnum>,
    ///<https://schema.org/releasedEvent>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub released_event: Vec<PublicationEvent>,
    ///<https://schema.org/reviews>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub reviews: Vec<Review>,
    ///<https://schema.org/disambiguatingDescription>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub disambiguating_description: Vec<String>,
    ///<https://schema.org/potentialAction>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub potential_action: Vec<Action>,
    ///<https://schema.org/additionalType>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub additional_type: Vec<CreativeWorkAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<CreativeWorkIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub image: Vec<CreativeWorkImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub description: Vec<CreativeWorkDescriptionFieldEnum>,
    ///<https://schema.org/alternateName>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub alternate_name: Vec<String>,
    ///<https://schema.org/url>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub url: Vec<String>,
    ///<https://schema.org/subjectOf>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub subject_of: Vec<CreativeWorkSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub main_entity_of_page: Vec<CreativeWorkMainEntityOfPageFieldEnum>,
}
