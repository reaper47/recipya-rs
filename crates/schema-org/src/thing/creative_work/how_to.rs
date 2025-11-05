use schemars::JsonSchema;
use serde::Deserialize;

use crate::thing::CreativeWork;
use crate::thing::intangible::quantity::duration::Duration;

/// Instructions that explain how to achieve a result by performing a sequence of steps.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct HowTo {
    /// The estimated cost of the supply or supplies consumed when performing instructions.
    pub estimated_cost: 	MonetaryAmountOrText,
    /// The length of time it takes to perform instructions or a direction (not including time to
    /// prepare the supplies), in ISO 8601 duration format.
    pub perform_time: 	Duration,
    /// The length of time it takes to prepare the items to be used in instructions or a direction,
    /// in ISO 8601 duration format.
    pub prep_time: 	Duration,
    /// A single step item (as HowToStep, text, document, video, etc.) or a HowToSection. Supersedes
    /// steps.
    pub step: 	CreativeWorkOrHowToSectionOrHowToStepOrText,
    /// A sub-property of instrument. A supply consumed when performing instructions or a direction.
    pub supply: 	HowToSupplyOrText,
    /// A sub property of instrument. An object used (but not consumed) when performing instructions
    /// or a direction.
    pub tool: 	HowToToolOrText,
    /// The total time required to perform instructions or a direction (including time to prepare
    /// the supplies), in ISO 8601 duration format.
    pub total_time: 	Duration,
    /// The quantity that results by performing instructions. For example, a paper airplane, 10
    /// personalized candles.
    pub r#yield: 	QuantitativeValueOrText,
    #[serde(flatten)]
    pub creative_work: CreativeWork,
}
