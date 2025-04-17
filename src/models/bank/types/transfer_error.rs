use crate::{Amount, Credentials};
use derive_more::{Error, From};
use fmt_derive::Display;

#[derive(Error, Display, From, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
#[non_exhaustive]
pub enum TransferError {
    InvalidCredentials(#[error(not(source))] Credentials),
    InsufficientAmount(#[error(not(source))] Amount),
    // there may be more errors
}
impl TransferError {}
