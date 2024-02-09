pub mod modules {
    pub mod after;
    pub mod ascii;
    pub mod before;
    pub mod case;
    pub mod contains;
    pub mod length;
    pub mod markdown;
    pub mod pad;
    pub mod random;
    pub mod split;
    pub mod start;
    pub mod substr;
    pub mod uuid;
}

use pgrx::prelude::*;

pub use modules::after::*;
pub use modules::ascii::*;
pub use modules::before::*;
pub use modules::case::*;
pub use modules::contains::*;
pub use modules::length::*;
pub use modules::markdown::*;
pub use modules::pad::*;
pub use modules::random::*;
pub use modules::split::*;
pub use modules::start::*;
pub use modules::substr::*;
pub use modules::uuid::*;

pgrx::pg_module_magic!();
// fn str_between<'a>(input: &'a str, search: &str) -> &'a str {
// }

#[pg_extern]
fn str_replace<'a>(input: &'a str, old: &'a str, new: &'a str) -> String {
    input.replace(old, new)
}

#[pg_extern]
fn str_append(mut input: String, extra: &str) -> String {
    input.push_str(extra);
    input
}

#[cfg(any(test, feature = "pg_test"))]
#[pgrx::pg_schema]
mod tests {

    // #[pg_test]
    // fn test_hello_pg_str() {
    //     assert_eq!("Hello, pg_str", crate::hello_pg_str());
    // }
}

#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
