use strum::Display;
#[allow(dead_code)]
pub use NaturalMessageRole::*;

#[derive(Display, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum NaturalMessageRole {
    Client,
    Agent,
}

impl NaturalMessageRole {}
