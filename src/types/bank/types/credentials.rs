use crate::{LoginBuf, Password};
use derive_getters::Getters;
use derive_more::{From, Into};
use derive_new::new;
use proptest_derive::Arbitrary;

#[derive(new, Getters, From, Into, Arbitrary, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Clone, Debug)]
pub struct Credentials {
    login: LoginBuf,
    password: Password,
}

impl Credentials {}
