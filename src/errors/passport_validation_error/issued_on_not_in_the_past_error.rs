use crate::IssuedOn;
use derive_more::{Error, From, Into};
use derive_new::new;
use fmt_derive::Display;

#[derive(new, Error, Display, From, Into, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub struct IssuedOnNotInThePastError {
    issued_on: IssuedOn,
}
impl IssuedOnNotInThePastError {}
