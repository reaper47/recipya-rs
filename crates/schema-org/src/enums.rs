use serde::{Deserialize, Serialize};

///<https://schema.org/MusicAlbumProductionType>
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum MusicAlbumProductionTypeEnum {
    ///<https://schema.org/DJMixAlbum>
    DJMixAlbum,
    ///<https://schema.org/DemoAlbum>
    DemoAlbum,
    ///<https://schema.org/StudioAlbum>
    #[default]
    StudioAlbum,
    ///<https://schema.org/SpokenWordAlbum>
    SpokenWordAlbum,
    ///<https://schema.org/LiveAlbum>
    LiveAlbum,
    ///<https://schema.org/RemixAlbum>
    RemixAlbum,
    ///<https://schema.org/MixtapeAlbum>
    MixtapeAlbum,
    ///<https://schema.org/SoundtrackAlbum>
    SoundtrackAlbum,
    ///<https://schema.org/CompilationAlbum>
    CompilationAlbum,
}
///<https://schema.org/MusicReleaseFormatType>
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum MusicReleaseFormatTypeEnum {
    ///<https://schema.org/DigitalFormat>
    #[default]
    DigitalFormat,
    ///<https://schema.org/VinylFormat>
    VinylFormat,
    ///<https://schema.org/LaserDiscFormat>
    LaserDiscFormat,
    ///<https://schema.org/CDFormat>
    CDFormat,
    ///<https://schema.org/CassetteFormat>
    CassetteFormat,
    ///<https://schema.org/DigitalAudioTapeFormat>
    DigitalAudioTapeFormat,
    ///<https://schema.org/DVDFormat>
    DVDFormat,
}
///<https://schema.org/ActionStatusType>
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum ActionStatusTypeEnum {
    ///<https://schema.org/ActiveActionStatus>
    #[default]
    ActiveActionStatus,
    ///<https://schema.org/PotentialActionStatus>
    PotentialActionStatus,
    ///<https://schema.org/CompletedActionStatus>
    CompletedActionStatus,
    ///<https://schema.org/FailedActionStatus>
    FailedActionStatus,
}
///<https://schema.org/OfferItemCondition>
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum OfferItemConditionEnum {
    ///<https://schema.org/NewCondition>
    NewCondition,
    ///<https://schema.org/UsedCondition>
    #[default]
    UsedCondition,
    ///<https://schema.org/RefurbishedCondition>
    RefurbishedCondition,
    ///<https://schema.org/DamagedCondition>
    DamagedCondition,
}
///<https://schema.org/RestrictedDiet>
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(try_from = "String")]
pub enum RestrictedDietEnum {
    ///<https://schema.org/HinduDiet>
    HinduDiet,
    ///<https://schema.org/KosherDiet>
    KosherDiet,
    ///<https://schema.org/GlutenFreeDiet>
    GlutenFreeDiet,
    ///<https://schema.org/LowCalorieDiet>
    LowCalorieDiet,
    ///<https://schema.org/HalalDiet>
    HalalDiet,
    ///<https://schema.org/DiabeticDiet>
    DiabeticDiet,
    ///<https://schema.org/VeganDiet>
    VeganDiet,
    ///<https://schema.org/LowSaltDiet>
    LowSaltDiet,
    ///<https://schema.org/LowFatDiet>
    LowFatDiet,
    ///<https://schema.org/VegetarianDiet>
    #[default]
    VegetarianDiet,
    ///<https://schema.org/LowLactoseDiet>
    LowLactoseDiet,
    UnspecifiedDiet,
}

impl From<String> for RestrictedDietEnum {
    fn from(value: String) -> Self {
        let value = value.to_lowercase();

        if value.contains("diabetic") {
            RestrictedDietEnum::DiabeticDiet
        } else if value.contains("gluten") {
            RestrictedDietEnum::GlutenFreeDiet
        } else if value.contains("halal") {
            RestrictedDietEnum::HalalDiet
        } else if value.contains("hindu") {
            RestrictedDietEnum::HinduDiet
        } else if value.contains("kosher") {
            RestrictedDietEnum::KosherDiet
        } else if value.contains("low") {
            if value.contains("calorie") {
                RestrictedDietEnum::LowCalorieDiet
            } else if value.contains("fat") {
                RestrictedDietEnum::LowFatDiet
            } else if value.contains("lactose") {
                RestrictedDietEnum::LowLactoseDiet
            } else if value.contains("salt") {
                RestrictedDietEnum::LowSaltDiet
            } else {
                RestrictedDietEnum::UnspecifiedDiet
            }
        } else if value.contains("vegan") {
            RestrictedDietEnum::VeganDiet
        } else if value.contains("vegetarian") {
            RestrictedDietEnum::VegetarianDiet
        } else {
            RestrictedDietEnum::UnspecifiedDiet
        }
    }
}

///<https://schema.org/AdultOrientedEnumeration>
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum AdultOrientedEnumerationEnum {
    ///<https://schema.org/ReducedRelevanceForChildrenConsideration>
    #[default]
    ReducedRelevanceForChildrenConsideration,
    ///<https://schema.org/ViolenceConsideration>
    ViolenceConsideration,
    ///<https://schema.org/WeaponConsideration>
    WeaponConsideration,
    ///<https://schema.org/NarcoticConsideration>
    NarcoticConsideration,
    ///<https://schema.org/SexualContentConsideration>
    SexualContentConsideration,
    ///<https://schema.org/HealthcareConsideration>
    HealthcareConsideration,
    ///<https://schema.org/AlcoholConsideration>
    AlcoholConsideration,
    ///<https://schema.org/DangerousGoodConsideration>
    DangerousGoodConsideration,
    ///<https://schema.org/TobaccoNicotineConsideration>
    TobaccoNicotineConsideration,
    ///<https://schema.org/UnclassifiedAdultConsideration>
    UnclassifiedAdultConsideration,
}
///<https://schema.org/ItemListOrderType>
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum ItemListOrderTypeEnum {
    ///<https://schema.org/ItemListOrderDescending>
    #[default]
    ItemListOrderDescending,
    ///<https://schema.org/ItemListOrderAscending>
    ItemListOrderAscending,
    ///<https://schema.org/ItemListUnordered>
    ItemListUnordered,
}
///<https://schema.org/MeasurementMethodEnum>
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum MeasurementMethodEnumEnum {
    ///<https://schema.org/ExampleMeasurementMethodEnum>
    #[default]
    ExampleMeasurementMethodEnum,
}
///<https://schema.org/EventStatusType>
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum EventStatusTypeEnum {
    ///<https://schema.org/EventPostponed>
    EventPostponed,
    ///<https://schema.org/EventScheduled>
    #[default]
    EventScheduled,
    ///<https://schema.org/EventRescheduled>
    EventRescheduled,
    ///<https://schema.org/EventMovedOnline>
    EventMovedOnline,
    ///<https://schema.org/EventCancelled>
    EventCancelled,
}
///<https://schema.org/EventAttendanceModeEnumeration>
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum EventAttendanceModeEnumerationEnum {
    ///<https://schema.org/MixedEventAttendanceMode>
    MixedEventAttendanceMode,
    ///<https://schema.org/OfflineEventAttendanceMode>
    #[default]
    OfflineEventAttendanceMode,
    ///<https://schema.org/OnlineEventAttendanceMode>
    OnlineEventAttendanceMode,
}
///<https://schema.org/GenderType>
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum GenderTypeEnum {
    ///<https://schema.org/Male>
    #[default]
    Male,
    ///<https://schema.org/Female>
    Female,
}
///<https://schema.org/MusicAlbumReleaseType>
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum MusicAlbumReleaseTypeEnum {
    ///<https://schema.org/EPRelease>
    EPRelease,
    ///<https://schema.org/AlbumRelease>
    #[default]
    AlbumRelease,
    ///<https://schema.org/SingleRelease>
    SingleRelease,
    ///<https://schema.org/BroadcastRelease>
    BroadcastRelease,
}
///<https://schema.org/ContactPointOption>
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum ContactPointOptionEnum {
    ///<https://schema.org/HearingImpairedSupported>
    HearingImpairedSupported,
    ///<https://schema.org/TollFree>
    #[default]
    TollFree,
}
