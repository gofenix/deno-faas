

use deno_core::error::AnyError;




use serde::{Deserialize, Serialize};


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
