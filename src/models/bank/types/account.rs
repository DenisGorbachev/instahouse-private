use crate::{Amount, AssetName, User};
use derive_getters::Getters;
use derive_more::{From, Into};
use derive_new::new;
use std::rc::Rc;

#[derive(new, Getters, From, Into, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub struct Account {
    owner: Rc<User>,
    asset: AssetName,
    amount: Amount,
}
impl Account {}
