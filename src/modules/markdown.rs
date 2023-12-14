use pgrx::prelude::*;

use pulldown_cmark::{html, Options, Parser};

#[pg_extern]
pub fn str_markdown(input: &str) -> String {
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

#[cfg(any(test, feature = "pg_test"))]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    use pgrx::prelude::*;
}
