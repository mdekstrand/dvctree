use std::collections::HashMap;

use serde::Deserialize;

use super::{OutFile, DepFile};

/// Schema for a DVC lockfile.
#[derive(Debug, Clone, Deserialize)]
pub struct LockFile {
  pub schema: String,
  pub stages: HashMap<String, LockStage>,
}

/// A single stage in a lockfile.
#[derive(Debug, Clone, Deserialize)]
pub struct LockStage {
  pub cmd: String,
  pub deps: Vec<DepFile>,
  pub outs: Vec<OutFile>,
}
