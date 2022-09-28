#[allow(unused_macros)]
macro_rules! hashset(
{ $($key:expr),+ } => {
    {
        let mut m = ::std::collections::HashSet::new();
        $(
            m.insert($key);
        )+
        m
    }
    };
);
