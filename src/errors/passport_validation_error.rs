use derive_more::{Error, From};
use fmt_derive::Display;

#[derive(Error, Display, From, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub enum PassportValidationError {
    IssuedOnNotInThePast(IssuedOnNotInThePastError),
}

vec_of_enum::define!(
    #[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
    pub struct PassportValidationErrors(Vec<PassportValidationError>);
);

impl PassportValidationError {}

mod issued_on_not_in_the_past_error;

pub use issued_on_not_in_the_past_error::*;
