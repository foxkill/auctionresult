//! # Macros
//! 
//! This file contains all macro implementations used in this project.
//

#[allow(unused)]
macro_rules! hashmap {
    ($($key:expr => $value:expr),*) => ({
        let mut map = HashMap::new();
        $(map.insert($key, $value);)*
        map
    });
}
#[allow(unused)]
pub(crate) use hashmap;