#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("Empty input")]
    EmptyInput,
    #[error("InvalidRowFormat")]
    InvalidRowFormat,
}
