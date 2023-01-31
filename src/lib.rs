use napi::Result;
use napi_derive::napi;
use std::process::Stdio;
use tokio::process::Command;

#[napi]
pub async fn test() -> Result<String> {
  node_version().await
}

pub async fn node_version() -> Result<String> {
  let mut cmd = Command::new("node");
  cmd.arg("--version");
  cmd.stdout(Stdio::inherit());
  cmd.stderr(Stdio::inherit());

  let mut child = cmd.spawn().unwrap();
  child.wait().await.unwrap();

  Ok("ok".into())
}
