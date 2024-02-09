use pgrx::pg_extern;

#[pg_extern]
pub fn str_contains(input: &str, search: &str) -> bool {
    input.contains(search)
}

#[pg_extern]
pub fn str_contains_all(input: &str, search: Vec<&str>) -> bool {
    search.iter().all(|s| input.contains(s))
}

// #[cfg(any(test, feature = "pg_test"))]
// mod tests {
//     #[allow(unused_imports)]
//     use super::*;
//     use pgrx::pg_extern;
// }
