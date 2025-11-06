use crate::permutations::text::TextOrURL;
use crate::thing::intangible::quantity::duration::Duration;
use crate::thing::intangible::service::FinancialProduct;
use crate::thing::intangible::structured_value::QuantitativeValue;
use schemars::JsonSchema;
use serde::Deserialize;

/// A financial product for the loaning of an amount of money, or line of credit, under agreed
/// terms and charges.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LoanOrCredit {
    /// The amount of money.
    pub amount: MonetaryAmountOrNumber,
    /// The currency in which the monetary amount is expressed.
    ///
    /// Use standard formats: ISO 4217 currency format, e.g. "USD"; Ticker symbol for
    /// cryptocurrencies, e.g. "BTC"; well known names for Local Exchange Trading Systems (LETS) and
    /// other currency types, e.g. "Ithaca HOUR".
    pub currency: String,
    /// The period of time after any due date that the borrower has to fulfil its obligations before
    /// a default (failure to pay) is deemed to have occurred.
    pub grace_period: Duration,
    /// A form of paying back money previously borrowed from a lender. Repayment usually takes the
    /// form of periodic payments that normally include part principal plus interest in each
    /// payment.
    pub loan_repayment_form: RepaymentSpecification,
    /// The duration of the loan or credit agreement.
    pub loan_term: QuantitativeValue,
    /// The type of a loan or credit.
    pub loan_type: TextOrURL,
    /// The only way you get the money back in the event of default is the security. Recourse is
    /// where you still have the opportunity to go back to the borrower for the rest of the money.
    pub recourse_loan: bool,
    /// Whether the terms for payment of interest can be renegotiated during the life of the loan.
    pub renegotiable_loan: bool,
    /// Assets required to secure loan or credit repayments. It may take form of third party pledge,
    /// goods, financial instruments (cash, securities, etc.)
    pub required_collateral: TextOrThing,
    #[serde(flatten)]
    pub financial_product: FinancialProduct,
}
