use crate::{LatLon, LineIntersectionError};
use derive_getters::Getters;
use derive_more::{From, Into};

#[derive(Getters, From, Into, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Clone, Debug)]
pub struct Polygon {
    points: Vec<LatLon>,
}

impl Polygon {
    pub fn new(_points: impl IntoIterator<Item = LatLon>) -> Result<Self, LineIntersectionError> {
        // TODO: any two lines must not intersect
        todo!()
    }
}
