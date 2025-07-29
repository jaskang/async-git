#![deny(clippy::all)]

use napi::bindgen_prelude::*;
use napi_derive::napi;
use tokio::process::Command;

#[napi]
pub async fn run(commands: Vec<Vec<String>>) -> Result<String> {
  let output = Command::new("echo")
    .arg("hello")
    .arg("world")
    .output()
    .await?;
  Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}
