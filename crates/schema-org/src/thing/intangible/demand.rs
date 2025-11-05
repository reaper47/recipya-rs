use schemars::JsonSchema;
use serde::Deserialize;

/// A demand entity represents the public, not necessarily binding, not necessarily exclusive,
/// announcement by an organization or person to seek a certain type of goods or services. For
/// describing demand using this type, the very same properties used for Offer apply.
#[derive(Debug, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Demand {
    /// The payment method(s) that are accepted in general by an organization, or for some specific
    /// demand or offer.
    pub accepted_payment_method: LoanOrCreditOrPaymentMethodOrText,
    /// The amount of time that is required between accepting the offer and the actual usage of the
    /// resource or service.
    pub advance_booking_requirement: QuantitativeValue,
}
