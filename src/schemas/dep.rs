use serde::Deserialize;

use super::Artifact;

/// Representation of a dependency file.
#[derive(Debug, Default, Clone, Deserialize)]
pub struct DepFile {
  pub path: String,
  pub wdir: Option<String>,
  pub md5: Option<String>,
  pub size: Option<usize>,
}

impl Artifact for DepFile {
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
