use pgrx::prelude::*;

use rand::distributions::{Alphanumeric, DistString};

use any_ascii::any_ascii;
use inflector::cases::{
    camelcase, kebabcase, pascalcase, screamingsnakecase, snakecase, titlecase,
};
use inflector::string::{pluralize, singularize};
use pulldown_cmark::{html, Options, Parser};
use str_slug::StrSlug;
use uuid::Uuid;

pgrx::pg_module_magic!();

#[pg_extern]
fn str_random(length: i32) -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), length as usize)
}

#[pg_extern]
fn str_length(input: &str) -> i32 {
    input.len() as i32
}

#[pg_extern]
fn str_after<'a>(input: &'a str, search: &str) -> &'a str {
    let matches: Vec<_> = input.match_indices(search).collect();
    match matches.first() {
        None => input,
        Some(x) => &input[x.1.len()..],
    }
}
// #[pg_extern]
// fn str_after_last<'a>(input: &'a str, search: &str) -> &'a str {
// }

// fn str_before<'a>(input: &'a str, search: &str) -> &'a str {
// }
// fn str_beforeLast<'a>(input: &'a str, search: &str) -> &'a str {
// }
// fn str_between<'a>(input: &'a str, search: &str) -> &'a str {
// }

#[pg_extern]
fn str_uuid() -> String {
    Uuid::new_v4().to_string()
}

#[pg_extern]
fn str_ascii(input: &str) -> String {
    any_ascii(input)
}

#[pg_extern]
fn str_is_ascii(input: &str) -> bool {
    input.is_ascii()
}

#[pg_extern]
fn str_contains(input: &str, search: &str) -> bool {
    input.contains(search)
}

#[pg_extern]
fn str_contains_all(input: &str, search: Vec<&str>) -> bool {
    search.iter().all(|s| input.contains(s))
}

#[pg_extern]
fn str_lower(input: &str) -> String {
    input.to_lowercase()
}

#[pg_extern]
fn str_upper(input: &str) -> String {
    input.to_uppercase()
}

#[pg_extern]
fn str_slug(input: &str, sep: char) -> String {
    let mut slug = StrSlug::new();
    slug.separator = sep;
    slug.slug(input)
}

#[pg_extern]
fn str_singular(input: &str) -> String {
    singularize::to_singular(input)
}

#[pg_extern]
fn str_plural(input: &str) -> String {
    pluralize::to_plural(input)
}

#[pg_extern]
fn str_title(input: &str) -> String {
    titlecase::to_title_case(input)
}

#[pg_extern]
fn str_camel(input: &str) -> String {
    camelcase::to_camel_case(input)
}

#[pg_extern]
fn str_kebab(input: &str) -> String {
    kebabcase::to_kebab_case(input)
}

#[pg_extern]
fn str_snake(input: &str) -> String {
    snakecase::to_snake_case(input)
}

#[pg_extern]
fn str_studly(input: &str) -> String {
    pascalcase::to_pascal_case(input)
}

#[pg_extern]
fn str_scream(input: &str) -> String {
    screamingsnakecase::to_screaming_snake_case(input)
}

#[pg_extern]
fn str_markdown(input: &str) -> String {
    // Set up options and parser. Strikethroughs are not part of the CommonMark standard
    // and we therefore must enable it explicitly.
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(input, options);

    // Write to String buffer.
    let mut html_output: String = String::with_capacity(input.len() * 3 / 2);
    html::push_html(&mut html_output, parser);
    html_output
}

#[pg_extern]
fn str_substr(input: &str, start: i32, end: i32) -> &str {
    &input[start as usize..end as usize]
}
#[pg_extern]
fn str_replace(input: &'static str, old: &'static str, new: &'static str) -> String {
    input.replace(old, new)
}

#[pg_extern]
fn str_append(mut input: String, extra: &str) -> String {
    input.push_str(extra);
    input
}

#[pg_extern]
fn str_split(input: &'static str, pattern: &str) -> Vec<&'static str> {
    input.split_terminator(pattern).into_iter().collect()
}

#[pg_extern]
fn str_split_set<'a>(input: &'a str, pattern: &'a str) -> SetOfIterator<'a, &'a str> {
    SetOfIterator::new(input.split_terminator(pattern).into_iter())
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {

    // #[pg_test]
    // fn test_hello_pg_str() {
    //     assert_eq!("Hello, pg_str", crate::hello_pg_str());
    // }
}

#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
