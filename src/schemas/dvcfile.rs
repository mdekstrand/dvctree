use serde::Deserialize;

use super::DepFile;
use super::OutFile;

/// A `.dvc` file typically representing an input file.
///
/// In modern DVC, `.dvc` files store a single file as an output; they may store
/// a dependency for e.g. web download.  Early versions of DVC used these files
/// for the entire pipeline.  Since the dependency and output schemas are the
/// same, there should be no problem with using `dvcfile` for basic reading of
/// repositories with such files.
#[derive(Debug, Clone, Deserialize)]
pub struct DvcFile {
  pub md5: Option<String>,
  pub outs: Vec<OutFile>,
  pub deps: Vec<DepFile>,
}