use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{Action, Comment, CreativeWork, Duration, Event, Place, Review, Thing};
use crate::aggregate_rating::AggregateRating;
use crate::alignment_object::AlignmentObject;
use crate::audience::Audience;
use crate::claim::Claim;
use crate::country::Country;
use crate::enums::IPTCDigitalSourceEnumerationEnum;
use crate::helpers::one_or_many;
use crate::field::*;
use crate::grant::Grant;
use crate::image_object::ImageObject;
use crate::interaction_counter::InteractionCounter;
use crate::item_list::ItemList;
use crate::news_article::NewsArticle;
use crate::organization::Organization;
use crate::person::Person;
use crate::publication_event::PublicationEvent;

///<https://schema.org/endTime>
///<https://schema.org/DateTime>
///<https://schema.org/Time>
pub type MediaObjectEndTimeFieldEnum = String;
///<https://schema.org/startTime>
///<https://schema.org/DateTime>
///<https://schema.org/Time>
pub type MediaObjectStartTimeFieldEnum = String;
///<https://schema.org/encodingFormat>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type MediaObjectEncodingFormatFieldEnum = String;
///<https://schema.org/uploadDate>
///<https://schema.org/Date>
///<https://schema.org/DateTime>
pub type MediaObjectUploadDateFieldEnum = String;
///<https://schema.org/dateCreated>
///<https://schema.org/Date>
///<https://schema.org/DateTime>
pub type MediaObjectDateCreatedFieldEnum = String;
///<https://schema.org/expires>
///<https://schema.org/Date>
///<https://schema.org/DateTime>
pub type MediaObjectExpiresFieldEnum = String;
///<https://schema.org/temporalCoverage>
///<https://schema.org/DateTime>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type MediaObjectTemporalCoverageFieldEnum = String;
///<https://schema.org/fileFormat>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type MediaObjectFileFormatFieldEnum = String;
///<https://schema.org/dateModified>
///<https://schema.org/Date>
///<https://schema.org/DateTime>
pub type MediaObjectDateModifiedFieldEnum = String;
///<https://schema.org/temporal>
///<https://schema.org/DateTime>
///<https://schema.org/Text>
pub type MediaObjectTemporalFieldEnum = String;
///<https://schema.org/datePublished>
///<https://schema.org/Date>
///<https://schema.org/DateTime>
pub type MediaObjectDatePublishedFieldEnum = String;
///<https://schema.org/genre>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type MediaObjectGenreFieldEnum = String;
///<https://schema.org/editEIDR>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type MediaObjectEditEIDRFieldEnum = String;
///<https://schema.org/schemaVersion>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type MediaObjectSchemaVersionFieldEnum = String;
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type MediaObjectAdditionalTypeFieldEnum = String;

///<https://schema.org/MediaObject>
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct MediaObject {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/requiresSubscription>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub requires_subscription: Vec<MediaObjectRequiresSubscriptionFieldEnum>,
    ///<https://schema.org/sha256>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sha256: Vec<String>,
    ///<https://schema.org/associatedArticle>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub associated_article: Vec<NewsArticle>,
    ///<https://schema.org/encodesCreativeWork>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub encodes_creative_work: Vec<CreativeWork>,
    ///<https://schema.org/endTime>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub end_time: Vec<MediaObjectEndTimeFieldEnum>,
    ///<https://schema.org/width>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub width: Vec<MediaObjectWidthFieldEnum>,
    ///<https://schema.org/regionsAllowed>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub regions_allowed: Vec<Place>,
    ///<https://schema.org/startTime>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub start_time: Vec<MediaObjectStartTimeFieldEnum>,
    ///<https://schema.org/encodingFormat>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub encoding_format: Vec<MediaObjectEncodingFormatFieldEnum>,
    ///<https://schema.org/height>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub height: Vec<MediaObjectHeightFieldEnum>,
    ///<https://schema.org/duration>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub duration: Vec<MediaObjectDurationFieldEnum>,
    ///<https://schema.org/interpretedAsClaim>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub interpreted_as_claim: Vec<Claim>,
    ///<https://schema.org/contentUrl>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub content_url: Vec<String>,
    ///<https://schema.org/productionCompany>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub production_company: Vec<Organization>,
    ///<https://schema.org/uploadDate>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub upload_date: Vec<MediaObjectUploadDateFieldEnum>,
    ///<https://schema.org/ineligibleRegion>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ineligible_region: Vec<MediaObjectIneligibleRegionFieldEnum>,
    ///<https://schema.org/embedUrl>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub embed_url: Vec<String>,
    ///<https://schema.org/bitrate>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub bitrate: Vec<String>,
    ///<https://schema.org/contentSize>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub content_size: Vec<String>,
    ///<https://schema.org/playerType>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub player_type: Vec<String>,
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
    pub is_based_on_url: Vec<MediaObjectIsBasedOnUrlFieldEnum>,
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
    pub date_created: Vec<MediaObjectDateCreatedFieldEnum>,
    ///<https://schema.org/wordCount>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub word_count: Vec<i32>,
    ///<https://schema.org/size>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub size: Vec<MediaObjectSizeFieldEnum>,
    ///<https://schema.org/maintainer>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub maintainer: Vec<MediaObjectMaintainerFieldEnum>,
    ///<https://schema.org/license>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub license: Vec<MediaObjectLicenseFieldEnum>,
    ///<https://schema.org/expires>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub expires: Vec<MediaObjectExpiresFieldEnum>,
    ///<https://schema.org/version>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub version: Vec<MediaObjectVersionFieldEnum>,
    ///<https://schema.org/educationalLevel>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub educational_level: Vec<MediaObjectEducationalLevelFieldEnum>,
    ///<https://schema.org/commentCount>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub comment_count: Vec<i32>,
    ///<https://schema.org/offers>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub offers: Vec<MediaObjectOffersFieldEnum>,
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
    pub contributor: Vec<MediaObjectContributorFieldEnum>,
    ///<https://schema.org/temporalCoverage>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub temporal_coverage: Vec<MediaObjectTemporalCoverageFieldEnum>,
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
    pub publisher: Vec<MediaObjectPublisherFieldEnum>,
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
    pub file_format: Vec<MediaObjectFileFormatFieldEnum>,
    ///<https://schema.org/material>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub material: Vec<MediaObjectMaterialFieldEnum>,
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
    pub date_modified: Vec<MediaObjectDateModifiedFieldEnum>,
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
    pub keywords: Vec<MediaObjectKeywordsFieldEnum>,
    ///<https://schema.org/provider>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub provider: Vec<MediaObjectProviderFieldEnum>,
    ///<https://schema.org/creator>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub creator: Vec<MediaObjectCreatorFieldEnum>,
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
    pub assesses: Vec<MediaObjectAssessesFieldEnum>,
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
    pub teaches: Vec<MediaObjectTeachesFieldEnum>,
    ///<https://schema.org/archivedAt>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub archived_at: Vec<MediaObjectArchivedAtFieldEnum>,
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
    pub content_rating: Vec<MediaObjectContentRatingFieldEnum>,
    ///<https://schema.org/funder>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub funder: Vec<MediaObjectFunderFieldEnum>,
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
    pub temporal: Vec<MediaObjectTemporalFieldEnum>,
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
    pub is_based_on: Vec<MediaObjectIsBasedOnFieldEnum>,
    ///<https://schema.org/pattern>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub pattern: Vec<MediaObjectPatternFieldEnum>,
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
    pub sponsor: Vec<MediaObjectSponsorFieldEnum>,
    ///<https://schema.org/aggregateRating>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub aggregate_rating: Vec<AggregateRating>,
    ///<https://schema.org/correction>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub correction: Vec<MediaObjectCorrectionFieldEnum>,
    ///<https://schema.org/accessibilityAPI>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub accessibility_api: Vec<String>,
    ///<https://schema.org/copyrightHolder>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub copyright_holder: Vec<MediaObjectCopyrightHolderFieldEnum>,
    ///<https://schema.org/inLanguage>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub in_language: Vec<MediaObjectInLanguageFieldEnum>,
    ///<https://schema.org/usageInfo>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub usage_info: Vec<MediaObjectUsageInfoFieldEnum>,
    ///<https://schema.org/datePublished>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub date_published: Vec<MediaObjectDatePublishedFieldEnum>,
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
    pub sd_publisher: Vec<MediaObjectSdPublisherFieldEnum>,
    ///<https://schema.org/genre>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub genre: Vec<MediaObjectGenreFieldEnum>,
    ///<https://schema.org/audio>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub audio: Vec<MediaObjectAudioFieldEnum>,
    ///<https://schema.org/creativeWorkStatus>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub creative_work_status: Vec<MediaObjectCreativeWorkStatusFieldEnum>,
    ///<https://schema.org/alternativeHeadline>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub alternative_headline: Vec<String>,
    ///<https://schema.org/editEIDR>
    #[serde(rename = "editEIDR")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub edit_eidr: Vec<MediaObjectEditEIDRFieldEnum>,
    ///<https://schema.org/learningResourceType>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub learning_resource_type: Vec<MediaObjectLearningResourceTypeFieldEnum>,
    ///<https://schema.org/about>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub about: Vec<Thing>,
    ///<https://schema.org/isPartOf>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub is_part_of: Vec<MediaObjectIsPartOfFieldEnum>,
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
    pub acquire_license_page: Vec<MediaObjectAcquireLicensePageFieldEnum>,
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
    pub publishing_principles: Vec<MediaObjectPublishingPrinciplesFieldEnum>,
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
    pub citation: Vec<MediaObjectCitationFieldEnum>,
    ///<https://schema.org/video>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub video: Vec<MediaObjectVideoFieldEnum>,
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
    pub producer: Vec<MediaObjectProducerFieldEnum>,
    ///<https://schema.org/schemaVersion>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub schema_version: Vec<MediaObjectSchemaVersionFieldEnum>,
    ///<https://schema.org/accessibilityControl>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub accessibility_control: Vec<String>,
    ///<https://schema.org/author>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub author: Vec<MediaObjectAuthorFieldEnum>,
    ///<https://schema.org/translator>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub translator: Vec<MediaObjectTranslatorFieldEnum>,
    ///<https://schema.org/materialExtent>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub material_extent: Vec<MediaObjectMaterialExtentFieldEnum>,
    ///<https://schema.org/sourceOrganization>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub source_organization: Vec<Organization>,
    ///<https://schema.org/position>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub position: Vec<MediaObjectPositionFieldEnum>,
    ///<https://schema.org/educationalUse>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub educational_use: Vec<MediaObjectEducationalUseFieldEnum>,
    ///<https://schema.org/sdLicense>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sd_license: Vec<MediaObjectSdLicenseFieldEnum>,
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
    pub additional_type: Vec<MediaObjectAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<MediaObjectIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub image: Vec<MediaObjectImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub description: Vec<MediaObjectDescriptionFieldEnum>,
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
    pub subject_of: Vec<MediaObjectSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub main_entity_of_page: Vec<MediaObjectMainEntityOfPageFieldEnum>,
}
