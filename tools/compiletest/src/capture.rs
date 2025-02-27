use std::fs;
use std::io::Error;
use std::path::Path;
use std::process::Child;
use std::process::Command;
use std::process::ExitStatus;
use std::process::Output;
use std::process::Stdio;

pub(crate) struct Capture {
  pub(crate) status: ExitStatus,
  pub(crate) stdout: String,
  pub(crate) stderr: String,
}

impl Capture {
  pub(crate) fn new(mut command: Command) -> Result<Self, Error> {
    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());
    command.stdin(Stdio::piped());

    let child: Child = command.spawn()?;
    let output: Output = child.wait_with_output()?;

    Ok(Self {
      status: output.status,
      stdout: String::from_utf8_lossy(output.stdout.as_slice()).into_owned(),
      stderr: String::from_utf8_lossy(output.stderr.as_slice()).into_owned(),
    })
  }

  pub(crate) fn check(&self) -> Result<(), Error> {
    if self.status.success() {
      return Ok(());
    }

    Err(Error::other(self.stderr.as_str()))
  }

  pub(crate) fn write(&self, path: &Path) -> Result<(), Error> {
    fs::write(path.join("out"), self.stdout.as_str())?;
    fs::write(path.join("err"), self.stderr.as_str())?;

    Ok(())
  }
}
