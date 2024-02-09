use pgrx::pg_extern;

#[pg_extern]
fn str_length(input: &str) -> i32 {
    input.len() as i32
}

// #[cfg(any(test, feature = "pg_test"))]
// mod tests {
//     #[allow(unused_imports)]
//     use super::*;
//     use pgrx::prelude::*;
// }
