use std::collections::BTreeMap;

pub const ALPHA: f64 = 0.05;
pub const BETA: f64 = 0.10;

pub const HI_1PCT_FACTOR: f64 = 1.01;
pub const HI_10PCT_FACTOR: f64 = 1.1;
pub const HI_25PCT_FACTOR: f64 = 1.25;

pub fn default_lo_stdev_log() -> f64 {
    1.2_f64.ln() / 2.0
}

pub fn default_hi_stdev_log() -> f64 {
    2.4_f64.ln() / 2.0
}

pub type NestedBTreeMap<T, U, V> = BTreeMap<T, BTreeMap<U, V>>;

pub fn nest_btree_map<T, U, V>(map: BTreeMap<(T, U), V>) -> NestedBTreeMap<T, U, V>
where
    T: Ord,
    U: Ord,
{
    let mut nested = NestedBTreeMap::<T, U, V>::new();
    for ((k1, k2), v) in map.into_iter() {
        let inner = nested.entry(k1).or_default();
        inner.insert(k2, v);
    }
    nested
}
