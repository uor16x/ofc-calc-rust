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

