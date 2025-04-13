use crate::{Credentials, GenerateCredentialsAction, Passport, PersonAction, PersonHasNoPassportError, PersonValidationErrors, Validate};
use derive_getters::Getters;
use derive_more::{From, Into};
use derive_new::new;
use rand::Rng;
use std::rc::Rc;
use time::OffsetDateTime;

#[derive(new, Getters, From, Into, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Clone, Debug)]
pub struct Person {
    /// A person must have at least one passport
    passports: Vec<Rc<Passport>>,
    credentials: Vec<Rc<Credentials>>,
}

impl Validate for Person {
    type Errors = PersonValidationErrors;

    fn validate(&self, _now: OffsetDateTime) -> PersonValidationErrors {
        let mut errors = PersonValidationErrors::default();
        if self.passports.is_empty() {
            errors.push(PersonHasNoPassportError::new());
        }
        errors
    }
}

impl Person {
    pub fn generate_credentials(&mut self, rng: &mut impl Rng) -> Rc<Credentials> {
        let credentials: Credentials = rng.random();
        let rc = Rc::new(credentials);
        self.credentials.push(rc.clone());
        rc
    }

    pub fn act(&self, _rng: &mut impl Rng) -> PersonAction {
        // use PersonAction::*;
        GenerateCredentialsAction::new().into()
    }
}
