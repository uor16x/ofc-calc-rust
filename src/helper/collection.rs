use std::collections::HashMap;

pub fn hashmap_get_key_by_value<K: Copy, T: PartialEq>(map: &HashMap<K, T>, value_seek: T) -> Option<K> {
    map
        .iter()
        .filter(|&(_, value)| value == &value_seek)
        .map(|(&key, _)| key)
        .next()
}

