/// A single-stage DVC file.
use relative_path::RelativePathBuf;
use crate::schemas::SingleStage;

/// A file containing a single stage.
#[derive(Debug, Clone)]
pub struct DVCFile {
  pub path: RelativePathBuf,
  pub stage: SingleStage,
}
