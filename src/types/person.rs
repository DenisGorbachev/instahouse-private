use crate::Credentials;
use derive_getters::Getters;
use derive_more::{From, Into};
use derive_new::new;
use std::rc::Rc;

#[derive(new, Getters, From, Into, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Clone, Debug)]
pub struct Person {
    credentials: Vec<Rc<Credentials>>,
}

impl Person {
    pub fn gen_credentials(&mut self) -> Rc<Credentials> {
        todo!()
    }
}
