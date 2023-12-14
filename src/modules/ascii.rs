use any_ascii::any_ascii;
use pgrx::prelude::*;

#[pg_extern]
pub fn str_ascii(input: &str) -> String {
    any_ascii(input)
}

#[pg_extern]
pub fn str_is_ascii(input: &str) -> bool {
    input.is_ascii()
}

#[cfg(any(test, feature = "pg_test"))]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    use pgrx::prelude::*;
}
