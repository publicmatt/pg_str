use pgrx::prelude::*;

use inflector::cases::{
    camelcase, kebabcase, pascalcase, screamingsnakecase, snakecase, titlecase,
};
use inflector::string::{pluralize, singularize};
use str_slug::StrSlug;

#[pg_extern]
pub fn str_lower(input: &str) -> String {
    input.to_lowercase()
}

#[pg_extern]
pub fn str_upper(input: &str) -> String {
    input.to_uppercase()
}

#[pg_extern]
pub fn str_slug(input: &str, sep: char) -> String {
    let mut slug = StrSlug::new();
    slug.separator = sep;
    slug.slug(input)
}

#[pg_extern]
pub fn str_singular(input: &str) -> String {
    singularize::to_singular(input)
}

#[pg_extern]
pub fn str_plural(input: &str) -> String {
    pluralize::to_plural(input)
}

#[pg_extern]
pub fn str_title(input: &str) -> String {
    titlecase::to_title_case(input)
}

#[pg_extern]
pub fn str_camel(input: &str) -> String {
    camelcase::to_camel_case(input)
}

#[pg_extern]
pub fn str_kebab(input: &str) -> String {
    kebabcase::to_kebab_case(input)
}

#[pg_extern]
pub fn str_snake(input: &str) -> String {
    snakecase::to_snake_case(input)
}

#[pg_extern]
pub fn str_studly(input: &str) -> String {
    pascalcase::to_pascal_case(input)
}

#[pg_extern]
pub fn str_scream(input: &str) -> String {
    screamingsnakecase::to_screaming_snake_case(input)
}

#[cfg(any(test, feature = "pg_test"))]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    use pgrx::prelude::*;
}
