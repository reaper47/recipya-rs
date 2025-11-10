use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::helpers::one_or_many;
use crate::field::*;
///<https://schema.org/cvdCollectionDate>
///<https://schema.org/DateTime>
///<https://schema.org/Text>
pub type CDCPMDRecordCvdCollectionDateFieldEnum = String;
///<https://schema.org/datePosted>
///<https://schema.org/Date>
///<https://schema.org/DateTime>
pub type CDCPMDRecordDatePostedFieldEnum = String;
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type CDCPMDRecordAdditionalTypeFieldEnum = String;
///<https://schema.org/CDCPMDRecord>
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CDCPMDRecord {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/cvdNumC19HOPats>
    #[serde(rename = "cvdNumC19HOPats")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cvd_num_c19ho_pats: Vec<f32>,
    ///<https://schema.org/cvdNumC19MechVentPats>
    #[serde(rename = "cvdNumC19MechVentPats")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cvd_num_c19_mech_vent_pats: Vec<f32>,
    ///<https://schema.org/cvdFacilityId>
    #[serde(rename = "cvdFacilityId")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cvd_facility_id: Vec<String>,
    ///<https://schema.org/cvdFacilityCounty>
    #[serde(rename = "cvdFacilityCounty")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cvd_facility_county: Vec<String>,
    ///<https://schema.org/cvdCollectionDate>
    #[serde(rename = "cvdCollectionDate")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cvd_collection_date: Vec<CDCPMDRecordCvdCollectionDateFieldEnum>,
    ///<https://schema.org/cvdNumTotBeds>
    #[serde(rename = "cvdNumTotBeds")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cvd_num_tot_beds: Vec<f32>,
    ///<https://schema.org/datePosted>
    #[serde(rename = "datePosted")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub date_posted: Vec<CDCPMDRecordDatePostedFieldEnum>,
    ///<https://schema.org/cvdNumC19OFMechVentPats>
    #[serde(rename = "cvdNumC19OFMechVentPats")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cvd_num_c19of_mech_vent_pats: Vec<f32>,
    ///<https://schema.org/cvdNumICUBeds>
    #[serde(rename = "cvdNumICUBeds")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cvd_num_icu_beds: Vec<f32>,
    ///<https://schema.org/cvdNumBeds>
    #[serde(rename = "cvdNumBeds")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cvd_num_beds: Vec<f32>,
    ///<https://schema.org/cvdNumC19Died>
    #[serde(rename = "cvdNumC19Died")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cvd_num_c19_died: Vec<f32>,
    ///<https://schema.org/cvdNumC19HospPats>
    #[serde(rename = "cvdNumC19HospPats")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cvd_num_c19_hosp_pats: Vec<f32>,
    ///<https://schema.org/cvdNumC19OverflowPats>
    #[serde(rename = "cvdNumC19OverflowPats")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cvd_num_c19_overflow_pats: Vec<f32>,
    ///<https://schema.org/cvdNumVentUse>
    #[serde(rename = "cvdNumVentUse")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cvd_num_vent_use: Vec<f32>,
    ///<https://schema.org/cvdNumBedsOcc>
    #[serde(rename = "cvdNumBedsOcc")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cvd_num_beds_occ: Vec<f32>,
    ///<https://schema.org/cvdNumVent>
    #[serde(rename = "cvdNumVent")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cvd_num_vent: Vec<f32>,
    ///<https://schema.org/cvdNumICUBedsOcc>
    #[serde(rename = "cvdNumICUBedsOcc")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cvd_num_icu_beds_occ: Vec<f32>,
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
    pub additional_type: Vec<CDCPMDRecordAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(rename = "identifier")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<CDCPMDRecordIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(rename = "image")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub image: Vec<CDCPMDRecordImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(rename = "sameAs")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(rename = "description")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub description: Vec<CDCPMDRecordDescriptionFieldEnum>,
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
    pub subject_of: Vec<CDCPMDRecordSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(rename = "name")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(rename = "mainEntityOfPage")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub main_entity_of_page: Vec<CDCPMDRecordMainEntityOfPageFieldEnum>,
}
