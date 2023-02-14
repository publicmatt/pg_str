use pgx::prelude::*;

use any_ascii::any_ascii;
use inflector::cases::{
    camelcase, kebabcase, pascalcase, screamingsnakecase, snakecase, titlecase,
};
use inflector::string::{pluralize, singularize};
use pulldown_cmark::{html, Options, Parser};
use rand::distributions::{Alphanumeric, DistString};
use str_slug::StrSlug;
use uuid::Uuid;


pgx::pg_module_magic!();

// #[pg_extern]
// fn str_random(length: u32) -> String {
//     Alphanumeric.sample_string(&mut rand::thread_rng(), length.try_into().unwrap())
// }

#[pg_extern]
fn after<'a>(input: &'a str, search: &str) -> &'a str {
    let matches: Vec<_> = input.match_indices(search).collect();
    match matches.first() {
        None => input,
        Some(x) => &input[x.1.len()..]
    }
}
#[pg_extern]
fn uuid() -> String {
    Uuid::new_v4().to_string()
}

#[pg_extern]
fn ascii(input: &str) -> String {
    any_ascii(input)
}

#[pg_extern]
fn is_ascii(input: &str) -> bool {
    input.is_ascii()
}

#[pg_extern]
fn contains(input: &str, search: &str) -> bool {
    input.contains(search)
}

#[pg_extern]
fn contains_all(input: &str, search: Vec<&str>) -> bool {
    search.iter().all(|s| input.contains(s))
}

#[pg_extern]
fn lower(input: &str) -> String {
    input.to_lowercase()
}

#[pg_extern]
fn upper(input: &str) -> String {
    input.to_uppercase()
}

#[pg_extern]
fn slug(input: &str, sep: char) -> String {
    let mut slug = StrSlug::new();
    slug.separator = sep;
    slug.slug(input)
}

#[pg_extern]
fn singular(input: &str) -> String {
    singularize::to_singular(input)
}

#[pg_extern]
fn plural(input: &str) -> String {
    pluralize::to_plural(input)
}

#[pg_extern]
fn title(input: &str) -> String {
    titlecase::to_title_case(input)
}

#[pg_extern]
fn camel(input: &str) -> String {
    camelcase::to_camel_case(input)
}

#[pg_extern]
fn kebab(input: &str) -> String {
    kebabcase::to_kebab_case(input)
}

#[pg_extern]
fn snake(input: &str) -> String {
    snakecase::to_snake_case(input)
}

#[pg_extern]
fn studly(input: &str) -> String {
    pascalcase::to_pascal_case(input)
}

#[pg_extern]
fn scream(input: &str) -> String {
    screamingsnakecase::to_screaming_snake_case(input)
}

#[pg_extern]
fn markdown(input: &str) -> String {
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
fn substr(input: &str, start: i32, end: i32) -> &str {
    &input[start as usize..end as usize]
}
#[pg_extern]
fn replace(input: &'static str, old: &'static str, new: &'static str) -> String {
    input.replace(old, new)
}

#[pg_extern]
fn append(mut input: String, extra: &str) -> String {
    input.push_str(extra);
    input
}

#[pg_extern]
fn split(input: &'static str, pattern: &str) -> Vec<&'static str> {
    input.split_terminator(pattern).into_iter().collect()
}

#[pg_extern]
fn split_set<'a>(input: &'a str, pattern: &'a str) -> SetOfIterator<'a, &'a str> {
    SetOfIterator::new(input.split_terminator(pattern).into_iter())
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgx::prelude::*;

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
