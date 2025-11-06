use std::fmt::Formatter;

use schemars::JsonSchema;
use serde::{Deserialize, Deserializer};

use crate::thing::Place;
use crate::thing::intangible::ItemList;
use crate::thing::intangible::structured_value::contact_point::PostalAddress;
use crate::thing::intangible::virtual_location::VirtualLocation;

#[derive(Debug, PartialEq, JsonSchema)]
pub enum PlaceOrPostalAddressOrTextOrVirtualLocation {
    Place(Place),
    PostalAddress(PostalAddress),
    Text(String),
    VirtualLocation(VirtualLocation),
}

impl Default for PlaceOrPostalAddressOrTextOrVirtualLocation {
    fn default() -> Self {
        Self::Text(Default::default())
    }
}

impl<'de> Deserialize<'de> for PlaceOrPostalAddressOrTextOrVirtualLocation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PlaceOrPostalAddressOrTextOrVirtualLocation;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                todo!()
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

#[derive(Debug, PartialEq, JsonSchema)]
pub enum ItemListOrPlace {
    ItemList(ItemList),
    Place(Place),
}
