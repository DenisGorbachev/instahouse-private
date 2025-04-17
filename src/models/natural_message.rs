use derive_getters::Getters;
use derive_more::{From, Into};
use derive_new::new;

#[derive(new, Getters, From, Into, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub struct NaturalMessage {
    role: NaturalMessageRole,
    text: String,
}

impl NaturalMessage {}

mod natural_message_role;

pub use natural_message_role::*;
