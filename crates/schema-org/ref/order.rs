use crate::*;
use serde_with::{serde_as, OneOrMany};
///<https://schema.org/orderDate>
///<https://schema.org/Date>
///<https://schema.org/DateTime>
pub type OrderOrderDateFieldEnum = String;
///<https://schema.org/paymentDueDate>
///<https://schema.org/Date>
///<https://schema.org/DateTime>
pub type OrderPaymentDueDateFieldEnum = String;
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type OrderAdditionalTypeFieldEnum = String;
///<https://schema.org/Order>
#[serde_as]
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct Order {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/discount>
    #[serde(rename = "discount")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub discount: Vec<OrderDiscountFieldEnum>,
    ///<https://schema.org/broker>
    #[serde(rename = "broker")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub broker: Vec<OrderBrokerFieldEnum>,
    ///<https://schema.org/orderedItem>
    #[serde(rename = "orderedItem")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub ordered_item: Vec<OrderOrderedItemFieldEnum>,
    ///<https://schema.org/paymentMethodId>
    #[serde(rename = "paymentMethodId")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub payment_method_id: Vec<String>,
    ///<https://schema.org/orderDelivery>
    #[serde(rename = "orderDelivery")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub order_delivery: Vec<ParcelDelivery>,
    ///<https://schema.org/isGift>
    #[serde(rename = "isGift")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub is_gift: Vec<String>,
    ///<https://schema.org/paymentUrl>
    #[serde(rename = "paymentUrl")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub payment_url: Vec<String>,
    ///<https://schema.org/orderNumber>
    #[serde(rename = "orderNumber")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub order_number: Vec<String>,
    ///<https://schema.org/orderDate>
    #[serde(rename = "orderDate")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub order_date: Vec<OrderOrderDateFieldEnum>,
    ///<https://schema.org/orderStatus>
    #[serde(rename = "orderStatus")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub order_status: Vec<OrderStatusEnum>,
    ///<https://schema.org/discountCode>
    #[serde(rename = "discountCode")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub discount_code: Vec<String>,
    ///<https://schema.org/partOfInvoice>
    #[serde(rename = "partOfInvoice")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub part_of_invoice: Vec<Invoice>,
    ///<https://schema.org/acceptedOffer>
    #[serde(rename = "acceptedOffer")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub accepted_offer: Vec<Offer>,
    ///<https://schema.org/discountCurrency>
    #[serde(rename = "discountCurrency")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub discount_currency: Vec<String>,
    ///<https://schema.org/paymentDueDate>
    #[serde(rename = "paymentDueDate")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub payment_due_date: Vec<OrderPaymentDueDateFieldEnum>,
    ///<https://schema.org/billingAddress>
    #[serde(rename = "billingAddress")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub billing_address: Vec<PostalAddress>,
    ///<https://schema.org/paymentDue>
    #[serde(rename = "paymentDue")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub payment_due: Vec<String>,
    ///<https://schema.org/customer>
    #[serde(rename = "customer")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub customer: Vec<OrderCustomerFieldEnum>,
    ///<https://schema.org/seller>
    #[serde(rename = "seller")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub seller: Vec<OrderSellerFieldEnum>,
    ///<https://schema.org/confirmationNumber>
    #[serde(rename = "confirmationNumber")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub confirmation_number: Vec<String>,
    ///<https://schema.org/merchant>
    #[serde(rename = "merchant")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub merchant: Vec<OrderMerchantFieldEnum>,
    ///<https://schema.org/paymentMethod>
    #[serde(rename = "paymentMethod")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub payment_method: Vec<OrderPaymentMethodFieldEnum>,
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
    pub additional_type: Vec<OrderAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(rename = "identifier")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub identifier: Vec<OrderIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(rename = "image")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub image: Vec<OrderImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(rename = "sameAs")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(rename = "description")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub description: Vec<OrderDescriptionFieldEnum>,
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
    pub subject_of: Vec<OrderSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(rename = "name")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(rename = "mainEntityOfPage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub main_entity_of_page: Vec<OrderMainEntityOfPageFieldEnum>,
}
