use crate::*;
use serde_with::{serde_as, OneOrMany};
///<https://schema.org/ticketToken>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type TicketTicketTokenFieldEnum = String;
///<https://schema.org/dateIssued>
///<https://schema.org/Date>
///<https://schema.org/DateTime>
pub type TicketDateIssuedFieldEnum = String;
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type TicketAdditionalTypeFieldEnum = String;
///<https://schema.org/Ticket>
#[serde_as]
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct Ticket {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/issuedBy>
    #[serde(rename = "issuedBy")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub issued_by: Vec<Organization>,
    ///<https://schema.org/ticketedSeat>
    #[serde(rename = "ticketedSeat")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub ticketed_seat: Vec<Seat>,
    ///<https://schema.org/ticketToken>
    #[serde(rename = "ticketToken")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub ticket_token: Vec<TicketTicketTokenFieldEnum>,
    ///<https://schema.org/priceCurrency>
    #[serde(rename = "priceCurrency")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub price_currency: Vec<String>,
    ///<https://schema.org/ticketNumber>
    #[serde(rename = "ticketNumber")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub ticket_number: Vec<String>,
    ///<https://schema.org/totalPrice>
    #[serde(rename = "totalPrice")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub total_price: Vec<TicketTotalPriceFieldEnum>,
    ///<https://schema.org/dateIssued>
    #[serde(rename = "dateIssued")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub date_issued: Vec<TicketDateIssuedFieldEnum>,
    ///<https://schema.org/underName>
    #[serde(rename = "underName")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub under_name: Vec<TicketUnderNameFieldEnum>,
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
    pub additional_type: Vec<TicketAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(rename = "identifier")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub identifier: Vec<TicketIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(rename = "image")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub image: Vec<TicketImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(rename = "sameAs")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(rename = "description")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub description: Vec<TicketDescriptionFieldEnum>,
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
    pub subject_of: Vec<TicketSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(rename = "name")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(rename = "mainEntityOfPage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub main_entity_of_page: Vec<TicketMainEntityOfPageFieldEnum>,
}
