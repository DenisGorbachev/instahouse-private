use crate::Shared;
use derive_more::{From, Into};
use derive_new::new;
use std::cell::Ref;

#[derive(new, From, Into, Ord, PartialOrd, Eq, PartialEq, Default, Clone, Debug)]
pub struct View<T>(Shared<T>);

impl<T> View<T> {
    pub fn borrow(&self) -> Ref<'_, T> {
        self.0.borrow()
    }
}
