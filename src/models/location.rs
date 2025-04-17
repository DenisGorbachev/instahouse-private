use crate::LatLon;
use derive_more::From;

#[derive(From, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub enum Location {
    Exact(LatLon),
}

impl Location {}
