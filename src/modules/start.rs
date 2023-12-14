use pgrx::prelude::*;
use regex::Regex;

#[pg_extern]
pub fn str_start(value: &str, prefix: &str) -> String {
    let quoted = regex::escape(prefix);
    let re = Regex::new(&format!("^(?:{})+", quoted)).unwrap();

    format!("{}{}", prefix, re.replace(value, ""))
}

#[cfg(any(test, feature = "pg_test"))]
#[pgrx::pg_schema]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    use pgrx::prelude::*;

    #[pg_test]
    fn test_no_slash_prefix() {
        let result = Spi::get_one::<String>("SELECT public.str_start('path/to/file', '/')");
        assert_eq!(result, Ok(Some("/path/to/file".to_string())));
    }
    #[pg_test]
    fn test_slash_prefix() {
        let result = Spi::get_one::<String>("SELECT public.str_start('/path/to/file', '/')");
        assert_eq!(result, Ok(Some("/path/to/file".to_string())));
    }
}
