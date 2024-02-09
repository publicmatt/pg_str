use pgrx::pg_extern;
use rand::distributions::{Alphanumeric, DistString};

#[pg_extern]
pub fn str_random(length: i32) -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), length as usize)
}

// #[cfg(any(test, feature = "pg_test"))]
// mod tests {
//     #[allow(unused_imports)]
//     use super::*;
//     use pgrx::pg_extern;
// }
