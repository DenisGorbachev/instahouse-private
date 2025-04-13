use crate::IndexedPersonValidationErrors;
use derive_more::From;
use fmt_derive::Display;
use std::error::Error;

#[derive(Display, From, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub enum StateValidationError {
    // TODO: should it be a Vec or HashMap? (the indexes must be unique)
    Persons(Vec<IndexedPersonValidationErrors>),
}

vec_of_enum::define!(
    #[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
    pub struct StateValidationErrors(Vec<StateValidationError>);
    variants = [Vec<IndexedPersonValidationErrors>];
);

impl Error for StateValidationError {}

impl StateValidationError {}
