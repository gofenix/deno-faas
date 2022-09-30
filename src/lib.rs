use std::rc::Rc;

use deno_core::{anyhow::Result, error::AnyError, serde_v8, v8, JsRuntime};
use deno_core::{Extension, FsModuleLoader, RuntimeOptions};
use serde::de::DeserializeOwned;

use crate::ops::file_op::{op_read_file, op_remove_file, op_write_file};

pub mod ops;

pub async fn eval<T>(rt: &mut JsRuntime, code: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    let ret = rt.execute_script("<anon>", code)?;
    let result = rt.resolve_value(ret).await?;
    let scope = &mut rt.handle_scope();
    let result = v8::Local::new(scope, result);
    Ok(serde_v8::from_v8(scope, result)?)
}

pub async fn run_js(code: String, params: String) -> Result<serde_json::Value, AnyError> {
    let exts = Extension::builder()
        .ops(vec![
            op_read_file::decl(),
            op_write_file::decl(),
            op_remove_file::decl(),
        ])
        .build();

    let options = RuntimeOptions {
        module_loader: Some(Rc::new(FsModuleLoader)),
        extensions: vec![exts],
        ..Default::default()
    };

    let mut rt = JsRuntime::new(options);
    const RUNTIME_JAVASCRIPT_CORE: &str = include_str!("runtime.js");
    rt.execute_script("[runjs:runtime.js]", RUNTIME_JAVASCRIPT_CORE)?;

    rt.execute_script("[example_handler.js]", &code)?;

    let handler = format!(
        r#"
        const req = JSON.parse({:?});
        handler(req);
    "#,
        serde_json::to_string(&params)?
    );
    let ret: serde_json::Value = eval(&mut rt, &handler).await?;
    Ok(ret)
}
