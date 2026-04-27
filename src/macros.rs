macro_rules! btreeset {
    ($($value:expr),*) => {
        { let mut set = std::collections::BTreeSet::new(); $(set.insert($value);)* set }
    };
}
