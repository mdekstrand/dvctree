//! Library to parse DVC worktrees.
use std::collections::HashMap;
use std::path::{PathBuf, Path};
use std::io;
use render::expand_entries;
use thiserror::Error;

use pipeline::PipelineFile;
use relative_path::{RelativePathBuf};
use stage::DVCFile;
use walk::Src;

mod walk;
mod render;
pub mod interpolate;
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

/// A single DVC-managed output file.
///
/// DVC "outputs" include files that are simply tracked by DVC, not produced — they appear
/// in the `outs` of their `.dvc` files.
#[derive(Debug, Clone)]
pub struct DVCOutput {
  pub path: RelativePathBuf,
  pub cache: bool,
  pub md5: Option<String>,
  pub size: Option<usize>,
}

impl DVCTree {
  /// Scan a DVC Tree rooted at a filesystem path.
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

  /// Extract all of the DVC-managed outputs from the tree.
  pub fn outputs(&self) -> Vec<DVCOutput> {
    let mut outs = Vec::new();

    for file in &self.dvcfiles {
      let mut wdir = match file.path.parent() {
        Some(p) => p.to_relative_path_buf(),
        None => ".".into(),
      };
      if let Some(dir) = &file.stage.wdir {
        wdir = wdir.join_normalized(dir);
      }
      for out in &file.stage.outs {
        let path = wdir.join_normalized(&out.path);
        outs.push(DVCOutput { path, cache: out.cache, md5: out.md5.clone(), size: None })
      }
    }

    for pipe in &self.pipelines {
      let pipedir = match pipe.path.parent() {
        Some(p) => p.to_relative_path_buf(),
        None => ".".into(),
      };
      for (_name, stage) in expand_entries(&pipe.spec.stages) {
        let wdir = stage.wdir.map(|p| pipedir.join_normalized(p)).unwrap_or_else(|| pipedir.clone());
        for out in &stage.outs {
          let path = wdir.join_normalized(&out.path);
          outs.push(DVCOutput {
            path,
            cache: out.cache,
            md5: out.md5.clone(),
            size: None
          });
        }
      }
    }

    outs
  }
}
