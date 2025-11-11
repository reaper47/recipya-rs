use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::enums::RestrictedDietEnum;
use crate::helpers::one_or_many;
use crate::field::*;
use crate::{Action, NutritionInformation};

///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type MenuItemAdditionalTypeFieldEnum = String;
///<https://schema.org/MenuItem>
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct MenuItem {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/offers>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub offers: Vec<MenuItemOffersFieldEnum>,
    ///<https://schema.org/nutrition>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub nutrition: Vec<NutritionInformation>,
    ///<https://schema.org/menuAddOn>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub menu_add_on: Vec<MenuItemMenuAddOnFieldEnum>,
    ///<https://schema.org/suitableForDiet>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub suitable_for_diet: Vec<RestrictedDietEnum>,
    ///<https://schema.org/disambiguatingDescription>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub disambiguating_description: Vec<String>,
    ///<https://schema.org/potentialAction>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub potential_action: Vec<Action>,
    ///<https://schema.org/additionalType>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub additional_type: Vec<MenuItemAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<MenuItemIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub image: Vec<MenuItemImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub description: Vec<MenuItemDescriptionFieldEnum>,
    ///<https://schema.org/alternateName>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub alternate_name: Vec<String>,
    ///<https://schema.org/url>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub url: Vec<String>,
    ///<https://schema.org/subjectOf>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub subject_of: Vec<MenuItemSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub main_entity_of_page: Vec<MenuItemMainEntityOfPageFieldEnum>,
}
