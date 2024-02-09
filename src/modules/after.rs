use pgrx::pg_extern;

#[pg_extern]
pub fn str_after<'a>(input: &'a str, search: &str) -> &'a str {
    let matches: Vec<_> = input.match_indices(search).collect();
    match matches.first() {
        None => input,
        Some(x) => &input[x.1.len()..],
    }
}
// #[pg_extern]
// fn str_after_last<'a>(input: &'a str, search: &str) -> &'a str {
// }

// #[cfg(any(test, feature = "pg_test"))]
// mod tests {
//     #[allow(unused_imports)]
//     use super::*;
//     use pgrx::prelude::*;
// }
