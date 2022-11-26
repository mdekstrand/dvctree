//! Pipeline support.
use relative_path::{RelativePathBuf};
use crate::schemas::{Pipeline, LockState};

/// A pipeline, with its lock file (if available).
#[derive(Debug, Clone)]
pub struct PipelineFile {
  pub path: RelativePathBuf,
  pub spec: Pipeline,
  pub lock: Option<LockState>,
}
