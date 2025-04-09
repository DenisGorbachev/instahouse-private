use crate::{Credentials, GenerateCredentialsAction, PersonAction};
use derive_getters::Getters;
use derive_more::{From, Into};
use derive_new::new;
use rand::Rng;
use std::rc::Rc;

#[derive(new, Getters, From, Into, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Clone, Debug)]
pub struct Person {
    credentials: Vec<Rc<Credentials>>,
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
