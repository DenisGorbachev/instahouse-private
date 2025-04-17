use crate::Polygon;
use derive_getters::Getters;
use derive_more::{From, Into};
use derive_new::new;

// TODO: what about units within a single building? What about multiple buildings on one land plot?
#[derive(new, Getters, From, Into, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub struct Property {
    polygon: Polygon,
}

impl Property {}
