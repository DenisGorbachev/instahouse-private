#[allow(dead_code)]
pub use AssetName::*;
use strum::Display;

#[derive(Display, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum AssetName {
    USD,
    THB,
    RUB,
    CNY,
}

impl AssetName {}
