use derive_getters::Getters;
use derive_more::{From, Into};
use derive_new::new;

/// TODO: When the land is registered, the land department issues a Chanote
/// TODO: When a condo unit is registered, the land department issues a Chanote as well
#[derive(new, Getters, From, Into, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Clone, Debug)]
pub struct ThailandLandDepartment {}

impl ThailandLandDepartment {}
