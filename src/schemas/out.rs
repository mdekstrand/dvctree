use serde::Deserialize;
use super::Artifact;

/// Representation of an output file.
#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct OutFile {
  pub path: String,
  pub cache: bool,
  pub md5: Option<String>,
  pub size: Option<usize>,
}

impl Default for OutFile {
  fn default() -> Self {
    OutFile { path: "".into(), cache: true, md5: None, size: None }
  }
}

impl Artifact for OutFile {
  fn path(&self) -> &str {
    &self.path
  }

  fn set_path(&mut self, path: &str) {
    self.path = path.into();
  }

  fn md5(&self) -> Option<&str> {
    self.md5.as_ref().map(|s| s.as_str())
  }
}
