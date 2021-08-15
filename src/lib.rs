pub mod parser;

#[macro_export]
macro_rules! hashmap {
    ( $($key:expr, $value:expr), *) => {
        {
            let mut temp = std::collections::hash_map::HashMap::new();
            $(
                temp.insert($key, $value);
            )*
            temp
        }
    }
}
