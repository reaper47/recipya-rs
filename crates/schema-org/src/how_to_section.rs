use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

use crate::field::{
    HowToSectionDescriptionFieldEnum, HowToSectionImageFieldEnum, HowToSectionInLanguageFieldEnum,
    HowToSectionIsBasedOnFieldEnum, HowToSectionIsBasedOnUrlFieldEnum,
    HowToSectionItemListElementFieldEnum, HowToSectionItemListOrderFieldEnum,
    HowToSectionKeywordsFieldEnum, HowToSectionPositionFieldEnum, HowToSectionSizeFieldEnum,
    HowToSectionStepsFieldEnum, HowToSectionSubjectOfFieldEnum, HowToSectionUsageInfoFieldEnum,
    HowToSectionVideoFieldEnum,
};
use crate::helpers::{is_smallvec_empty, one_or_many};
use crate::{Comment, CreativeWork, Duration, ImageObject, InteractionCounter, ListItem, Thing};

///<https://schema.org/HowToSection>
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct HowToSection {
    #[serde(rename = "@type")]
    pub r#type: Option<String>,
    #[serde(rename = "@context")]
    pub context: Option<String>,
    ///<https://schema.org/steps>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub steps: SmallVec<[HowToSectionStepsFieldEnum; 6]>,
    ///<https://schema.org/numberOfItems>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub number_of_items: SmallVec<[i32; 1]>,
    ///<https://schema.org/itemListElement>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub item_list_element: SmallVec<[HowToSectionItemListElementFieldEnum; 6]>,
    ///<https://schema.org/aggregateElement>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub aggregate_element: SmallVec<[Thing; 1]>,
    ///<https://schema.org/itemListOrder>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub item_list_order: SmallVec<[HowToSectionItemListOrderFieldEnum; 1]>,
    ///<https://schema.org/disambiguatingDescription>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub disambiguating_description: SmallVec<[String; 1]>,
    ///<https://schema.org/image>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub image: SmallVec<[HowToSectionImageFieldEnum; 2]>,
    ///<https://schema.org/description>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub description: SmallVec<[HowToSectionDescriptionFieldEnum; 1]>,
    ///<https://schema.org/alternateName>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub alternate_name: SmallVec<[String; 1]>,
    ///<https://schema.org/url>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub url: SmallVec<[String; 1]>,
    ///<https://schema.org/subjectOf>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub subject_of: SmallVec<[HowToSectionSubjectOfFieldEnum; 1]>,
    ///<https://schema.org/name>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub name: SmallVec<[String; 1]>,
    ///<https://schema.org/item>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub item: SmallVec<[Thing; 1]>,
    ///<https://schema.org/nextItem>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub next_item: SmallVec<[ListItem; 1]>,
    ///<https://schema.org/previousItem>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub previous_item: SmallVec<[ListItem; 1]>,
    ///<https://schema.org/position>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub position: SmallVec<[HowToSectionPositionFieldEnum; 1]>,
    ///<https://schema.org/comment>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub comment: SmallVec<[Comment; 4]>,
    ///<https://schema.org/isBasedOnUrl>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub is_based_on_url: SmallVec<[HowToSectionIsBasedOnUrlFieldEnum; 1]>,
    ///<https://schema.org/translationOfWork>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub translation_of_work: SmallVec<[CreativeWork; 1]>,
    ///<https://schema.org/workTranslation>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub work_translation: SmallVec<[CreativeWork; 1]>,
    ///<https://schema.org/wordCount>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub word_count: SmallVec<[i32; 1]>,
    ///<https://schema.org/size>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub size: SmallVec<[Box<HowToSectionSizeFieldEnum>; 1]>,
    ///<https://schema.org/timeRequired>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub time_required: SmallVec<[Duration; 1]>,
    ///<https://schema.org/interactionStatistic>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub interaction_statistic: SmallVec<[InteractionCounter; 1]>,
    ///<https://schema.org/headline>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub headline: SmallVec<[String; 1]>,
    ///<https://schema.org/keywords>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub keywords: SmallVec<[HowToSectionKeywordsFieldEnum; 4]>,
    ///<https://schema.org/text>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub text: SmallVec<[String; 1]>,
    ///<https://schema.org/isBasedOn>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub is_based_on: SmallVec<[HowToSectionIsBasedOnFieldEnum; 1]>,
    ///<https://schema.org/inLanguage>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub in_language: SmallVec<[HowToSectionInLanguageFieldEnum; 1]>,
    ///<https://schema.org/usageInfo>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub usage_info: SmallVec<[HowToSectionUsageInfoFieldEnum; 1]>,
    ///<https://schema.org/alternativeHeadline>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub alternative_headline: SmallVec<[String; 1]>,
    ///<https://schema.org/thumbnail>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub thumbnail: SmallVec<[ImageObject; 1]>,
    ///<https://schema.org/thumbnailUrl>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub thumbnail_url: SmallVec<[String; 1]>,
    ///<https://schema.org/video>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub video: SmallVec<[HowToSectionVideoFieldEnum; 1]>,
}
