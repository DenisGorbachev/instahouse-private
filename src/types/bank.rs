use derive_getters::Getters;
use derive_more::{From, Into};
use derive_new::new;
use std::rc::Rc;

#[derive(new, Getters, From, Into, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Clone, Debug)]
pub struct Bank {
    name: String,
    users: Vec<Rc<User>>,
    accounts: Vec<Rc<Account>>,
}

impl Bank {
    pub fn signup(&mut self, _credentials: Credentials) -> Result<(), SignupError> {
        todo!()
    }

    /// The caller must ensure that the money has been physically moved
    pub fn deposit(&mut self, _credentials: &Credentials, _account_index: usize, _amount: Amount) -> Result<(), DepositError> {
        todo!()
    }

    /// The caller must ensure that the money has been physically moved
    pub fn withdraw(&mut self, _credentials: &Credentials, _account_index: usize, _amount: Amount) -> Result<(), WithdrawError> {
        todo!()
    }

    pub fn transfer(&mut self, _from: &Login, _to: &Login, _amount: Amount, _password: PasswordBuf) -> Result<(), TransferError> {
        todo!()
    }
}

mod types;

pub use types::*;
