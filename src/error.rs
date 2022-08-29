use thiserror::Error;

#[derive(Error, Debug)]
pub enum BundleError {
    #[error("Unexpected")]
    Unexpected(String),
}

