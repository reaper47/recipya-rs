use schemars::JsonSchema;
use serde::Deserialize;

use crate::thing::intangible::structured_value::GeoShape;
use crate::thing::place::{AdministrativeArea, Country, Place};

#[derive(Debug, Deserialize, PartialEq, JsonSchema)]
pub enum CountryOrText {
    Country(Country),
    Text(String),
}

#[derive(Debug, Deserialize, PartialEq, JsonSchema)]
pub enum AdministrativeAreaOrGeoShapeOrPlaceOrText {
    AdministrativeArea(AdministrativeArea),
    GeoShape(GeoShape),
    Place(Place),
    Text(String),
}
