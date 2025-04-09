use strum::Display;
#[allow(dead_code)]
pub use AssetName::*;

#[derive(Display, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum AssetName {
    USD,
    THB,
    RUB,
    CNY,
}

impl AssetName {}
