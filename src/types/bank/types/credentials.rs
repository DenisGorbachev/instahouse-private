use crate::{LoginBuf, PasswordBuf};
use derive_getters::Getters;
use derive_more::{From, Into};
use derive_new::new;
use proptest_derive::Arbitrary;
use rand::distr::{Distribution, StandardUniform};
use rand::Rng;

#[derive(new, Getters, From, Into, Arbitrary, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Clone, Debug)]
pub struct Credentials {
    login: LoginBuf,
    password: PasswordBuf,
}

impl Credentials {}

impl Distribution<Credentials> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, _rng: &mut R) -> Credentials {
        todo!()
    }
}
