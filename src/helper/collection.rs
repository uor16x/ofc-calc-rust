use std::collections::HashMap;

// todo: rewrite module using generics
pub fn slice_get_by_index<T>(items: &[T], index: usize) -> Option<&T> {
    items.iter().nth(index)
}

pub fn hashmap_get_key_by_value<K: Copy, T: PartialEq>(map: &HashMap<K, T>, value_seek: T) -> Option<K> {
    map
        .iter()
        .filter(|&(key, value)| value == &value_seek)
        .map(|(&key, value)| key)
        .next()
}

pub fn hashmap_key_max_by_value<K: Copy, T: Ord>(map: &HashMap<K, T>) -> &K {
    let f = | &(_, a_value): &(&K, &T), &(_, b_value): &(&K, &T) |
        a_value.cmp(&b_value);
    map
        .iter()
        .max_by(f)
        .map(|(key, _)| key)
        .unwrap()
}

pub fn hashmap_key_min_by_value<K: Copy, T: Ord>(map: &HashMap<K, T>) -> &K {
    let f = | &(_, a_value): &(&K, &T), &(_, b_value): &(&K, &T) |
        a_value.cmp(&b_value);
    map
        .iter()
        .min_by(f)
        .map(|(key, _)| key)
        .unwrap()
}

