use crate::*;
use serde_with::{serde_as, OneOrMany};
///<https://schema.org/paymentDueDate>
///<https://schema.org/Date>
///<https://schema.org/DateTime>
pub type InvoicePaymentDueDateFieldEnum = String;
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type InvoiceAdditionalTypeFieldEnum = String;
///<https://schema.org/Invoice>
#[serde_as]
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct Invoice {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/minimumPaymentDue>
    #[serde(rename = "minimumPaymentDue")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub minimum_payment_due: Vec<InvoiceMinimumPaymentDueFieldEnum>,
    ///<https://schema.org/broker>
    #[serde(rename = "broker")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub broker: Vec<InvoiceBrokerFieldEnum>,
    ///<https://schema.org/paymentMethodId>
    #[serde(rename = "paymentMethodId")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub payment_method_id: Vec<String>,
    ///<https://schema.org/billingPeriod>
    #[serde(rename = "billingPeriod")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub billing_period: Vec<Duration>,
    ///<https://schema.org/paymentStatus>
    #[serde(rename = "paymentStatus")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub payment_status: Vec<InvoicePaymentStatusFieldEnum>,
    ///<https://schema.org/provider>
    #[serde(rename = "provider")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub provider: Vec<InvoiceProviderFieldEnum>,
    ///<https://schema.org/totalPaymentDue>
    #[serde(rename = "totalPaymentDue")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub total_payment_due: Vec<InvoiceTotalPaymentDueFieldEnum>,
    ///<https://schema.org/paymentDueDate>
    #[serde(rename = "paymentDueDate")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub payment_due_date: Vec<InvoicePaymentDueDateFieldEnum>,
    ///<https://schema.org/paymentDue>
    #[serde(rename = "paymentDue")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub payment_due: Vec<String>,
    ///<https://schema.org/customer>
    #[serde(rename = "customer")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub customer: Vec<InvoiceCustomerFieldEnum>,
    ///<https://schema.org/referencesOrder>
    #[serde(rename = "referencesOrder")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub references_order: Vec<Order>,
    ///<https://schema.org/accountId>
    #[serde(rename = "accountId")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub account_id: Vec<String>,
    ///<https://schema.org/scheduledPaymentDate>
    #[serde(rename = "scheduledPaymentDate")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub scheduled_payment_date: Vec<String>,
    ///<https://schema.org/confirmationNumber>
    #[serde(rename = "confirmationNumber")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub confirmation_number: Vec<String>,
    ///<https://schema.org/category>
    #[serde(rename = "category")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub category: Vec<InvoiceCategoryFieldEnum>,
    ///<https://schema.org/paymentMethod>
    #[serde(rename = "paymentMethod")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub payment_method: Vec<InvoicePaymentMethodFieldEnum>,
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
    pub additional_type: Vec<InvoiceAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(rename = "identifier")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub identifier: Vec<InvoiceIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(rename = "image")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub image: Vec<InvoiceImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(rename = "sameAs")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(rename = "description")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub description: Vec<InvoiceDescriptionFieldEnum>,
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
    pub subject_of: Vec<InvoiceSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(rename = "name")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(rename = "mainEntityOfPage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub main_entity_of_page: Vec<InvoiceMainEntityOfPageFieldEnum>,
}
