use thiserror::Error;

#[derive(Error, Debug)]
pub enum TypstError {
    #[error("Template error: {0}")]
    TemplateError(String),
    #[error("Schema validation error: {0}")]
    SchemaValidationError(String),
    #[error("Render error: {0}")]
    RenderError(String),
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Invalid input: {0}")]
    InvalidInput(String),
}
