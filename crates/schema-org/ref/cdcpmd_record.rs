use crate::*;
use serde_with::{serde_as, OneOrMany};
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
#[serde_as]
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct CDCPMDRecord {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/cvdNumC19HOPats>
    #[serde(rename = "cvdNumC19HOPats")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub cvd_num_c19ho_pats: Vec<f32>,
    ///<https://schema.org/cvdNumC19MechVentPats>
    #[serde(rename = "cvdNumC19MechVentPats")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub cvd_num_c19_mech_vent_pats: Vec<f32>,
    ///<https://schema.org/cvdFacilityId>
    #[serde(rename = "cvdFacilityId")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub cvd_facility_id: Vec<String>,
    ///<https://schema.org/cvdFacilityCounty>
    #[serde(rename = "cvdFacilityCounty")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub cvd_facility_county: Vec<String>,
    ///<https://schema.org/cvdCollectionDate>
    #[serde(rename = "cvdCollectionDate")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub cvd_collection_date: Vec<CDCPMDRecordCvdCollectionDateFieldEnum>,
    ///<https://schema.org/cvdNumTotBeds>
    #[serde(rename = "cvdNumTotBeds")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub cvd_num_tot_beds: Vec<f32>,
    ///<https://schema.org/datePosted>
    #[serde(rename = "datePosted")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub date_posted: Vec<CDCPMDRecordDatePostedFieldEnum>,
    ///<https://schema.org/cvdNumC19OFMechVentPats>
    #[serde(rename = "cvdNumC19OFMechVentPats")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub cvd_num_c19of_mech_vent_pats: Vec<f32>,
    ///<https://schema.org/cvdNumICUBeds>
    #[serde(rename = "cvdNumICUBeds")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub cvd_num_icu_beds: Vec<f32>,
    ///<https://schema.org/cvdNumBeds>
    #[serde(rename = "cvdNumBeds")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub cvd_num_beds: Vec<f32>,
    ///<https://schema.org/cvdNumC19Died>
    #[serde(rename = "cvdNumC19Died")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub cvd_num_c19_died: Vec<f32>,
    ///<https://schema.org/cvdNumC19HospPats>
    #[serde(rename = "cvdNumC19HospPats")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub cvd_num_c19_hosp_pats: Vec<f32>,
    ///<https://schema.org/cvdNumC19OverflowPats>
    #[serde(rename = "cvdNumC19OverflowPats")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub cvd_num_c19_overflow_pats: Vec<f32>,
    ///<https://schema.org/cvdNumVentUse>
    #[serde(rename = "cvdNumVentUse")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub cvd_num_vent_use: Vec<f32>,
    ///<https://schema.org/cvdNumBedsOcc>
    #[serde(rename = "cvdNumBedsOcc")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub cvd_num_beds_occ: Vec<f32>,
    ///<https://schema.org/cvdNumVent>
    #[serde(rename = "cvdNumVent")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub cvd_num_vent: Vec<f32>,
    ///<https://schema.org/cvdNumICUBedsOcc>
    #[serde(rename = "cvdNumICUBedsOcc")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub cvd_num_icu_beds_occ: Vec<f32>,
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
    pub additional_type: Vec<CDCPMDRecordAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(rename = "identifier")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub identifier: Vec<CDCPMDRecordIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(rename = "image")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub image: Vec<CDCPMDRecordImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(rename = "sameAs")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(rename = "description")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub description: Vec<CDCPMDRecordDescriptionFieldEnum>,
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
    pub subject_of: Vec<CDCPMDRecordSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(rename = "name")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(rename = "mainEntityOfPage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub main_entity_of_page: Vec<CDCPMDRecordMainEntityOfPageFieldEnum>,
}
