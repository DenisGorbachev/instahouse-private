use crate::LoginBuf;
use derive_more::{Error, From};
use fmt_derive::Display;

#[derive(Error, Display, From, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub enum SignupError {
    LoginAlreadyExists(#[error(not(source))] LoginBuf),
}
impl SignupError {}
