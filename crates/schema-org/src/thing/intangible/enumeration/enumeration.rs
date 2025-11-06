use schemars::JsonSchema;
use serde::Deserialize;

/// Enumerated options related to a ContactPoint.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
pub enum ContactPointOption {
    #[default]
    HearingImpairedSupported,
    TollFree,
}

/// The day of the week, e.g. used to specify to which day the opening hours of an
/// OpeningHoursSpecification refer.
///
/// Originally, URLs from GoodRelations were used (for Monday, Tuesday, Wednesday, Thursday, Friday,
/// Saturday, Sunday plus a special entry for PublicHolidays); these have now been integrated
/// directly into schema.org.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
pub enum DayOfWeek {
    #[default]
    Friday,
    Monday,
    PublicHolidays,
    Saturday,
    Sunday,
    Thursday,
    Tuesday,
    Wednesday,
}

/// Enumeration(s) for use with measurementMethod.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
pub enum MeasurementMethodEnum {
    #[default]
    ExampleMeasurementMethodEnum,
}

/// Enumeration of common measurement types (or dimensions), for example "chest" for a person,
/// "inseam" for pants, "gauge" for screws, or "wheel" for bicycles.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
pub enum MeasurementTypeEnumeration {
    #[default]
    BodyMeasurementTypeEnumeration,
    WearableMeasurementTypeEnumeration,
}

/// Any branch of a field in which people typically develop specific expertise, usually after
/// significant study, time, and effort.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
pub enum Specialty {
    #[default]
    MedicalSpecialty,
}

/// Categories of physical activity, organized by physiologic classification.
#[derive(Debug, Deserialize, PartialEq, JsonSchema)]
pub enum PhysicalActivityCategory {
    AerobicActivity,
    AnaerobicActivity,
    Balance,
    Flexibility,
    LeisureTimeActivity,
    OccupationalActivity,
    StrengthTraining,
}
