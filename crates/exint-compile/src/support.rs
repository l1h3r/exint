use std::borrow::Cow;
use std::io::Error;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::process::Output;
use std::str;

fn locate_project() -> Result<Output, Error> {
  let output: Output = Command::new("cargo")
    .arg("locate-project")
    .arg("--workspace")
    .arg("--message-format")
    .arg("plain")
    .output()?;

  if output.status.success() {
    return Ok(output);
  }

  let content: &[u8] = output.stderr.as_slice();
  let message: Cow<'_, str> = String::from_utf8_lossy(content);

  Err(Error::other(message))
}

pub fn workspace_root() -> Result<PathBuf, Error> {
  let output: Output = locate_project()?;
  let stdout: &[u8] = output.stdout.as_slice();

  str::from_utf8(stdout)
    .ok()
    .map(str::trim)
    .map(Path::new)
    .and_then(Path::parent)
    .map(Path::to_path_buf)
    .ok_or_else(|| Error::other("failed to read path from `cargo locate-project`"))
}
