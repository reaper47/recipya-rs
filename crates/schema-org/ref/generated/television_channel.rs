use crate::*;
use serde_with::{serde_as, OneOrMany};
///<https://schema.org/genre>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type TelevisionChannelGenreFieldEnum = String;
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type TelevisionChannelAdditionalTypeFieldEnum = String;
///<https://schema.org/TelevisionChannel>
#[serde_as]
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct TelevisionChannel {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/broadcastChannelId>
    #[serde(rename = "broadcastChannelId")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub broadcast_channel_id: Vec<String>,
    ///<https://schema.org/providesBroadcastService>
    #[serde(rename = "providesBroadcastService")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub provides_broadcast_service: Vec<BroadcastService>,
    ///<https://schema.org/broadcastServiceTier>
    #[serde(rename = "broadcastServiceTier")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub broadcast_service_tier: Vec<String>,
    ///<https://schema.org/genre>
    #[serde(rename = "genre")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub genre: Vec<TelevisionChannelGenreFieldEnum>,
    ///<https://schema.org/inBroadcastLineup>
    #[serde(rename = "inBroadcastLineup")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub in_broadcast_lineup: Vec<CableOrSatelliteService>,
    ///<https://schema.org/broadcastFrequency>
    #[serde(rename = "broadcastFrequency")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub broadcast_frequency: Vec<TelevisionChannelBroadcastFrequencyFieldEnum>,
    ///<https://schema.org/disambiguatingDescription>
    #[serde(rename = "disambiguatingDescription")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub disambiguating_description: Vec<String>,
    ///<https://schema.org/potentialAction>
    #[serde(rename = "potentialAction")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub potential_action: Vec<Action>,
    ///<https://schema.org/additionalType>
    #[serde(rename = "additionalType")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub additional_type: Vec<TelevisionChannelAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(rename = "identifier")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub identifier: Vec<TelevisionChannelIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(rename = "image")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub image: Vec<TelevisionChannelImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(rename = "sameAs")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(rename = "description")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub description: Vec<TelevisionChannelDescriptionFieldEnum>,
    ///<https://schema.org/alternateName>
    #[serde(rename = "alternateName")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub alternate_name: Vec<String>,
    ///<https://schema.org/url>
    #[serde(rename = "url")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub url: Vec<String>,
    ///<https://schema.org/subjectOf>
    #[serde(rename = "subjectOf")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub subject_of: Vec<TelevisionChannelSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(rename = "name")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(rename = "mainEntityOfPage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub main_entity_of_page: Vec<TelevisionChannelMainEntityOfPageFieldEnum>,
}
