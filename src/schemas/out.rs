use relative_path::{RelativePathBuf, RelativePath};
use serde::Deserialize;
use super::PathRecord;

/// Representation of an output file.
#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct OutFile {
  pub path: RelativePathBuf,
  pub cache: bool,
  pub md5: Option<String>,
  pub size: Option<usize>,
}

impl Default for OutFile {
  fn default() -> Self {
    OutFile { path: "".into(), cache: true, md5: None, size: None }
  }
}

impl PathRecord for OutFile {
  fn path(&self) -> &RelativePath {
    &self.path
  }

  fn set_path(&mut self, path: &RelativePath) {
    self.path = path.to_owned();
  }

  fn md5(&self) -> Option<&str> {
    self.md5.as_ref().map(|s| s.as_str())
  }
}
