use pgrx::prelude::*;

#[pg_extern]
pub fn str_split<'a>(input: &'a str, pattern: &str) -> Vec<&'a str> {
    input.split_terminator(pattern).into_iter().collect()
}

#[pg_extern]
pub fn str_split_set<'a>(input: &'a str, pattern: &'a str) -> SetOfIterator<'a, &'a str> {
    SetOfIterator::new(input.split_terminator(pattern).into_iter())
}

#[cfg(any(test, feature = "pg_test"))]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    use pgrx::prelude::*;
}
