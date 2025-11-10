use crate::*;
use serde_with::{serde_as, OneOrMany};
///<https://schema.org/arrivalTime>
///<https://schema.org/DateTime>
///<https://schema.org/Time>
pub type FlightArrivalTimeFieldEnum = String;
///<https://schema.org/departureTime>
///<https://schema.org/DateTime>
///<https://schema.org/Time>
pub type FlightDepartureTimeFieldEnum = String;
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type FlightAdditionalTypeFieldEnum = String;
///<https://schema.org/Flight>
#[serde_as]
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct Flight {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/webCheckinTime>
    #[serde(rename = "webCheckinTime")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub web_checkin_time: Vec<String>,
    ///<https://schema.org/departureGate>
    #[serde(rename = "departureGate")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub departure_gate: Vec<String>,
    ///<https://schema.org/estimatedFlightDuration>
    #[serde(rename = "estimatedFlightDuration")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub estimated_flight_duration: Vec<FlightEstimatedFlightDurationFieldEnum>,
    ///<https://schema.org/arrivalTerminal>
    #[serde(rename = "arrivalTerminal")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub arrival_terminal: Vec<String>,
    ///<https://schema.org/arrivalGate>
    #[serde(rename = "arrivalGate")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub arrival_gate: Vec<String>,
    ///<https://schema.org/flightDistance>
    #[serde(rename = "flightDistance")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub flight_distance: Vec<FlightFlightDistanceFieldEnum>,
    ///<https://schema.org/flightNumber>
    #[serde(rename = "flightNumber")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub flight_number: Vec<String>,
    ///<https://schema.org/departureTerminal>
    #[serde(rename = "departureTerminal")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub departure_terminal: Vec<String>,
    ///<https://schema.org/aircraft>
    #[serde(rename = "aircraft")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub aircraft: Vec<FlightAircraftFieldEnum>,
    ///<https://schema.org/boardingPolicy>
    #[serde(rename = "boardingPolicy")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub boarding_policy: Vec<BoardingPolicyTypeEnum>,
    ///<https://schema.org/seller>
    #[serde(rename = "seller")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub seller: Vec<FlightSellerFieldEnum>,
    ///<https://schema.org/mealService>
    #[serde(rename = "mealService")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub meal_service: Vec<String>,
    ///<https://schema.org/arrivalAirport>
    #[serde(rename = "arrivalAirport")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub arrival_airport: Vec<Airport>,
    ///<https://schema.org/carrier>
    #[serde(rename = "carrier")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub carrier: Vec<Organization>,
    ///<https://schema.org/departureAirport>
    #[serde(rename = "departureAirport")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub departure_airport: Vec<Airport>,
    ///<https://schema.org/offers>
    #[serde(rename = "offers")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub offers: Vec<FlightOffersFieldEnum>,
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
    pub provider: Vec<FlightProviderFieldEnum>,
    ///<https://schema.org/arrivalTime>
    #[serde(rename = "arrivalTime")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub arrival_time: Vec<FlightArrivalTimeFieldEnum>,
    ///<https://schema.org/subTrip>
    #[serde(rename = "subTrip")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub sub_trip: Vec<Trip>,
    ///<https://schema.org/departureTime>
    #[serde(rename = "departureTime")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub departure_time: Vec<FlightDepartureTimeFieldEnum>,
    ///<https://schema.org/itinerary>
    #[serde(rename = "itinerary")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub itinerary: Vec<FlightItineraryFieldEnum>,
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
    pub additional_type: Vec<FlightAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(rename = "identifier")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub identifier: Vec<FlightIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(rename = "image")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub image: Vec<FlightImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(rename = "sameAs")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(rename = "description")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub description: Vec<FlightDescriptionFieldEnum>,
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
    pub subject_of: Vec<FlightSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(rename = "name")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(rename = "mainEntityOfPage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub main_entity_of_page: Vec<FlightMainEntityOfPageFieldEnum>,
}
