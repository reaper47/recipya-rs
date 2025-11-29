use serde::{Deserialize, Serialize};

use crate::enums::{MusicAlbumProductionTypeEnum, MusicAlbumReleaseTypeEnum};
use crate::field::{
    MusicAlbumArchivedAtFieldEnum, MusicAlbumAudioFieldEnum, MusicAlbumAuthorFieldEnum,
    MusicAlbumCitationFieldEnum, MusicAlbumCreatorFieldEnum, MusicAlbumDescriptionFieldEnum,
    MusicAlbumImageFieldEnum, MusicAlbumIsBasedOnFieldEnum, MusicAlbumIsBasedOnUrlFieldEnum,
    MusicAlbumIsPartOfFieldEnum, MusicAlbumKeywordsFieldEnum, MusicAlbumPublisherFieldEnum,
    MusicAlbumSdPublisherFieldEnum, MusicAlbumTrackFieldEnum,
};
use crate::helpers::one_or_many;
use crate::{
    AtType, Comment, Country, Duration, Event, ImageObject, InteractionCounter, MediaObject,
    MusicRecording, MusicRelease, Thing,
};

///<https://schema.org/dateCreated>
///<https://schema.org/Date>
///<https://schema.org/DateTime>
pub type MusicAlbumDateCreatedFieldEnum = String;
///<https://schema.org/fileFormat>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type MusicAlbumFileFormatFieldEnum = String;
///<https://schema.org/dateModified>
///<https://schema.org/Date>
///<https://schema.org/DateTime>
pub type MusicAlbumDateModifiedFieldEnum = String;
///<https://schema.org/datePublished>
///<https://schema.org/Date>
///<https://schema.org/DateTime>
pub type MusicAlbumDatePublishedFieldEnum = String;
///<https://schema.org/genre>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type MusicAlbumGenreFieldEnum = String;

///<https://schema.org/MusicAlbum>
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct MusicAlbum {
    #[serde(rename = "@type", default = "set_type")]
    pub r#type: Option<String>,
    #[serde(rename = "@context")]
    pub context: Option<String>,
    ///<https://schema.org/albumRelease>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub album_release: Vec<MusicRelease>,
    ///<https://schema.org/albumReleaseType>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub album_release_type: Vec<MusicAlbumReleaseTypeEnum>,
    ///<https://schema.org/albumProductionType>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub album_production_type: Vec<MusicAlbumProductionTypeEnum>,
    ///<https://schema.org/track>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub track: Vec<MusicAlbumTrackFieldEnum>,
    ///<https://schema.org/numTracks>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub num_tracks: Vec<i32>,
    ///<https://schema.org/tracks>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tracks: Vec<MusicRecording>,
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
    pub is_based_on_url: Vec<MusicAlbumIsBasedOnUrlFieldEnum>,
    ///<https://schema.org/dateCreated>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub date_created: Vec<MusicAlbumDateCreatedFieldEnum>,
    ///<https://schema.org/commentCount>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub comment_count: Vec<i32>,
    ///<https://schema.org/timeRequired>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub time_required: Vec<Duration>,
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
    pub publisher: Vec<MusicAlbumPublisherFieldEnum>,
    ///<https://schema.org/creditText>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub credit_text: Vec<String>,
    ///<https://schema.org/headline>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub headline: Vec<String>,
    ///<https://schema.org/fileFormat>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub file_format: Vec<MusicAlbumFileFormatFieldEnum>,
    ///<https://schema.org/dateModified>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub date_modified: Vec<MusicAlbumDateModifiedFieldEnum>,
    ///<https://schema.org/keywords>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub keywords: Vec<MusicAlbumKeywordsFieldEnum>,
    ///<https://schema.org/creator>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub creator: Vec<MusicAlbumCreatorFieldEnum>,
    ///<https://schema.org/sdDatePublished>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sd_date_published: Vec<String>,
    ///<https://schema.org/archivedAt>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub archived_at: Vec<MusicAlbumArchivedAtFieldEnum>,
    ///<https://schema.org/discussionUrl>
    ///<https://schema.org/countryOfOrigin>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub country_of_origin: Vec<Country>,
    ///<https://schema.org/text>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub text: Vec<String>,
    ///<https://schema.org/associatedMedia>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub associated_media: Vec<MediaObject>,
    ///<https://schema.org/award>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub award: Vec<String>,
    ///<https://schema.org/isBasedOn>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub is_based_on: Vec<MusicAlbumIsBasedOnFieldEnum>,
    ///<https://schema.org/datePublished>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub date_published: Vec<MusicAlbumDatePublishedFieldEnum>,
    ///<https://schema.org/sdPublisher>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sd_publisher: Vec<MusicAlbumSdPublisherFieldEnum>,
    ///<https://schema.org/genre>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub genre: Vec<MusicAlbumGenreFieldEnum>,
    ///<https://schema.org/audio>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub audio: Vec<MusicAlbumAudioFieldEnum>,
    ///<https://schema.org/alternativeHeadline>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub alternative_headline: Vec<String>,
    ///<https://schema.org/about>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub about: Vec<Thing>,
    ///<https://schema.org/isPartOf>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub is_part_of: Vec<MusicAlbumIsPartOfFieldEnum>,
    ///<https://schema.org/thumbnail>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub thumbnail: Vec<ImageObject>,
    ///<https://schema.org/thumbnailUrl>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub thumbnail_url: Vec<String>,
    ///<https://schema.org/copyrightYear>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub copyright_year: Vec<f32>,
    ///<https://schema.org/citation>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub citation: Vec<MusicAlbumCitationFieldEnum>,
    ///<https://schema.org/awards>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub awards: Vec<String>,
    ///<https://schema.org/author>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub author: Vec<MusicAlbumAuthorFieldEnum>,
    ///<https://schema.org/disambiguatingDescription>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub disambiguating_description: Vec<String>,
    ///<https://schema.org/image>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub image: Vec<MusicAlbumImageFieldEnum>,
    ///<https://schema.org/description>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub description: Vec<MusicAlbumDescriptionFieldEnum>,
    ///<https://schema.org/alternateName>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub alternate_name: Vec<String>,
    ///<https://schema.org/url>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub url: Vec<String>,
    ///<https://schema.org/name>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub name: Vec<String>,
}

fn set_type() -> Option<String> {
    Some(AtType::MusicAlbum.to_string())
}
