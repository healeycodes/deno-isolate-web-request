use deno_core::error::AnyError;
use deno_core::op;
use deno_core::serde::Serialize;
use deno_core::Extension;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use rouille::Response;
use std::fs::File;
use std::io::{Read, Write};
use std::rc::Rc;
use std::vec;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestResponse {
    status: u16,
    headers: Vec<(String, String)>,
    url: String,
    body: String,
}

#[op]
async fn op_request(
    method: String,
    url: String,
    headers: Vec<(String, String)>,
    body: String,
) -> Result<RequestResponse, AnyError> {
    match method.as_str() {
        "GET" => {
            let client = reqwest::Client::new();
            let mut req = client.get(&url);
            for (key, value) in headers.iter() {
                req = req.header(key, value);
            }
            let res = req.send().await?;

            let mut res_headers: Vec<(String, String)> = vec![];
            for (key, value) in res.headers().iter() {
                res_headers.push((
                    key.to_string(),
                    String::from_utf8_lossy(value.as_bytes()).to_string(),
                ))
            }

            Ok(RequestResponse {
                status: res.status().into(),
                headers: res_headers,
                url,
                body: res.text().await?,
            })
        }
        "POST" => {
            let client = reqwest::Client::new();
            let mut req = client.post(&url);
            for (key, value) in headers.iter() {
                req = req.header(key, value);
            }
            let res = req.body(body).send().await?;

            let mut res_headers: Vec<(String, String)> = vec![];
            for (key, value) in res.headers().iter() {
                res_headers.push((
                    key.to_string(),
                    String::from_utf8_lossy(value.as_bytes()).to_string(),
                ))
            }

            Ok(RequestResponse {
                status: res.status().into(),
                headers: res_headers,
                url,
                body: res.text().await?,
            })
        }
        // Checked in src/minijs.js
        _ => unreachable!(),
    }
}

#[op]
async fn op_log(text: String) -> Result<(), AnyError> {
    println!("{:?}", text);
    Ok(())
}

#[op]
async fn op_err_log(text: String) -> Result<(), AnyError> {
    println!("{:?}", text);
    Ok(())
}

// See https://deno.com/blog/roll-your-own-javascript-runtime
async fn run_js(file_path: &str) -> Result<(), AnyError> {
    let main_module = deno_core::resolve_path(file_path)?;
    let runjs_extension = Extension::builder("some name")
        .ops(vec![op_request::decl(), op_log::decl(), op_err_log::decl()])
        .build();
    let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
        module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
        extensions: vec![runjs_extension],
        ..Default::default()
    });
    js_runtime
        .execute_script("[minijs.js]", include_str!("./minijs.js"))
        .unwrap();

    let mod_id = js_runtime.load_main_module(&main_module, None).await?;
    let result = js_runtime.mod_evaluate(mod_id);
    js_runtime.run_event_loop(false).await?;
    result.await?
}

fn run_user_code(source: Vec<u8>) -> Result<(), AnyError> {
    let rnd_file_name: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();
    let script_path = format!("./{}", rnd_file_name);

    let mut file = File::create(&script_path)?;
    file.write_all(&source)?;

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    runtime.block_on(run_js(&script_path))
}

fn main() {
    rouille::start_server("localhost:3000", move |request| {
        let mut data = request.data().expect("Oops, body already retrieved");
        let mut buf = Vec::new();
        match data.read_to_end(&mut buf) {
            Ok(_) => (),
            Err(_) => return Response::text("Failed to read body"),
        };

        match run_user_code(buf) {
            Ok(value) => return Response::text(format!("{:?}", value)),
            Err(err) => return Response::text(err.to_string()),
        }
    });
}
