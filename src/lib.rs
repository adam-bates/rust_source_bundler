mod error;

use error::BundleError;

use std::{
    fs,
    path::PathBuf,
};

use anyhow::Result;

pub fn bundle_source(root_filepath: impl Into<PathBuf>) -> Result<String> {
    let root_code = fs::read_to_string(root_filepath.into())?;

    return Ok(root_code);
}

