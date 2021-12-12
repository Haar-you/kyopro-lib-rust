#[macro_export]
macro_rules! hashset {
    ($($a:expr),*) => {{
        let mut a = std::collections::HashSet::new();
        a.extend(vec![$($a),*]);
        a
    }}
}

#[macro_export]
macro_rules! btreeset {
    ($($a:expr),*) => {{
        let mut a = std::collections::BTreeSet::new();
        a.extend(vec![$($a),*]);
        a
    }}
}

#[macro_export]
macro_rules! hashmap {
    ($($key:expr => $val:expr),*) => {{
        let mut a = std::collections::HashMap::new();
        a.extend(vec![$(($key, $val)),*]);
        a
    }}
}

#[macro_export]
macro_rules! btreemap {
    ($($key:expr => $val:expr),*) => {{
        let mut a = std::collections::BTreeMap::new();
        a.extend(vec![$(($key, $val)),*]);
        a
    }}
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        dbg!(hashset!(1, 2, 3, 3, 4));
        dbg!(btreeset!(1, 2, 3, 3, 4));
        dbg!(hashmap!(1 => "a", 2 => "b", 3 => "c", 3 => "d", 4 => "e"));
        dbg!(btreemap!(1 => "a", 2 => "b", 3 => "c", 3 => "d", 4 => "e"));
    }
}
