use relative_path::{RelativePath, RelativePathBuf};
use serde::Deserialize;

use crate::interpolate::{Interpolatable, InterpContext, InterpError};

use super::PathRecord;

/// Representation of a dependency file.
#[derive(Debug, Default, Clone, Deserialize)]
#[serde(default)]
pub struct DepFile {
  pub path: RelativePathBuf,
  pub wdir: Option<String>,
  pub md5: Option<String>,
  pub size: Option<usize>,
}

impl PathRecord for DepFile {
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

impl Interpolatable for DepFile {
  fn interpolate(&self, context: &InterpContext<'_>) -> Result<DepFile, InterpError> {
    let mut dep = self.clone();
    dep.path = self.path.interpolate(context)?;
    Ok(dep)
  }
}
