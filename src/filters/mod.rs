mod render_comment;
pub use render_comment::RenderComment;

mod camelcase_to_snakecase;
pub use camelcase_to_snakecase::CamelcaseToSnakecase;

mod snakecase_to_camelcase;
pub use snakecase_to_camelcase::SnakecaseToCamelcase;

mod escape_rust_keyword;
pub use escape_rust_keyword::EscapeRustKeyword;

mod escape_c_keyword;
pub use escape_c_keyword::EscapeCKeyword;

mod escape_cpp_keyword;
pub use escape_cpp_keyword::EscapeCppKeyword;

mod escape_js_keyword;
pub use escape_js_keyword::EscapeJsKeyword;

pub fn invalid_input<S>(cause: S) -> liquid_core::Error
where
    S: Into<liquid_core::model::KString>,
{
    liquid_core::Error::with_msg("Invalid input").context("cause", cause)
}

pub(crate) fn all() -> Vec<Box<dyn liquid_core::parser::ParseFilter>> {
    vec![
        crate::filters::RenderComment.into(),
        crate::filters::CamelcaseToSnakecase.into(),
        crate::filters::SnakecaseToCamelcase.into(),
        crate::filters::EscapeRustKeyword.into(),
        crate::filters::EscapeCKeyword.into(),
        crate::filters::EscapeCppKeyword.into(),
        crate::filters::EscapeJsKeyword.into(),
    ]
}
