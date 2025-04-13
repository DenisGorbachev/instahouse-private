use derive_more::{Error, From};
use fmt_derive::Display;

#[derive(Error, Display, From, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub enum PersonValidationError {
    PersonHasNoPassport(PersonHasNoPassportError),
}

vec_of_enum::define!(
    #[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
    pub struct PersonValidationErrors(Vec<PersonValidationError>);
    variants = [PersonHasNoPassportError];
);

pub type IndexedPersonValidationErrors = Indexed<PersonValidationErrors>;

impl PersonValidationError {}

mod person_has_no_passport_error;

use crate::Indexed;
pub use person_has_no_passport_error::*;
