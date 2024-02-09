use pgrx::pg_extern;

#[pg_extern]
pub fn str_substr(input: &str, start: i32, end: i32) -> &str {
    &input[start as usize..end as usize]
}

// #[cfg(any(test, feature = "pg_test"))]
// mod tests {
//     #[allow(unused_imports)]
//     use super::*;
//     use pgrx::prelude::*;
// }
