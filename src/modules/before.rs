use pgrx::pg_extern;

#[pg_extern]
pub fn str_before<'a>(input: &'a str, search: &str) -> &'a str {
    local::str_before(input, search)
}

#[pg_extern]
pub fn str_before_last<'a>(input: &'a str, search: &str) -> &'a str {
    local::str_before_last(input, search)
}

mod local {
    pub fn str_before<'a>(input: &'a str, search: &str) -> &'a str {
        if search.is_empty() {
            return input;
        }
        match input.find(search) {
            Some(index) => &input[..index],
            None => input,
        }
    }

    pub fn str_before_last<'a>(input: &'a str, search: &str) -> &'a str {
        match input.rfind(search) {
            Some(index) => &input[..index],
            None => input,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_before() {
        assert_eq!(local::str_before("Hello, world!", "world"), "Hello, ");
        assert_eq!(local::str_before("Hello, world!", "xyz"), "Hello, world!");
        assert_eq!(local::str_before("", "world"), "");
        assert_eq!(local::str_before("Hello, world!", ""), "Hello, world!");
    }

    #[test]
    fn test_str_before_last() {
        assert_eq!(local::str_before_last("Hello, world!", "o"), "Hello, w");
        assert_eq!(
            local::str_before_last("Hello, world!", "xyz"),
            "Hello, world!"
        );
        assert_eq!(local::str_before_last("", "world"), "");
        assert_eq!(local::str_before_last("Hello, world!", ""), "Hello, world!");
    }
}
