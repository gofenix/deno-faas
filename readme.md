# deno faas run time

your js function like this

```js
async function handler(context) {
    console.log(context);

    console.log("Hello", "runjs!");
    console.error("Boom!");
    
    const path = "./log.txt";
    try {
      const contents = await runjs.readFile(path);
      console.log("Read from a file", contents);
    } catch (err) {
      console.error("Unable to read file", path, err);
    }
    
    await runjs.writeFile(path, "I can write to a file.");
    const contents = await runjs.readFile(path);
    console.log("Read from a file", path, "contents:", contents);
    console.log("Removing file", path);
    runjs.removeFile(path);
    console.log("File removed");

    return {
        code: 200, 
        data: ["nonce", "geek"]
    }
}
```

function name should be handler.

function argument should be context.

and it should have a return.

# used in your project

```rs
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

```