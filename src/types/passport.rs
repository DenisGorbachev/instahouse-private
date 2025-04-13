use crate::types::passport_number::PassportNumber;
use crate::{Country, IssuedOn, IssuedOnNotInThePastError, PassportValidationErrors, Validate};
use derive_getters::Getters;
use derive_more::{From, Into};
use derive_new::new;
use time::OffsetDateTime;

#[derive(new, Getters, From, Into, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub struct Passport {
    number: PassportNumber,
    country: Country,
    issued_on: IssuedOn,
}

impl Passport {}

impl Validate for Passport {
    type Errors = PassportValidationErrors;

    fn validate(&self, now: OffsetDateTime) -> PassportValidationErrors {
        let mut errors = PassportValidationErrors::default();
        if self.issued_on > now {
            errors.push(IssuedOnNotInThePastError::new(self.issued_on));
        }
        errors
    }
}
