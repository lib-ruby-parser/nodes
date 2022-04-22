mod render_comment;
pub use render_comment::RenderComment;

mod camelcase_to_snakecase;
pub use camelcase_to_snakecase::CamelcaseToSnakecase;

mod snakecase_to_camelcase;
pub use snakecase_to_camelcase::SnakecaseToCamelcase;

mod escape_rust_keyword;
pub use escape_rust_keyword::EscapeRustKeyword;

pub fn invalid_input<S>(cause: S) -> liquid_core::Error
where
    S: Into<liquid_core::model::KString>,
{
    liquid_core::Error::with_msg("Invalid input").context("cause", cause)
}
