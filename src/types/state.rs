use crate::{Bank, Person, PersonAction};
use derive_getters::Getters;
use derive_more::{From, Into};
use derive_new::new;
use rand::seq::SliceRandom;
use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(new, Getters, From, Into, Ord, PartialOrd, Eq, PartialEq, Default, Clone, Debug)]
pub struct State {
    persons: Vec<Rc<RefCell<Person>>>,
    banks: Vec<Rc<RefCell<Bank>>>,
}

impl State {
    pub fn init() -> Self {
        let clint = Rc::new(RefCell::new(Person::default()));
        let kasikorn = Rc::new(RefCell::new(Bank::default()));
        let persons = vec![clint];
        let banks = vec![kasikorn];
        Self {
            persons,
            banks,
        }
    }

    pub fn run(&mut self, rng: &mut impl Rng) {
        // We must randomize the order of actions because we want to execute actions sequentially, so some actions won't execute because they will be blocked by the results of other actions
        let mut indexes = (0..self.persons.len()).collect::<Vec<usize>>();
        indexes.shuffle(rng);
        indexes.into_iter().for_each(|i| {
            use PersonAction::*;
            let mut person = self.persons[i].borrow_mut();
            let action = person.act(rng);
            match action {
                GenerateCredentials(_action) => {
                    person.generate_credentials(rng);
                }
            }
        });
        if rng.random::<u8>() % 10 == 0 {
            let person = Person::default();
            self.persons.push(Rc::new(RefCell::new(person)));
        }
    }
}
