use std::collections::HashMap;

pub fn hashmap_get_key_by_value<K: Copy, T: PartialEq>(map: &HashMap<K, T>, value_seek: T) -> Option<K> {
    map
        .iter()
        .filter(|&(_, value)| value == &value_seek)
        .map(|(&key, _)| key)
        .next()
}

pub fn hashmap_key_max_by_value<K: Copy, T: Ord>(map: &HashMap<K, T>) -> &K {
    let f = | &(_, a_value): &(&K, &T), &(_, b_value): &(&K, &T) |
        b_value.cmp(&a_value);
    map
        .iter()
        .max_by(f)
        .map(|(key, _)| key)
        .unwrap()
}

pub fn hashmap_key_min_by_value<K: Copy, T: Ord>(map: &HashMap<K, T>) -> &K {
    let f = | &(_, a_value): &(&K, &T), &(_, b_value): &(&K, &T) |
        b_value.cmp(&a_value);
    map
        .iter()
        .min_by(f)
        .map(|(key, _)| key)
        .unwrap()
}

