///<https://schema.org/MusicAlbumProductionType>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum MusicAlbumProductionTypeEnum {
    ///<https://schema.org/DJMixAlbum>
    DJMixAlbum,
    ///<https://schema.org/DemoAlbum>
    DemoAlbum,
    ///<https://schema.org/StudioAlbum>
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
///<https://schema.org/USNonprofitType>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum USNonprofitTypeEnum {
    ///<https://schema.org/Nonprofit501c27>
    Nonprofit501c27,
    ///<https://schema.org/Nonprofit501c7>
    Nonprofit501c7,
    ///<https://schema.org/Nonprofit501c9>
    Nonprofit501c9,
    ///<https://schema.org/Nonprofit501c4>
    Nonprofit501c4,
    ///<https://schema.org/Nonprofit501c22>
    Nonprofit501c22,
    ///<https://schema.org/Nonprofit501c28>
    Nonprofit501c28,
    ///<https://schema.org/Nonprofit501d>
    Nonprofit501d,
    ///<https://schema.org/Nonprofit501c19>
    Nonprofit501c19,
    ///<https://schema.org/Nonprofit527>
    Nonprofit527,
    ///<https://schema.org/Nonprofit501c20>
    Nonprofit501c20,
    ///<https://schema.org/Nonprofit501c25>
    Nonprofit501c25,
    ///<https://schema.org/Nonprofit501c24>
    Nonprofit501c24,
    ///<https://schema.org/Nonprofit501c15>
    Nonprofit501c15,
    ///<https://schema.org/Nonprofit501c6>
    Nonprofit501c6,
    ///<https://schema.org/Nonprofit501e>
    Nonprofit501e,
    ///<https://schema.org/Nonprofit501c12>
    Nonprofit501c12,
    ///<https://schema.org/Nonprofit501k>
    Nonprofit501k,
    ///<https://schema.org/Nonprofit501c11>
    Nonprofit501c11,
    ///<https://schema.org/Nonprofit501c14>
    Nonprofit501c14,
    ///<https://schema.org/Nonprofit501c5>
    Nonprofit501c5,
    ///<https://schema.org/Nonprofit501c3>
    Nonprofit501c3,
    ///<https://schema.org/Nonprofit501c10>
    Nonprofit501c10,
    ///<https://schema.org/Nonprofit501c26>
    Nonprofit501c26,
    ///<https://schema.org/Nonprofit501c16>
    Nonprofit501c16,
    ///<https://schema.org/Nonprofit501c13>
    Nonprofit501c13,
    ///<https://schema.org/Nonprofit501c17>
    Nonprofit501c17,
    ///<https://schema.org/Nonprofit501c1>
    Nonprofit501c1,
    ///<https://schema.org/Nonprofit501a>
    Nonprofit501a,
    ///<https://schema.org/Nonprofit501c23>
    Nonprofit501c23,
    ///<https://schema.org/Nonprofit501c18>
    Nonprofit501c18,
    ///<https://schema.org/Nonprofit501c2>
    Nonprofit501c2,
    ///<https://schema.org/Nonprofit501f>
    Nonprofit501f,
    ///<https://schema.org/Nonprofit501q>
    Nonprofit501q,
    ///<https://schema.org/Nonprofit501c8>
    Nonprofit501c8,
    ///<https://schema.org/Nonprofit501c21>
    Nonprofit501c21,
    ///<https://schema.org/Nonprofit501n>
    Nonprofit501n,
}
///<https://schema.org/UKNonprofitType>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum UKNonprofitTypeEnum {
    ///<https://schema.org/UKTrust>
    UKTrust,
    ///<https://schema.org/CharitableIncorporatedOrganization>
    CharitableIncorporatedOrganization,
    ///<https://schema.org/UnincorporatedAssociationCharity>
    UnincorporatedAssociationCharity,
    ///<https://schema.org/LimitedByGuaranteeCharity>
    LimitedByGuaranteeCharity,
}
///<https://schema.org/ReturnMethodEnumeration>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum ReturnMethodEnumerationEnum {
    ///<https://schema.org/ReturnAtKiosk>
    ReturnAtKiosk,
    ///<https://schema.org/ReturnByMail>
    ReturnByMail,
    ///<https://schema.org/ReturnInStore>
    ReturnInStore,
    ///<https://schema.org/KeepProduct>
    KeepProduct,
}
///<https://schema.org/PhysicalActivityCategory>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum PhysicalActivityCategoryEnum {
    ///<https://schema.org/AerobicActivity>
    AerobicActivity,
    ///<https://schema.org/AnaerobicActivity>
    AnaerobicActivity,
    ///<https://schema.org/LeisureTimeActivity>
    LeisureTimeActivity,
    ///<https://schema.org/StrengthTraining>
    StrengthTraining,
    ///<https://schema.org/Balance>
    Balance,
    ///<https://schema.org/Flexibility>
    Flexibility,
    ///<https://schema.org/OccupationalActivity>
    OccupationalActivity,
}
///<https://schema.org/WearableMeasurementTypeEnumeration>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum WearableMeasurementTypeEnumerationEnum {
    ///<https://schema.org/WearableMeasurementHeight>
    WearableMeasurementHeight,
    ///<https://schema.org/WearableMeasurementCollar>
    WearableMeasurementCollar,
    ///<https://schema.org/WearableMeasurementHips>
    WearableMeasurementHips,
    ///<https://schema.org/WearableMeasurementSleeve>
    WearableMeasurementSleeve,
    ///<https://schema.org/WearableMeasurementCup>
    WearableMeasurementCup,
    ///<https://schema.org/WearableMeasurementBack>
    WearableMeasurementBack,
    ///<https://schema.org/WearableMeasurementInseam>
    WearableMeasurementInseam,
    ///<https://schema.org/WearableMeasurementWaist>
    WearableMeasurementWaist,
    ///<https://schema.org/WearableMeasurementChestOrBust>
    WearableMeasurementChestOrBust,
    ///<https://schema.org/WearableMeasurementWidth>
    WearableMeasurementWidth,
    ///<https://schema.org/WearableMeasurementOutsideLeg>
    WearableMeasurementOutsideLeg,
    ///<https://schema.org/WearableMeasurementLength>
    WearableMeasurementLength,
}
///<https://schema.org/NLNonprofitType>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum NLNonprofitTypeEnum {
    ///<https://schema.org/NonprofitANBI>
    NonprofitANBI,
    ///<https://schema.org/NonprofitSBBI>
    NonprofitSBBI,
}
///<https://schema.org/MusicReleaseFormatType>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum MusicReleaseFormatTypeEnum {
    ///<https://schema.org/DigitalFormat>
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
///<https://schema.org/MerchantReturnEnumeration>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum MerchantReturnEnumerationEnum {
    ///<https://schema.org/MerchantReturnFiniteReturnWindow>
    MerchantReturnFiniteReturnWindow,
    ///<https://schema.org/MerchantReturnNotPermitted>
    MerchantReturnNotPermitted,
    ///<https://schema.org/MerchantReturnUnlimitedWindow>
    MerchantReturnUnlimitedWindow,
    ///<https://schema.org/MerchantReturnUnspecified>
    MerchantReturnUnspecified,
}
///<https://schema.org/ActionStatusType>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum ActionStatusTypeEnum {
    ///<https://schema.org/ActiveActionStatus>
    ActiveActionStatus,
    ///<https://schema.org/PotentialActionStatus>
    PotentialActionStatus,
    ///<https://schema.org/CompletedActionStatus>
    CompletedActionStatus,
    ///<https://schema.org/FailedActionStatus>
    FailedActionStatus,
}
///<https://schema.org/ReturnFeesEnumeration>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum ReturnFeesEnumerationEnum {
    ///<https://schema.org/FreeReturn>
    FreeReturn,
    ///<https://schema.org/RestockingFees>
    RestockingFees,
    ///<https://schema.org/ReturnShippingFees>
    ReturnShippingFees,
    ///<https://schema.org/ReturnFeesCustomerResponsibility>
    ReturnFeesCustomerResponsibility,
    ///<https://schema.org/OriginalShippingFees>
    OriginalShippingFees,
}
///<https://schema.org/PhysicalExam>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum PhysicalExamEnum {
    ///<https://schema.org/Neuro>
    Neuro,
    ///<https://schema.org/MusculoskeletalExam>
    MusculoskeletalExam,
    ///<https://schema.org/Genitourinary>
    Genitourinary,
    ///<https://schema.org/Ear>
    Ear,
    ///<https://schema.org/Abdomen>
    Abdomen,
    ///<https://schema.org/Appearance>
    Appearance,
    ///<https://schema.org/Nose>
    Nose,
    ///<https://schema.org/Head>
    Head,
    ///<https://schema.org/CardiovascularExam>
    CardiovascularExam,
    ///<https://schema.org/Lung>
    Lung,
    ///<https://schema.org/Throat>
    Throat,
    ///<https://schema.org/Skin>
    Skin,
    ///<https://schema.org/Neck>
    Neck,
    ///<https://schema.org/Eye>
    Eye,
}
///<https://schema.org/GovernmentBenefitsType>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum GovernmentBenefitsTypeEnum {
    ///<https://schema.org/UnemploymentSupport>
    UnemploymentSupport,
    ///<https://schema.org/HealthCare>
    HealthCare,
    ///<https://schema.org/ParentalSupport>
    ParentalSupport,
    ///<https://schema.org/BusinessSupport>
    BusinessSupport,
    ///<https://schema.org/BasicIncome>
    BasicIncome,
    ///<https://schema.org/DisabilitySupport>
    DisabilitySupport,
    ///<https://schema.org/PaidLeave>
    PaidLeave,
    ///<https://schema.org/OneTimePayments>
    OneTimePayments,
}
///<https://schema.org/DriveWheelConfigurationValue>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum DriveWheelConfigurationValueEnum {
    ///<https://schema.org/RearWheelDriveConfiguration>
    RearWheelDriveConfiguration,
    ///<https://schema.org/FrontWheelDriveConfiguration>
    FrontWheelDriveConfiguration,
    ///<https://schema.org/FourWheelDriveConfiguration>
    FourWheelDriveConfiguration,
    ///<https://schema.org/AllWheelDriveConfiguration>
    AllWheelDriveConfiguration,
}
///<https://schema.org/EUEnergyEfficiencyEnumeration>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum EUEnergyEfficiencyEnumerationEnum {
    ///<https://schema.org/EUEnergyEfficiencyCategoryE>
    EUEnergyEfficiencyCategoryE,
    ///<https://schema.org/EUEnergyEfficiencyCategoryA1Plus>
    EUEnergyEfficiencyCategoryA1Plus,
    ///<https://schema.org/EUEnergyEfficiencyCategoryA3Plus>
    EUEnergyEfficiencyCategoryA3Plus,
    ///<https://schema.org/EUEnergyEfficiencyCategoryF>
    EUEnergyEfficiencyCategoryF,
    ///<https://schema.org/EUEnergyEfficiencyCategoryB>
    EUEnergyEfficiencyCategoryB,
    ///<https://schema.org/EUEnergyEfficiencyCategoryA2Plus>
    EUEnergyEfficiencyCategoryA2Plus,
    ///<https://schema.org/EUEnergyEfficiencyCategoryA>
    EUEnergyEfficiencyCategoryA,
    ///<https://schema.org/EUEnergyEfficiencyCategoryD>
    EUEnergyEfficiencyCategoryD,
    ///<https://schema.org/EUEnergyEfficiencyCategoryG>
    EUEnergyEfficiencyCategoryG,
    ///<https://schema.org/EUEnergyEfficiencyCategoryC>
    EUEnergyEfficiencyCategoryC,
}
///<https://schema.org/IncentiveStatus>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum IncentiveStatusEnum {
    ///<https://schema.org/IncentiveStatusRetired>
    IncentiveStatusRetired,
    ///<https://schema.org/IncentiveStatusActive>
    IncentiveStatusActive,
    ///<https://schema.org/IncentiveStatusOnHold>
    IncentiveStatusOnHold,
    ///<https://schema.org/IncentiveStatusInDevelopment>
    IncentiveStatusInDevelopment,
}
///<https://schema.org/MedicineSystem>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum MedicineSystemEnum {
    ///<https://schema.org/Homeopathic>
    Homeopathic,
    ///<https://schema.org/Chiropractic>
    Chiropractic,
    ///<https://schema.org/Ayurvedic>
    Ayurvedic,
    ///<https://schema.org/Osteopathic>
    Osteopathic,
    ///<https://schema.org/WesternConventional>
    WesternConventional,
    ///<https://schema.org/TraditionalChinese>
    TraditionalChinese,
}
///<https://schema.org/HealthAspectEnumeration>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum HealthAspectEnumerationEnum {
    ///<https://schema.org/GettingAccessHealthAspect>
    GettingAccessHealthAspect,
    ///<https://schema.org/UsageOrScheduleHealthAspect>
    UsageOrScheduleHealthAspect,
    ///<https://schema.org/CausesHealthAspect>
    CausesHealthAspect,
    ///<https://schema.org/ContagiousnessHealthAspect>
    ContagiousnessHealthAspect,
    ///<https://schema.org/IngredientsHealthAspect>
    IngredientsHealthAspect,
    ///<https://schema.org/PreventionHealthAspect>
    PreventionHealthAspect,
    ///<https://schema.org/SymptomsHealthAspect>
    SymptomsHealthAspect,
    ///<https://schema.org/BenefitsHealthAspect>
    BenefitsHealthAspect,
    ///<https://schema.org/RisksOrComplicationsHealthAspect>
    RisksOrComplicationsHealthAspect,
    ///<https://schema.org/RelatedTopicsHealthAspect>
    RelatedTopicsHealthAspect,
    ///<https://schema.org/ScreeningHealthAspect>
    ScreeningHealthAspect,
    ///<https://schema.org/SideEffectsHealthAspect>
    SideEffectsHealthAspect,
    ///<https://schema.org/StagesHealthAspect>
    StagesHealthAspect,
    ///<https://schema.org/MisconceptionsHealthAspect>
    MisconceptionsHealthAspect,
    ///<https://schema.org/EffectivenessHealthAspect>
    EffectivenessHealthAspect,
    ///<https://schema.org/SeeDoctorHealthAspect>
    SeeDoctorHealthAspect,
    ///<https://schema.org/PrognosisHealthAspect>
    PrognosisHealthAspect,
    ///<https://schema.org/TreatmentsHealthAspect>
    TreatmentsHealthAspect,
    ///<https://schema.org/SelfCareHealthAspect>
    SelfCareHealthAspect,
    ///<https://schema.org/SafetyHealthAspect>
    SafetyHealthAspect,
    ///<https://schema.org/AllergiesHealthAspect>
    AllergiesHealthAspect,
    ///<https://schema.org/PregnancyHealthAspect>
    PregnancyHealthAspect,
    ///<https://schema.org/OverviewHealthAspect>
    OverviewHealthAspect,
    ///<https://schema.org/MayTreatHealthAspect>
    MayTreatHealthAspect,
    ///<https://schema.org/HowItWorksHealthAspect>
    HowItWorksHealthAspect,
    ///<https://schema.org/LivingWithHealthAspect>
    LivingWithHealthAspect,
    ///<https://schema.org/HowOrWhereHealthAspect>
    HowOrWhereHealthAspect,
    ///<https://schema.org/TypesHealthAspect>
    TypesHealthAspect,
    ///<https://schema.org/PatientExperienceHealthAspect>
    PatientExperienceHealthAspect,
}
///<https://schema.org/WearableSizeGroupEnumeration>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum WearableSizeGroupEnumerationEnum {
    ///<https://schema.org/WearableSizeGroupExtraTall>
    WearableSizeGroupExtraTall,
    ///<https://schema.org/WearableSizeGroupPetite>
    WearableSizeGroupPetite,
    ///<https://schema.org/WearableSizeGroupMaternity>
    WearableSizeGroupMaternity,
    ///<https://schema.org/WearableSizeGroupInfants>
    WearableSizeGroupInfants,
    ///<https://schema.org/WearableSizeGroupBoys>
    WearableSizeGroupBoys,
    ///<https://schema.org/WearableSizeGroupMens>
    WearableSizeGroupMens,
    ///<https://schema.org/WearableSizeGroupRegular>
    WearableSizeGroupRegular,
    ///<https://schema.org/WearableSizeGroupGirls>
    WearableSizeGroupGirls,
    ///<https://schema.org/WearableSizeGroupTall>
    WearableSizeGroupTall,
    ///<https://schema.org/WearableSizeGroupMisses>
    WearableSizeGroupMisses,
    ///<https://schema.org/WearableSizeGroupExtraShort>
    WearableSizeGroupExtraShort,
    ///<https://schema.org/WearableSizeGroupShort>
    WearableSizeGroupShort,
    ///<https://schema.org/WearableSizeGroupWomens>
    WearableSizeGroupWomens,
    ///<https://schema.org/WearableSizeGroupHusky>
    WearableSizeGroupHusky,
    ///<https://schema.org/WearableSizeGroupPlus>
    WearableSizeGroupPlus,
    ///<https://schema.org/WearableSizeGroupJuniors>
    WearableSizeGroupJuniors,
    ///<https://schema.org/WearableSizeGroupBig>
    WearableSizeGroupBig,
}
///<https://schema.org/MedicalObservationalStudyDesign>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum MedicalObservationalStudyDesignEnum {
    ///<https://schema.org/CrossSectional>
    CrossSectional,
    ///<https://schema.org/Registry>
    Registry,
    ///<https://schema.org/CaseSeries>
    CaseSeries,
    ///<https://schema.org/Longitudinal>
    Longitudinal,
    ///<https://schema.org/CohortStudy>
    CohortStudy,
    ///<https://schema.org/Observational>
    Observational,
}
///<https://schema.org/OfferItemCondition>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum OfferItemConditionEnum {
    ///<https://schema.org/NewCondition>
    NewCondition,
    ///<https://schema.org/UsedCondition>
    UsedCondition,
    ///<https://schema.org/RefurbishedCondition>
    RefurbishedCondition,
    ///<https://schema.org/DamagedCondition>
    DamagedCondition,
}
///<https://schema.org/WearableSizeSystemEnumeration>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum WearableSizeSystemEnumerationEnum {
    ///<https://schema.org/WearableSizeSystemJP>
    WearableSizeSystemJP,
    ///<https://schema.org/WearableSizeSystemEN13402>
    WearableSizeSystemEN13402,
    ///<https://schema.org/WearableSizeSystemMX>
    WearableSizeSystemMX,
    ///<https://schema.org/WearableSizeSystemFR>
    WearableSizeSystemFR,
    ///<https://schema.org/WearableSizeSystemCN>
    WearableSizeSystemCN,
    ///<https://schema.org/WearableSizeSystemContinental>
    WearableSizeSystemContinental,
    ///<https://schema.org/WearableSizeSystemEurope>
    WearableSizeSystemEurope,
    ///<https://schema.org/WearableSizeSystemUK>
    WearableSizeSystemUK,
    ///<https://schema.org/WearableSizeSystemIT>
    WearableSizeSystemIT,
    ///<https://schema.org/WearableSizeSystemAU>
    WearableSizeSystemAU,
    ///<https://schema.org/WearableSizeSystemGS1>
    WearableSizeSystemGS1,
    ///<https://schema.org/WearableSizeSystemUS>
    WearableSizeSystemUS,
    ///<https://schema.org/WearableSizeSystemDE>
    WearableSizeSystemDE,
    ///<https://schema.org/WearableSizeSystemBR>
    WearableSizeSystemBR,
}
///<https://schema.org/MedicalStudyStatus>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum MedicalStudyStatusEnum {
    ///<https://schema.org/ActiveNotRecruiting>
    ActiveNotRecruiting,
    ///<https://schema.org/Withdrawn>
    Withdrawn,
    ///<https://schema.org/Recruiting>
    Recruiting,
    ///<https://schema.org/Terminated>
    Terminated,
    ///<https://schema.org/Suspended>
    Suspended,
    ///<https://schema.org/EnrollingByInvitation>
    EnrollingByInvitation,
    ///<https://schema.org/ResultsAvailable>
    ResultsAvailable,
    ///<https://schema.org/ResultsNotAvailable>
    ResultsNotAvailable,
    ///<https://schema.org/NotYetRecruiting>
    NotYetRecruiting,
    ///<https://schema.org/Completed>
    Completed,
}
///<https://schema.org/RestrictedDiet>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
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
    VegetarianDiet,
    ///<https://schema.org/LowLactoseDiet>
    LowLactoseDiet,
}
///<https://schema.org/MedicalSpecialty>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum MedicalSpecialtyEnum {
    ///<https://schema.org/Musculoskeletal>
    Musculoskeletal,
    ///<https://schema.org/Dermatology>
    Dermatology,
    ///<https://schema.org/DietNutrition>
    DietNutrition,
    ///<https://schema.org/Hematologic>
    Hematologic,
    ///<https://schema.org/Neurologic>
    Neurologic,
    ///<https://schema.org/Genetic>
    Genetic,
    ///<https://schema.org/Nursing>
    Nursing,
    ///<https://schema.org/PublicHealth>
    PublicHealth,
    ///<https://schema.org/Physiotherapy>
    Physiotherapy,
    ///<https://schema.org/Urologic>
    Urologic,
    ///<https://schema.org/Surgical>
    Surgical,
    ///<https://schema.org/CommunityHealth>
    CommunityHealth,
    ///<https://schema.org/Dentistry>
    Dentistry,
    ///<https://schema.org/Psychiatric>
    Psychiatric,
    ///<https://schema.org/RespiratoryTherapy>
    RespiratoryTherapy,
    ///<https://schema.org/SpeechPathology>
    SpeechPathology,
    ///<https://schema.org/Renal>
    Renal,
    ///<https://schema.org/PlasticSurgery>
    PlasticSurgery,
    ///<https://schema.org/Pediatric>
    Pediatric,
    ///<https://schema.org/Optometric>
    Optometric,
    ///<https://schema.org/Oncologic>
    Oncologic,
    ///<https://schema.org/Infectious>
    Infectious,
    ///<https://schema.org/Pathology>
    Pathology,
    ///<https://schema.org/LaboratoryScience>
    LaboratoryScience,
    ///<https://schema.org/Pulmonary>
    Pulmonary,
    ///<https://schema.org/Otolaryngologic>
    Otolaryngologic,
    ///<https://schema.org/PrimaryCare>
    PrimaryCare,
    ///<https://schema.org/Cardiovascular>
    Cardiovascular,
    ///<https://schema.org/Rheumatologic>
    Rheumatologic,
    ///<https://schema.org/Toxicologic>
    Toxicologic,
    ///<https://schema.org/Midwifery>
    Midwifery,
    ///<https://schema.org/Obstetric>
    Obstetric,
    ///<https://schema.org/PharmacySpecialty>
    PharmacySpecialty,
    ///<https://schema.org/Emergency>
    Emergency,
    ///<https://schema.org/Radiography>
    Radiography,
    ///<https://schema.org/Geriatric>
    Geriatric,
    ///<https://schema.org/Anesthesia>
    Anesthesia,
    ///<https://schema.org/Podiatric>
    Podiatric,
    ///<https://schema.org/Gynecologic>
    Gynecologic,
    ///<https://schema.org/Gastroenterologic>
    Gastroenterologic,
    ///<https://schema.org/Endocrine>
    Endocrine,
    ///<https://schema.org/Dermatologic>
    Dermatologic,
}
///<https://schema.org/LegalValueLevel>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum LegalValueLevelEnum {
    ///<https://schema.org/UnofficialLegalValue>
    UnofficialLegalValue,
    ///<https://schema.org/DefinitiveLegalValue>
    DefinitiveLegalValue,
    ///<https://schema.org/AuthoritativeLegalValue>
    AuthoritativeLegalValue,
    ///<https://schema.org/OfficialLegalValue>
    OfficialLegalValue,
}
///<https://schema.org/DrugPregnancyCategory>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum DrugPregnancyCategoryEnum {
    ///<https://schema.org/FDAcategoryC>
    FDAcategoryC,
    ///<https://schema.org/FDAcategoryX>
    FDAcategoryX,
    ///<https://schema.org/FDAcategoryD>
    FDAcategoryD,
    ///<https://schema.org/FDAcategoryB>
    FDAcategoryB,
    ///<https://schema.org/FDAnotEvaluated>
    FDAnotEvaluated,
    ///<https://schema.org/FDAcategoryA>
    FDAcategoryA,
}
///<https://schema.org/AdultOrientedEnumeration>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum AdultOrientedEnumerationEnum {
    ///<https://schema.org/ReducedRelevanceForChildrenConsideration>
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
///<https://schema.org/ReservationStatusType>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum ReservationStatusTypeEnum {
    ///<https://schema.org/ReservationPending>
    ReservationPending,
    ///<https://schema.org/ReservationConfirmed>
    ReservationConfirmed,
    ///<https://schema.org/ReservationCancelled>
    ReservationCancelled,
    ///<https://schema.org/ReservationHold>
    ReservationHold,
}
///<https://schema.org/ItemAvailability>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum ItemAvailabilityEnum {
    ///<https://schema.org/Reserved>
    Reserved,
    ///<https://schema.org/PreSale>
    PreSale,
    ///<https://schema.org/Discontinued>
    Discontinued,
    ///<https://schema.org/PreOrder>
    PreOrder,
    ///<https://schema.org/BackOrder>
    BackOrder,
    ///<https://schema.org/MadeToOrder>
    MadeToOrder,
    ///<https://schema.org/InStoreOnly>
    InStoreOnly,
    ///<https://schema.org/OnlineOnly>
    OnlineOnly,
    ///<https://schema.org/OutOfStock>
    OutOfStock,
    ///<https://schema.org/InStock>
    InStock,
    ///<https://schema.org/SoldOut>
    SoldOut,
    ///<https://schema.org/LimitedAvailability>
    LimitedAvailability,
}
///<https://schema.org/ItemListOrderType>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum ItemListOrderTypeEnum {
    ///<https://schema.org/ItemListOrderDescending>
    ItemListOrderDescending,
    ///<https://schema.org/ItemListOrderAscending>
    ItemListOrderAscending,
    ///<https://schema.org/ItemListUnordered>
    ItemListUnordered,
}
///<https://schema.org/MedicalTrialDesign>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum MedicalTrialDesignEnum {
    ///<https://schema.org/RandomizedTrial>
    RandomizedTrial,
    ///<https://schema.org/OpenTrial>
    OpenTrial,
    ///<https://schema.org/TripleBlindedTrial>
    TripleBlindedTrial,
    ///<https://schema.org/DoubleBlindedTrial>
    DoubleBlindedTrial,
    ///<https://schema.org/SingleCenterTrial>
    SingleCenterTrial,
    ///<https://schema.org/InternationalTrial>
    InternationalTrial,
    ///<https://schema.org/PlaceboControlledTrial>
    PlaceboControlledTrial,
    ///<https://schema.org/SingleBlindedTrial>
    SingleBlindedTrial,
    ///<https://schema.org/MultiCenterTrial>
    MultiCenterTrial,
}
///<https://schema.org/MedicalEvidenceLevel>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum MedicalEvidenceLevelEnum {
    ///<https://schema.org/EvidenceLevelB>
    EvidenceLevelB,
    ///<https://schema.org/EvidenceLevelC>
    EvidenceLevelC,
    ///<https://schema.org/EvidenceLevelA>
    EvidenceLevelA,
}
///<https://schema.org/IPTCDigitalSourceEnumeration>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum IPTCDigitalSourceEnumerationEnum {
    ///<https://schema.org/DigitalCaptureDigitalSource>
    DigitalCaptureDigitalSource,
    ///<https://schema.org/MinorHumanEditsDigitalSource>
    MinorHumanEditsDigitalSource,
    ///<https://schema.org/NegativeFilmDigitalSource>
    NegativeFilmDigitalSource,
    ///<https://schema.org/CompositeDigitalSource>
    CompositeDigitalSource,
    ///<https://schema.org/ScreenCaptureDigitalSource>
    ScreenCaptureDigitalSource,
    ///<https://schema.org/PrintDigitalSource>
    PrintDigitalSource,
    ///<https://schema.org/CompositeCaptureDigitalSource>
    CompositeCaptureDigitalSource,
    ///<https://schema.org/DigitalArtDigitalSource>
    DigitalArtDigitalSource,
    ///<https://schema.org/VirtualRecordingDigitalSource>
    VirtualRecordingDigitalSource,
    ///<https://schema.org/MultiFrameComputationalCaptureDigitalSource>
    MultiFrameComputationalCaptureDigitalSource,
    ///<https://schema.org/CompositeWithTrainedAlgorithmicMediaDigitalSource>
    CompositeWithTrainedAlgorithmicMediaDigitalSource,
    ///<https://schema.org/TrainedAlgorithmicMediaDigitalSource>
    TrainedAlgorithmicMediaDigitalSource,
    ///<https://schema.org/CompositeSyntheticDigitalSource>
    CompositeSyntheticDigitalSource,
    ///<https://schema.org/AlgorithmicallyEnhancedDigitalSource>
    AlgorithmicallyEnhancedDigitalSource,
    ///<https://schema.org/DataDrivenMediaDigitalSource>
    DataDrivenMediaDigitalSource,
    ///<https://schema.org/AlgorithmicMediaDigitalSource>
    AlgorithmicMediaDigitalSource,
    ///<https://schema.org/PositiveFilmDigitalSource>
    PositiveFilmDigitalSource,
}
///<https://schema.org/MeasurementMethodEnum>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum MeasurementMethodEnumEnum {
    ///<https://schema.org/ExampleMeasurementMethodEnum>
    ExampleMeasurementMethodEnum,
}
///<https://schema.org/DeliveryMethod>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum DeliveryMethodEnum {
    ///<https://schema.org/OnSitePickup>
    OnSitePickup,
    ///<https://schema.org/LockerDelivery>
    LockerDelivery,
    ///<https://schema.org/ParcelService>
    ParcelService,
}
///<https://schema.org/BoardingPolicyType>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum BoardingPolicyTypeEnum {
    ///<https://schema.org/ZoneBoardingPolicy>
    ZoneBoardingPolicy,
    ///<https://schema.org/GroupBoardingPolicy>
    GroupBoardingPolicy,
}
///<https://schema.org/PaymentStatusType>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum PaymentStatusTypeEnum {
    ///<https://schema.org/PaymentDeclined>
    PaymentDeclined,
    ///<https://schema.org/PaymentDue>
    PaymentDue,
    ///<https://schema.org/PaymentPastDue>
    PaymentPastDue,
    ///<https://schema.org/PaymentAutomaticallyApplied>
    PaymentAutomaticallyApplied,
    ///<https://schema.org/PaymentComplete>
    PaymentComplete,
}
///<https://schema.org/ReturnLabelSourceEnumeration>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum ReturnLabelSourceEnumerationEnum {
    ///<https://schema.org/ReturnLabelInBox>
    ReturnLabelInBox,
    ///<https://schema.org/ReturnLabelCustomerResponsibility>
    ReturnLabelCustomerResponsibility,
    ///<https://schema.org/ReturnLabelDownloadAndPrint>
    ReturnLabelDownloadAndPrint,
}
///<https://schema.org/EventStatusType>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum EventStatusTypeEnum {
    ///<https://schema.org/EventPostponed>
    EventPostponed,
    ///<https://schema.org/EventScheduled>
    EventScheduled,
    ///<https://schema.org/EventRescheduled>
    EventRescheduled,
    ///<https://schema.org/EventMovedOnline>
    EventMovedOnline,
    ///<https://schema.org/EventCancelled>
    EventCancelled,
}
///<https://schema.org/SizeSystemEnumeration>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum SizeSystemEnumerationEnum {
    ///<https://schema.org/SizeSystemImperial>
    SizeSystemImperial,
    ///<https://schema.org/SizeSystemMetric>
    SizeSystemMetric,
}
///<https://schema.org/BookFormatType>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum BookFormatTypeEnum {
    ///<https://schema.org/Hardcover>
    Hardcover,
    ///<https://schema.org/GraphicNovel>
    GraphicNovel,
    ///<https://schema.org/AudiobookFormat>
    AudiobookFormat,
    ///<https://schema.org/EBook>
    EBook,
    ///<https://schema.org/Paperback>
    Paperback,
}
///<https://schema.org/OrderStatus>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum OrderStatusEnum {
    ///<https://schema.org/OrderInTransit>
    OrderInTransit,
    ///<https://schema.org/OrderPickupAvailable>
    OrderPickupAvailable,
    ///<https://schema.org/OrderDelivered>
    OrderDelivered,
    ///<https://schema.org/OrderProblem>
    OrderProblem,
    ///<https://schema.org/OrderProcessing>
    OrderProcessing,
    ///<https://schema.org/OrderReturned>
    OrderReturned,
    ///<https://schema.org/OrderPaymentDue>
    OrderPaymentDue,
    ///<https://schema.org/OrderCancelled>
    OrderCancelled,
}
///<https://schema.org/BodyMeasurementTypeEnumeration>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum BodyMeasurementTypeEnumerationEnum {
    ///<https://schema.org/BodyMeasurementInsideLeg>
    BodyMeasurementInsideLeg,
    ///<https://schema.org/BodyMeasurementUnderbust>
    BodyMeasurementUnderbust,
    ///<https://schema.org/BodyMeasurementBust>
    BodyMeasurementBust,
    ///<https://schema.org/BodyMeasurementHand>
    BodyMeasurementHand,
    ///<https://schema.org/BodyMeasurementHeight>
    BodyMeasurementHeight,
    ///<https://schema.org/BodyMeasurementHead>
    BodyMeasurementHead,
    ///<https://schema.org/BodyMeasurementWeight>
    BodyMeasurementWeight,
    ///<https://schema.org/BodyMeasurementWaist>
    BodyMeasurementWaist,
    ///<https://schema.org/BodyMeasurementHips>
    BodyMeasurementHips,
    ///<https://schema.org/BodyMeasurementChest>
    BodyMeasurementChest,
    ///<https://schema.org/BodyMeasurementFoot>
    BodyMeasurementFoot,
    ///<https://schema.org/BodyMeasurementArm>
    BodyMeasurementArm,
    ///<https://schema.org/BodyMeasurementNeck>
    BodyMeasurementNeck,
}
///<https://schema.org/InfectiousAgentClass>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum InfectiousAgentClassEnum {
    ///<https://schema.org/Fungus>
    Fungus,
    ///<https://schema.org/Virus>
    Virus,
    ///<https://schema.org/MulticellularParasite>
    MulticellularParasite,
    ///<https://schema.org/Bacteria>
    Bacteria,
    ///<https://schema.org/Protozoa>
    Protozoa,
    ///<https://schema.org/Prion>
    Prion,
}
///<https://schema.org/MedicalAudienceType>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum MedicalAudienceTypeEnum {
    ///<https://schema.org/Clinician>
    Clinician,
    ///<https://schema.org/MedicalResearcher>
    MedicalResearcher,
}
///<https://schema.org/PriceTypeEnumeration>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum PriceTypeEnumerationEnum {
    ///<https://schema.org/SalePrice>
    SalePrice,
    ///<https://schema.org/MinimumAdvertisedPrice>
    MinimumAdvertisedPrice,
    ///<https://schema.org/InvoicePrice>
    InvoicePrice,
    ///<https://schema.org/RegularPrice>
    RegularPrice,
    ///<https://schema.org/MSRP>
    MSRP,
    ///<https://schema.org/SRP>
    SRP,
    ///<https://schema.org/StrikethroughPrice>
    StrikethroughPrice,
    ///<https://schema.org/ListPrice>
    ListPrice,
}
///<https://schema.org/EnergyStarEnergyEfficiencyEnumeration>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum EnergyStarEnergyEfficiencyEnumerationEnum {
    ///<https://schema.org/EnergyStarCertified>
    EnergyStarCertified,
}
///<https://schema.org/TierBenefitEnumeration>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum TierBenefitEnumerationEnum {
    ///<https://schema.org/TierBenefitLoyaltyReturns>
    TierBenefitLoyaltyReturns,
    ///<https://schema.org/TierBenefitLoyaltyShipping>
    TierBenefitLoyaltyShipping,
    ///<https://schema.org/TierBenefitLoyaltyPoints>
    TierBenefitLoyaltyPoints,
    ///<https://schema.org/TierBenefitLoyaltyPrice>
    TierBenefitLoyaltyPrice,
}
///<https://schema.org/CarUsageType>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum CarUsageTypeEnum {
    ///<https://schema.org/RentalVehicleUsage>
    RentalVehicleUsage,
    ///<https://schema.org/DrivingSchoolVehicleUsage>
    DrivingSchoolVehicleUsage,
    ///<https://schema.org/TaxiVehicleUsage>
    TaxiVehicleUsage,
}
///<https://schema.org/FulfillmentTypeEnumeration>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum FulfillmentTypeEnumerationEnum {
    ///<https://schema.org/FulfillmentTypePickupInStore>
    FulfillmentTypePickupInStore,
    ///<https://schema.org/FulfillmentTypeScheduledDelivery>
    FulfillmentTypeScheduledDelivery,
    ///<https://schema.org/FulfillmentTypeDelivery>
    FulfillmentTypeDelivery,
    ///<https://schema.org/FulfillmentTypeCollectionPoint>
    FulfillmentTypeCollectionPoint,
    ///<https://schema.org/FulfillmentTypePickupDropoff>
    FulfillmentTypePickupDropoff,
}
///<https://schema.org/EventAttendanceModeEnumeration>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum EventAttendanceModeEnumerationEnum {
    ///<https://schema.org/MixedEventAttendanceMode>
    MixedEventAttendanceMode,
    ///<https://schema.org/OfflineEventAttendanceMode>
    OfflineEventAttendanceMode,
    ///<https://schema.org/OnlineEventAttendanceMode>
    OnlineEventAttendanceMode,
}
///<https://schema.org/GameServerStatus>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum GameServerStatusEnum {
    ///<https://schema.org/Online>
    Online,
    ///<https://schema.org/OfflineTemporarily>
    OfflineTemporarily,
    ///<https://schema.org/OfflinePermanently>
    OfflinePermanently,
    ///<https://schema.org/OnlineFull>
    OnlineFull,
}
///<https://schema.org/PaymentMethodType>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum PaymentMethodTypeEnum {
    ///<https://schema.org/PhoneCarrierPayment>
    PhoneCarrierPayment,
    ///<https://schema.org/CheckInAdvance>
    CheckInAdvance,
    ///<https://schema.org/InStorePrepay>
    InStorePrepay,
    ///<https://schema.org/DirectDebit>
    DirectDebit,
    ///<https://schema.org/COD>
    COD,
    ///<https://schema.org/ByInvoice>
    ByInvoice,
    ///<https://schema.org/Cash>
    Cash,
    ///<https://schema.org/ByBankTransferInAdvance>
    ByBankTransferInAdvance,
}
///<https://schema.org/PurchaseType>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum PurchaseTypeEnum {
    ///<https://schema.org/PurchaseTypeUsedPurchase>
    PurchaseTypeUsedPurchase,
    ///<https://schema.org/PurchaseTypeNewPurchase>
    PurchaseTypeNewPurchase,
    ///<https://schema.org/PurchaseTypeLease>
    PurchaseTypeLease,
    ///<https://schema.org/PurchaseTypeTradeIn>
    PurchaseTypeTradeIn,
}
///<https://schema.org/CertificationStatusEnumeration>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum CertificationStatusEnumerationEnum {
    ///<https://schema.org/CertificationActive>
    CertificationActive,
    ///<https://schema.org/CertificationInactive>
    CertificationInactive,
}
///<https://schema.org/DrugPrescriptionStatus>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum DrugPrescriptionStatusEnum {
    ///<https://schema.org/PrescriptionOnly>
    PrescriptionOnly,
    ///<https://schema.org/OTC>
    OTC,
}
///<https://schema.org/RefundTypeEnumeration>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum RefundTypeEnumerationEnum {
    ///<https://schema.org/ExchangeRefund>
    ExchangeRefund,
    ///<https://schema.org/FullRefund>
    FullRefund,
    ///<https://schema.org/StoreCreditRefund>
    StoreCreditRefund,
}
///<https://schema.org/GamePlayMode>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum GamePlayModeEnum {
    ///<https://schema.org/MultiPlayer>
    MultiPlayer,
    ///<https://schema.org/SinglePlayer>
    SinglePlayer,
    ///<https://schema.org/CoOp>
    CoOp,
}
///<https://schema.org/DayOfWeek>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum DayOfWeekEnum {
    ///<https://schema.org/Thursday>
    Thursday,
    ///<https://schema.org/Friday>
    Friday,
    ///<https://schema.org/Saturday>
    Saturday,
    ///<https://schema.org/Tuesday>
    Tuesday,
    ///<https://schema.org/PublicHolidays>
    PublicHolidays,
    ///<https://schema.org/Wednesday>
    Wednesday,
    ///<https://schema.org/Sunday>
    Sunday,
    ///<https://schema.org/Monday>
    Monday,
}
///<https://schema.org/GenderType>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum GenderTypeEnum {
    ///<https://schema.org/Male>
    Male,
    ///<https://schema.org/Female>
    Female,
}
///<https://schema.org/DigitalPlatformEnumeration>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum DigitalPlatformEnumerationEnum {
    ///<https://schema.org/IOSPlatform>
    IOSPlatform,
    ///<https://schema.org/DesktopWebPlatform>
    DesktopWebPlatform,
    ///<https://schema.org/AndroidPlatform>
    AndroidPlatform,
    ///<https://schema.org/MobileWebPlatform>
    MobileWebPlatform,
    ///<https://schema.org/GenericWebPlatform>
    GenericWebPlatform,
}
///<https://schema.org/MediaManipulationRatingEnumeration>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum MediaManipulationRatingEnumerationEnum {
    ///<https://schema.org/TransformedContent>
    TransformedContent,
    ///<https://schema.org/SatireOrParodyContent>
    SatireOrParodyContent,
    ///<https://schema.org/OriginalMediaContent>
    OriginalMediaContent,
    ///<https://schema.org/DecontextualizedContent>
    DecontextualizedContent,
    ///<https://schema.org/EditedOrCroppedContent>
    EditedOrCroppedContent,
    ///<https://schema.org/StagedContent>
    StagedContent,
}
///<https://schema.org/PriceComponentTypeEnumeration>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum PriceComponentTypeEnumerationEnum {
    ///<https://schema.org/CleaningFee>
    CleaningFee,
    ///<https://schema.org/DistanceFee>
    DistanceFee,
    ///<https://schema.org/Downpayment>
    Downpayment,
    ///<https://schema.org/Subscription>
    Subscription,
    ///<https://schema.org/ActivationFee>
    ActivationFee,
    ///<https://schema.org/Installment>
    Installment,
}
///<https://schema.org/IncentiveQualifiedExpenseType>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum IncentiveQualifiedExpenseTypeEnum {
    ///<https://schema.org/IncentiveQualifiedExpenseTypeGoodsOrServices>
    IncentiveQualifiedExpenseTypeGoodsOrServices,
    ///<https://schema.org/IncentiveQualifiedExpenseTypeServicesOnly>
    IncentiveQualifiedExpenseTypeServicesOnly,
    ///<https://schema.org/IncentiveQualifiedExpenseTypeUtilityBill>
    IncentiveQualifiedExpenseTypeUtilityBill,
    ///<https://schema.org/IncentiveQualifiedExpenseTypeGoodsOnly>
    IncentiveQualifiedExpenseTypeGoodsOnly,
}
///<https://schema.org/IncentiveType>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum IncentiveTypeEnum {
    ///<https://schema.org/IncentiveTypeTaxDeduction>
    IncentiveTypeTaxDeduction,
    ///<https://schema.org/IncentiveTypeRebateOrSubsidy>
    IncentiveTypeRebateOrSubsidy,
    ///<https://schema.org/IncentiveTypeTaxWaiver>
    IncentiveTypeTaxWaiver,
    ///<https://schema.org/IncentiveTypeTaxCredit>
    IncentiveTypeTaxCredit,
    ///<https://schema.org/IncentiveTypeLoan>
    IncentiveTypeLoan,
}
///<https://schema.org/DigitalDocumentPermissionType>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum DigitalDocumentPermissionTypeEnum {
    ///<https://schema.org/ReadPermission>
    ReadPermission,
    ///<https://schema.org/CommentPermission>
    CommentPermission,
    ///<https://schema.org/WritePermission>
    WritePermission,
}
///<https://schema.org/LegalForceStatus>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum LegalForceStatusEnum {
    ///<https://schema.org/NotInForce>
    NotInForce,
    ///<https://schema.org/InForce>
    InForce,
    ///<https://schema.org/PartiallyInForce>
    PartiallyInForce,
}
///<https://schema.org/GameAvailabilityEnumeration>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum GameAvailabilityEnumerationEnum {
    ///<https://schema.org/DemoGameAvailability>
    DemoGameAvailability,
    ///<https://schema.org/FullGameAvailability>
    FullGameAvailability,
}
///<https://schema.org/DrugCostCategory>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum DrugCostCategoryEnum {
    ///<https://schema.org/Retail>
    Retail,
    ///<https://schema.org/Wholesale>
    Wholesale,
    ///<https://schema.org/ReimbursementCap>
    ReimbursementCap,
}
///<https://schema.org/MapCategoryType>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum MapCategoryTypeEnum {
    ///<https://schema.org/SeatingMap>
    SeatingMap,
    ///<https://schema.org/TransitMap>
    TransitMap,
    ///<https://schema.org/VenueMap>
    VenueMap,
    ///<https://schema.org/ParkingMap>
    ParkingMap,
}
///<https://schema.org/SteeringPositionValue>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum SteeringPositionValueEnum {
    ///<https://schema.org/RightHandDriving>
    RightHandDriving,
    ///<https://schema.org/LeftHandDriving>
    LeftHandDriving,
}
///<https://schema.org/RsvpResponseType>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum RsvpResponseTypeEnum {
    ///<https://schema.org/RsvpResponseYes>
    RsvpResponseYes,
    ///<https://schema.org/RsvpResponseMaybe>
    RsvpResponseMaybe,
    ///<https://schema.org/RsvpResponseNo>
    RsvpResponseNo,
}
///<https://schema.org/MedicalDevicePurpose>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum MedicalDevicePurposeEnum {
    ///<https://schema.org/Diagnostic>
    Diagnostic,
    ///<https://schema.org/Therapeutic>
    Therapeutic,
}
///<https://schema.org/MusicAlbumReleaseType>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum MusicAlbumReleaseTypeEnum {
    ///<https://schema.org/EPRelease>
    EPRelease,
    ///<https://schema.org/AlbumRelease>
    AlbumRelease,
    ///<https://schema.org/SingleRelease>
    SingleRelease,
    ///<https://schema.org/BroadcastRelease>
    BroadcastRelease,
}
///<https://schema.org/MedicalImagingTechnique>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum MedicalImagingTechniqueEnum {
    ///<https://schema.org/PET>
    PET,
    ///<https://schema.org/Radiography>
    Radiography,
    ///<https://schema.org/Ultrasound>
    Ultrasound,
    ///<https://schema.org/XRay>
    XRay,
    ///<https://schema.org/CT>
    CT,
    ///<https://schema.org/MRI>
    MRI,
}
///<https://schema.org/Boolean>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum BooleanEnum {
    ///<https://schema.org/False>
    False,
    ///<https://schema.org/True>
    True,
}
///<https://schema.org/ContactPointOption>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum ContactPointOptionEnum {
    ///<https://schema.org/HearingImpairedSupported>
    HearingImpairedSupported,
    ///<https://schema.org/TollFree>
    TollFree,
}
///<https://schema.org/MedicalProcedureType>
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[serde(untagged)]
pub enum MedicalProcedureTypeEnum {
    ///<https://schema.org/NoninvasiveProcedure>
    NoninvasiveProcedure,
    ///<https://schema.org/PercutaneousProcedure>
    PercutaneousProcedure,
}
