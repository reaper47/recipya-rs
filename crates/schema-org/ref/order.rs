use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::helpers::one_or_many;
use crate::field::*;
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
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Order {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/discount>
    #[serde(rename = "discount")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub discount: Vec<OrderDiscountFieldEnum>,
    ///<https://schema.org/broker>
    #[serde(rename = "broker")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub broker: Vec<OrderBrokerFieldEnum>,
    ///<https://schema.org/orderedItem>
    #[serde(rename = "orderedItem")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ordered_item: Vec<OrderOrderedItemFieldEnum>,
    ///<https://schema.org/paymentMethodId>
    #[serde(rename = "paymentMethodId")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub payment_method_id: Vec<String>,
    ///<https://schema.org/orderDelivery>
    #[serde(rename = "orderDelivery")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub order_delivery: Vec<ParcelDelivery>,
    ///<https://schema.org/isGift>
    #[serde(rename = "isGift")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub is_gift: Vec<String>,
    ///<https://schema.org/paymentUrl>
    #[serde(rename = "paymentUrl")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub payment_url: Vec<String>,
    ///<https://schema.org/orderNumber>
    #[serde(rename = "orderNumber")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub order_number: Vec<String>,
    ///<https://schema.org/orderDate>
    #[serde(rename = "orderDate")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub order_date: Vec<OrderOrderDateFieldEnum>,
    ///<https://schema.org/orderStatus>
    #[serde(rename = "orderStatus")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub order_status: Vec<OrderStatusEnum>,
    ///<https://schema.org/discountCode>
    #[serde(rename = "discountCode")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub discount_code: Vec<String>,
    ///<https://schema.org/partOfInvoice>
    #[serde(rename = "partOfInvoice")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub part_of_invoice: Vec<Invoice>,
    ///<https://schema.org/acceptedOffer>
    #[serde(rename = "acceptedOffer")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub accepted_offer: Vec<Offer>,
    ///<https://schema.org/discountCurrency>
    #[serde(rename = "discountCurrency")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub discount_currency: Vec<String>,
    ///<https://schema.org/paymentDueDate>
    #[serde(rename = "paymentDueDate")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub payment_due_date: Vec<OrderPaymentDueDateFieldEnum>,
    ///<https://schema.org/billingAddress>
    #[serde(rename = "billingAddress")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub billing_address: Vec<PostalAddress>,
    ///<https://schema.org/paymentDue>
    #[serde(rename = "paymentDue")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub payment_due: Vec<String>,
    ///<https://schema.org/customer>
    #[serde(rename = "customer")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub customer: Vec<OrderCustomerFieldEnum>,
    ///<https://schema.org/seller>
    #[serde(rename = "seller")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub seller: Vec<OrderSellerFieldEnum>,
    ///<https://schema.org/confirmationNumber>
    #[serde(rename = "confirmationNumber")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub confirmation_number: Vec<String>,
    ///<https://schema.org/merchant>
    #[serde(rename = "merchant")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub merchant: Vec<OrderMerchantFieldEnum>,
    ///<https://schema.org/paymentMethod>
    #[serde(rename = "paymentMethod")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub payment_method: Vec<OrderPaymentMethodFieldEnum>,
    ///<https://schema.org/disambiguatingDescription>
    #[serde(rename = "disambiguatingDescription")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub disambiguating_description: Vec<String>,
    ///<https://schema.org/potentialAction>
    #[serde(rename = "potentialAction")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub potential_action: Vec<Action>,
    ///<https://schema.org/additionalType>
    #[serde(rename = "additionalType")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub additional_type: Vec<OrderAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(rename = "identifier")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<OrderIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(rename = "image")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub image: Vec<OrderImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(rename = "sameAs")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(rename = "description")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub description: Vec<OrderDescriptionFieldEnum>,
    ///<https://schema.org/alternateName>
    #[serde(rename = "alternateName")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub alternate_name: Vec<String>,
    ///<https://schema.org/url>
    #[serde(rename = "url")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub url: Vec<String>,
    ///<https://schema.org/subjectOf>
    #[serde(rename = "subjectOf")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub subject_of: Vec<OrderSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(rename = "name")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(rename = "mainEntityOfPage")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub main_entity_of_page: Vec<OrderMainEntityOfPageFieldEnum>,
}
