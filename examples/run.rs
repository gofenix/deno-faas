use std::rc::Rc;

use deno_core::error::AnyError;
use deno_core::Extension;
use deno_core::{FsModuleLoader, JsRuntime, RuntimeOptions};
use deno_faas::eval;
use deno_faas::ops::file_op::{op_read_file, op_remove_file, op_write_file};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub code: String,
}

#[tokio::main]
async fn main() -> Result<(), AnyError> {
    let code = include_str!("./example.js");
    let params = Message {
        code: "hello run it".to_string(),
    };

    let ret = deno_faas::run_js(code.to_string(), serde_json::to_string(&params)?).await?;
    println!("rust: {:?}", ret);
    Ok(())
}
