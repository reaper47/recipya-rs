use crate::*;
use serde_with::{serde_as, OneOrMany};
///<https://schema.org/checkinTime>
///<https://schema.org/DateTime>
///<https://schema.org/Time>
pub type LodgingReservationCheckinTimeFieldEnum = String;
///<https://schema.org/checkoutTime>
///<https://schema.org/DateTime>
///<https://schema.org/Time>
pub type LodgingReservationCheckoutTimeFieldEnum = String;
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type LodgingReservationAdditionalTypeFieldEnum = String;
///<https://schema.org/LodgingReservation>
#[serde_as]
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct LodgingReservation {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/checkinTime>
    #[serde(rename = "checkinTime")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub checkin_time: Vec<LodgingReservationCheckinTimeFieldEnum>,
    ///<https://schema.org/numAdults>
    #[serde(rename = "numAdults")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub num_adults: Vec<LodgingReservationNumAdultsFieldEnum>,
    ///<https://schema.org/lodgingUnitDescription>
    #[serde(rename = "lodgingUnitDescription")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub lodging_unit_description: Vec<String>,
    ///<https://schema.org/checkoutTime>
    #[serde(rename = "checkoutTime")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub checkout_time: Vec<LodgingReservationCheckoutTimeFieldEnum>,
    ///<https://schema.org/lodgingUnitType>
    #[serde(rename = "lodgingUnitType")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub lodging_unit_type: Vec<LodgingReservationLodgingUnitTypeFieldEnum>,
    ///<https://schema.org/numChildren>
    #[serde(rename = "numChildren")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub num_children: Vec<LodgingReservationNumChildrenFieldEnum>,
    ///<https://schema.org/broker>
    #[serde(rename = "broker")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub broker: Vec<LodgingReservationBrokerFieldEnum>,
    ///<https://schema.org/programMembershipUsed>
    #[serde(rename = "programMembershipUsed")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub program_membership_used: Vec<ProgramMembership>,
    ///<https://schema.org/priceCurrency>
    #[serde(rename = "priceCurrency")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub price_currency: Vec<String>,
    ///<https://schema.org/reservedTicket>
    #[serde(rename = "reservedTicket")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub reserved_ticket: Vec<Ticket>,
    ///<https://schema.org/provider>
    #[serde(rename = "provider")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub provider: Vec<LodgingReservationProviderFieldEnum>,
    ///<https://schema.org/bookingTime>
    #[serde(rename = "bookingTime")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub booking_time: Vec<String>,
    ///<https://schema.org/modifiedTime>
    #[serde(rename = "modifiedTime")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub modified_time: Vec<String>,
    ///<https://schema.org/reservationId>
    #[serde(rename = "reservationId")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub reservation_id: Vec<String>,
    ///<https://schema.org/totalPrice>
    #[serde(rename = "totalPrice")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub total_price: Vec<LodgingReservationTotalPriceFieldEnum>,
    ///<https://schema.org/reservationStatus>
    #[serde(rename = "reservationStatus")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub reservation_status: Vec<ReservationStatusTypeEnum>,
    ///<https://schema.org/reservationFor>
    #[serde(rename = "reservationFor")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub reservation_for: Vec<Thing>,
    ///<https://schema.org/bookingAgent>
    #[serde(rename = "bookingAgent")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub booking_agent: Vec<LodgingReservationBookingAgentFieldEnum>,
    ///<https://schema.org/underName>
    #[serde(rename = "underName")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub under_name: Vec<LodgingReservationUnderNameFieldEnum>,
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
    pub additional_type: Vec<LodgingReservationAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(rename = "identifier")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub identifier: Vec<LodgingReservationIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(rename = "image")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub image: Vec<LodgingReservationImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(rename = "sameAs")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(rename = "description")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub description: Vec<LodgingReservationDescriptionFieldEnum>,
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
    pub subject_of: Vec<LodgingReservationSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(rename = "name")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(rename = "mainEntityOfPage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub main_entity_of_page: Vec<LodgingReservationMainEntityOfPageFieldEnum>,
}
