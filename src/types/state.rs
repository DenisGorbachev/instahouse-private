use crate::{Bank, Person};
use derive_getters::Getters;
use derive_more::{From, Into};
use derive_new::new;
use std::rc::Rc;

#[derive(new, Getters, From, Into, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Clone, Debug)]
pub struct State {
    persons: Vec<Rc<Person>>,
    banks: Vec<Rc<Bank>>,
}

impl State {
    pub fn init() -> Self {
        let clint = Rc::new(Person::default());
        let kasikorn = Rc::new(Bank::default());
        let persons = vec![clint];
        let banks = vec![kasikorn];
        Self {
            persons,
            banks,
        }
    }
}
