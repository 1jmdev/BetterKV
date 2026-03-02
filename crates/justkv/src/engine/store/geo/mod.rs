mod geo_math;
mod read_ops;
mod search_ops;
mod write_ops;

use ahash::RandomState;
use hashbrown::HashMap;

use crate::engine::value::{CompactKey, Entry, GeoValue};

pub struct GeoSearchMatch {
    pub member: CompactKey,
    pub longitude: f64,
    pub latitude: f64,
    pub distance_meters: Option<f64>,
}

fn get_geo(entry: &Entry) -> Option<&GeoValue> {
    entry.as_geo()
}

fn get_geo_mut(entry: &mut Entry) -> Option<&mut GeoValue> {
    entry.as_geo_mut()
}

fn new_geo() -> GeoValue {
    HashMap::with_hasher(RandomState::new())
}
