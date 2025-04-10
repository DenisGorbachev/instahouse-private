use crate::LatLon;
use derive_more::{Error, From, Into};
use derive_new::new;
use fmt_derive::Display;

#[derive(new, Error, Display, From, Into, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct LineIntersectionError {
    pub a: (LatLon, LatLon),
    pub b: (LatLon, LatLon),
}

impl LineIntersectionError {}
