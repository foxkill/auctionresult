//! # Macros
//! 
//! This file contains all macro implementations used in this project.
//

#[allow(unused)]
macro_rules! enum_to_string {
    (enum $name:ident {
        $($variant:ident = $val:expr),*,
    }) => {
        enum $name {
            $($variant = $val),*
        }

        impl $name {
            fn name(&self) -> &'static str {
                match self {
                    $($name::$variant => stringify!($variant)),*
                }
            }
        }
    };
}

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