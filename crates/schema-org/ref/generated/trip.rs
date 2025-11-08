use crate::*;
use serde_with::{serde_as, OneOrMany};
///<https://schema.org/arrivalTime>
///<https://schema.org/DateTime>
///<https://schema.org/Time>
pub type TripArrivalTimeFieldEnum = String;
///<https://schema.org/departureTime>
///<https://schema.org/DateTime>
///<https://schema.org/Time>
pub type TripDepartureTimeFieldEnum = String;
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type TripAdditionalTypeFieldEnum = String;
///<https://schema.org/Trip>
#[serde_as]
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct Trip {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/offers>
    #[serde(rename = "offers")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub offers: Vec<TripOffersFieldEnum>,
    ///<https://schema.org/tripOrigin>
    #[serde(rename = "tripOrigin")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub trip_origin: Vec<Place>,
    ///<https://schema.org/partOfTrip>
    #[serde(rename = "partOfTrip")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub part_of_trip: Vec<Trip>,
    ///<https://schema.org/provider>
    #[serde(rename = "provider")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub provider: Vec<TripProviderFieldEnum>,
    ///<https://schema.org/arrivalTime>
    #[serde(rename = "arrivalTime")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub arrival_time: Vec<TripArrivalTimeFieldEnum>,
    ///<https://schema.org/subTrip>
    #[serde(rename = "subTrip")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub sub_trip: Vec<Trip>,
    ///<https://schema.org/departureTime>
    #[serde(rename = "departureTime")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub departure_time: Vec<TripDepartureTimeFieldEnum>,
    ///<https://schema.org/itinerary>
    #[serde(rename = "itinerary")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub itinerary: Vec<TripItineraryFieldEnum>,
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
    pub additional_type: Vec<TripAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(rename = "identifier")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub identifier: Vec<TripIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(rename = "image")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub image: Vec<TripImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(rename = "sameAs")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(rename = "description")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub description: Vec<TripDescriptionFieldEnum>,
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
    pub subject_of: Vec<TripSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(rename = "name")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(rename = "mainEntityOfPage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub main_entity_of_page: Vec<TripMainEntityOfPageFieldEnum>,
}
