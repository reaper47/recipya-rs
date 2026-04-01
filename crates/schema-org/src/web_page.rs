use serde::{Deserialize, Serialize};

use crate::field::{
    WebPageArchivedAtFieldEnum, WebPageAuthorFieldEnum, WebPageBreadcrumbFieldEnum,
    WebPageCitationFieldEnum, WebPageContributorFieldEnum, WebPageCreatorFieldEnum,
    WebPageDescriptionFieldEnum, WebPageImageFieldEnum, WebPageInLanguageFieldEnum,
    WebPageIsBasedOnFieldEnum, WebPageIsBasedOnUrlFieldEnum, WebPageKeywordsFieldEnum,
    WebPageMaintainerFieldEnum, WebPagePublishingPrinciplesFieldEnum, WebPageReviewedByFieldEnum,
    WebPageSubjectOfFieldEnum, WebPageTranslatorFieldEnum, WebPageVideoFieldEnum,
};
use crate::helpers::{one_or_many, one_or_many_string_or_num};
use crate::{
    Comment, CreativeWork, ImageObject, InteractionCounter, MediaObject, Thing, WebPageElement,
};

///<https://schema.org/dateCreated>
///<https://schema.org/Date>
///<https://schema.org/DateTime>
pub type WebPageDateCreatedFieldEnum = String;
///<https://schema.org/dateModified>
///<https://schema.org/Date>
///<https://schema.org/DateTime>
pub type WebPageDateModifiedFieldEnum = String;
///<https://schema.org/datePublished>
///<https://schema.org/Date>
///<https://schema.org/DateTime>
pub type WebPageDatePublishedFieldEnum = String;

///<https://schema.org/WebPage>
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct WebPage {
    #[serde(rename = "@type")]
    pub r#type: Option<String>,
    #[serde(rename = "@context")]
    pub context: Option<String>,
    ///<https://schema.org/mainContentOfPage>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub main_content_of_page: Vec<WebPageElement>,
    ///<https://schema.org/reviewedBy>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub reviewed_by: Vec<WebPageReviewedByFieldEnum>,
    ///<https://schema.org/significantLink>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub significant_link: Vec<String>,
    ///<https://schema.org/breadcrumb>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub breadcrumb: Vec<WebPageBreadcrumbFieldEnum>,
    ///<https://schema.org/lastReviewed>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub last_reviewed: Vec<String>,
    ///<https://schema.org/primaryImageOfPage>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub primary_image_of_page: Vec<ImageObject>,
    ///<https://schema.org/relatedLink>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub related_link: Vec<String>,
    ///<https://schema.org/comment>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<Comment>,
    ///<https://schema.org/isBasedOnUrl>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub is_based_on_url: Vec<WebPageIsBasedOnUrlFieldEnum>,
    ///<https://schema.org/translationOfWork>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub translation_of_work: Vec<CreativeWork>,
    ///<https://schema.org/workTranslation>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub work_translation: Vec<CreativeWork>,
    ///<https://schema.org/dateCreated>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub date_created: Vec<WebPageDateCreatedFieldEnum>,
    ///<https://schema.org/wordCount>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub word_count: Vec<i32>,
    ///<https://schema.org/maintainer>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub maintainer: Vec<WebPageMaintainerFieldEnum>,
    ///<https://schema.org/commentCount>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub comment_count: Vec<i32>,
    ///<https://schema.org/contributor>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub contributor: Vec<WebPageContributorFieldEnum>,
    ///<https://schema.org/temporalCoverage>
    ///<https://schema.org/interactionStatistic>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub interaction_statistic: Vec<InteractionCounter>,
    ///<https://schema.org/mainEntity>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub main_entity: Vec<Thing>,
    ///<https://schema.org/creditText>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub credit_text: Vec<String>,
    ///<https://schema.org/headline>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub headline: Vec<String>,
    ///<https://schema.org/dateModified>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub date_modified: Vec<WebPageDateModifiedFieldEnum>,
    ///<https://schema.org/keywords>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub keywords: Vec<WebPageKeywordsFieldEnum>,
    ///<https://schema.org/creator>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub creator: Vec<WebPageCreatorFieldEnum>,
    ///<https://schema.org/sdDatePublished>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sd_date_published: Vec<String>,
    ///<https://schema.org/archivedAt>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub archived_at: Vec<WebPageArchivedAtFieldEnum>,
    ///<https://schema.org/text>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub text: Vec<String>,
    ///<https://schema.org/associatedMedia>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub associated_media: Vec<MediaObject>,
    ///<https://schema.org/isBasedOn>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub is_based_on: Vec<WebPageIsBasedOnFieldEnum>,
    ///<https://schema.org/inLanguage>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub in_language: Vec<WebPageInLanguageFieldEnum>,
    ///<https://schema.org/datePublished>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub date_published: Vec<WebPageDatePublishedFieldEnum>,
    ///<https://schema.org/alternativeHeadline>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub alternative_headline: Vec<String>,
    ///<https://schema.org/thumbnail>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub thumbnail: Vec<ImageObject>,
    ///<https://schema.org/publishingPrinciples>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub publishing_principles: Vec<WebPagePublishingPrinciplesFieldEnum>,
    ///<https://schema.org/thumbnailUrl>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub thumbnail_url: Vec<String>,
    ///<https://schema.org/copyrightYear>
    #[serde(default, deserialize_with = "one_or_many_string_or_num")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub copyright_year: Vec<i32>,
    ///<https://schema.org/citation>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub citation: Vec<WebPageCitationFieldEnum>,
    ///<https://schema.org/video>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub video: Vec<WebPageVideoFieldEnum>,
    ///<https://schema.org/author>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub author: Vec<WebPageAuthorFieldEnum>,
    ///<https://schema.org/translator>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub translator: Vec<WebPageTranslatorFieldEnum>,
    ///<https://schema.org/disambiguatingDescription>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub disambiguating_description: Vec<String>,
    ///<https://schema.org/image>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub image: Vec<WebPageImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub description: Vec<WebPageDescriptionFieldEnum>,
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
    pub subject_of: Vec<WebPageSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub name: Vec<String>,
}
