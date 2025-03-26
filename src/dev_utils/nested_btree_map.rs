use std::collections::BTreeMap;

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
