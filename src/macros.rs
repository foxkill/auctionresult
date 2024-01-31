//! # Macros
//! 
//! This file contains all macro implementations used in this project.
//
macro_rules! hashmap {
    ($($key:expr => $value:expr),*) => ({
        let mut map = HashMap::new();
        $(map.insert($key, $value);)*
        map
    });
}

pub(crate) use hashmap;