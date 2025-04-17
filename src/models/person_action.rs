use derive_more::From;

#[derive(From, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub enum PersonAction {
    GenerateCredentials(GenerateCredentialsAction),
}
impl PersonAction {}
mod generate_credentials_action;

pub use generate_credentials_action::*;
