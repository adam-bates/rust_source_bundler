mod bundler;

use bundler::Bundler;

use std::path::PathBuf;

use quote::ToTokens;

use rust_format::{
    Formatter,
    RustFmt,
};

use anyhow::Result;

pub fn bundle_source(root_dir: impl Into<PathBuf>, filename: String) -> Result<String> {
    let ast = Bundler::new(root_dir.into()).bundle_to_ast(filename)?;

    let token_stream = ast.into_token_stream();

    let generated_code = RustFmt::default().format_tokens(token_stream)?;

    return Ok(generated_code);
}

