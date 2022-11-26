//! Library to parse DVC worktrees.
use std::collections::HashMap;
use std::path::{PathBuf, Path};
use std::io;
use thiserror::Error;

use pipeline::PipelineFile;
use relative_path::{RelativePathBuf};
use stage::DVCFile;
use walk::Src;

mod walk;
pub mod schemas;
pub mod stage;
pub mod pipeline;

#[derive(Error, Debug)]
pub enum DVCError {
  /// IO error loading DVC data.
  #[error("I/O operation failed")]
  IO(#[from] io::Error),
  #[error("YAML parse error")]
  Parse(#[from] serde_yaml::Error),
  #[error("invalid path encountered")]
  BadPath,
}

impl From<relative_path::FromPathError> for DVCError {
  fn from(_: relative_path::FromPathError) -> Self {
    DVCError::BadPath
  }
}

/// Implementation of a DVC tree.
pub struct DVCTree {
  pub root: PathBuf,
  pub dvcfiles: Vec<DVCFile>,
  pub pipelines: Vec<PipelineFile>,
}

impl DVCTree {
  pub fn scan_tree<P: AsRef<Path>>(path: P) -> Result<DVCTree, DVCError> {
    let sources = walk::scan_tree(path.as_ref())?;
    let mut dvcfiles = Vec::new();
    let mut pipelines = Vec::new();
    let mut lockfiles = HashMap::new();

    let root = RelativePathBuf::from_path(path.as_ref())?;

    // process files
    for (sfp, src) in sources.into_iter() {
      let path = RelativePathBuf::from_path(sfp)?;
      let path = root.relative(path);
      match src {
        Src::Single(stage) => {
          dvcfiles.push(DVCFile {
            path, stage,
          });
        },
        Src::Pipe(spec) => {
          pipelines.push(PipelineFile { path, spec, lock: None });
        },
        Src::Lock(lf) => {
          lockfiles.insert(path, lf);
        }
      }
    }

    // match pipelines with lockfiles
    for pf in pipelines.iter_mut() {
      let lp = pf.path.with_extension(".lock");
      if let Some(lf) = lockfiles.get(&lp) {
        pf.lock = Some(lf.clone())
      }
    }

    Ok(DVCTree {
      root: path.as_ref().to_path_buf(),
      dvcfiles,
      pipelines,
    })
  }
}
