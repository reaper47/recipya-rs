use schemars::JsonSchema;

#[derive(Debug, PartialEq, JsonSchema)]
pub enum LoanOrCreditOrPaymentMethodOrText {
    LoanOrCredit(LoanOrCredit),
    PaymentMethod(PaymentMethod),
    Text(String),
}

impl Default for LoanOrCreditOrPaymentMethodOrText {
    fn default() -> Self {
        Self::Text(String::new())
    }
}
