use deno_core::{
    anyhow::{anyhow, Result},
    error::AnyError,
    op, resolve_url_or_path, serde_v8, v8, JsRuntime,
};
use serde::de::DeserializeOwned;

#[op]
async fn op_read_file(path: String) -> Result<String, AnyError> {
    let contents = tokio::fs::read_to_string(path).await?;
    Ok(contents)
}

#[op]
async fn op_write_file(path: String, contents: String) -> Result<(), AnyError> {
    tokio::fs::write(path, contents).await?;
    Ok(())
}

#[op]
fn op_remove_file(path: String) -> Result<(), AnyError> {
    std::fs::remove_file(path)?;
    Ok(())
}
