use std::str::FromStr;

use serde::de;
use serde::de::Error;
use serde_json::Value;

use crate::data_type::text::URL;

pub(crate) fn has_bio_chem_entity_properties(value: &Value) -> bool {
    let values = [
        "bioChemInteraction",
        "bioChemSimilarity",
        "biologicalRole",
        "associatedDisease",
        "",
        "taxonomicRange",
    ];
    values.iter().any(|v| value.get(v).is_some())
}

pub(crate) fn has_creative_work_properties(value: &Value) -> bool {
    let values = [
        "headline",
        "author",
        "datePublished",
        "encoding",
        "publisher",
        "license",
        "creator",
        "abstract",
        "text",
        "position",
    ];
    values.iter().any(|v| value.get(v).is_some())
}

pub(crate) fn has_datafeed_properties(value: &Value) -> bool {
    let values = ["dateCreated", "dateDeleted", "dateModified", "item"];
    values.iter().any(|v| value.get(v).is_some())
}

pub(crate) fn has_defined_term_set_properties(value: &Value) -> bool {
    value.get("hasDefinedTerm").is_some()
}

pub(crate) fn has_event_properties(value: &Value) -> bool {
    let values = [
        "about",
        "duration",
        "startDate",
        "endDate",
        "location",
        "offers",
        "performer",
        "subEvent",
        "superEvent",
    ];
    values.iter().any(|v| value.get(v).is_some())
}

pub(crate) fn has_geoshape_properties(value: &Value) -> bool {
    let values = ["box", "circle", "elevation", "polygon", "polyline"];
    values.iter().any(|v| value.get(v).is_some())
}

pub(crate) fn has_list_item_properties(value: &Value) -> bool {
    let values = ["position", "nextItem", "previousItem", "item"];
    values.iter().any(|v| value.get(v).is_some())
}

pub(crate) fn has_medical_condition_properties(value: &Value) -> bool {
    let values = [
        "differentialDiagnosis",
        "drug",
        "epidemiology",
        "expectedPrognosis",
        "pathophysiology",
        "possibleTreatment",
        "status",
        "signOrSymptom",
    ];
    values.iter().any(|v| value.get(v).is_some())
}

pub(crate) fn has_medical_entity_properties(value: &Value) -> bool {
    let values = [
        "code",
        "guideline",
        "medicineSystem",
        "study",
        "relevantSpecialty",
    ];
    values.iter().any(|v| value.get(v).is_some())
}

pub(crate) fn has_organization_properties(value: &Value) -> bool {
    let values = [
        "address",
        "brand",
        "contactPoint",
        "email",
        "faxNumber",
        "globalLocationNumber",
        "hasPOS",
        "logo",
        "makesOffer",
        "member",
        "parentOrganization",
        "slogan",
        "telephone",
        "url",
    ];
    values.iter().any(|v| value.get(v).is_some())
}

pub(crate) fn has_person_properties(value: &Value) -> bool {
    let values = [
        "birthDate",
        "deathDate",
        "gender",
        "homeLocation",
        "jobTitle",
        "nationality",
        "spouse",
    ];
    values.iter().any(|v| value.get(v).is_some())
}

pub(crate) fn has_place_properties(value: &Value) -> bool {
    let values = [
        "address",
        "amenityFeature",
        "branchCode",
        "latitude",
        "longitude",
        "geo",
        "geoShape",
    ];
    values.iter().any(|v| value.get(v).is_some())
}

pub(crate) fn has_property_properties(value: &Value) -> bool {
    value.get("domainIncludes").is_some() || value.get("rangeIncludes").is_some()
}

pub(crate) fn parse_url_variant<E, T>(v: &str, map_fn: impl FnOnce(URL) -> T) -> Result<T, E>
where
    E: Error,
{
    URL::from_str(v)
        .map(map_fn)
        .map_err(|_| Error::invalid_value(de::Unexpected::Str(v), &"a valid URL"))
}
