mod loan_or_credit;

pub use loan_or_credit::*;

use crate::permutations::text::TextOrURL;
use schemars::JsonSchema;
use serde::Deserialize;

/// A product provided to consumers and businesses by financial institutions such as banks,
/// insurance companies, brokerage firms, consumer finance companies, and investment companies which
/// comprise the financial services industry.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct FinancialProduct {
    /// The annual rate that is charged for borrowing (or made by investing), expressed as a single
    /// percentage number that represents the actual yearly cost of funds over the term of a loan.
    /// This includes any fees or additional costs associated with the transaction.
    pub annual_percentage_rate: NumberOrQuantitativeValue,
    /// Description of fees, commissions, and other terms applied either to a class of financial
    /// product, or by a financial service organization.
    pub fees_and_commissions_specification: TextOrURL,
    /// The interest rate, charged or paid, applicable to the financial product. Note: This is
    /// different from the calculated annualPercentageRate.
    pub interest_rate: NumberOrQuantitativeValue,
    #[serde(flatten)]
    pub service: Service,
}
