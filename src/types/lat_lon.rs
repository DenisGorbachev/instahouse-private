use derive_getters::Getters;
use derive_more::{From, Into};
use derive_new::new;
use ordered_float::OrderedFloat;

#[derive(new, Getters, From, Into, Ord, PartialOrd, Eq, PartialEq, Hash, Default, Clone, Copy, Debug)]
pub struct LatLon {
    pub latitude: OrderedFloat<f64>,
    pub longitude: OrderedFloat<f64>,
}

impl LatLon {}
