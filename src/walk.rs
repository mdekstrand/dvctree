use std::{error::Error, path::PathBuf};
use std::path::Path;
use std::collections::LinkedList;
use std::io;
use std::fs::{read_dir, File};

use relative_path::RelativePathBuf;
use serde_yaml::from_reader;

use crate::schemas::{SingleStage, Pipeline, LockState};
use crate::DVCError;

/// Recursively walk a directory and collect matching results.
pub fn filter_map_directory<P, R, E, F>(path: P, func: F) -> Result<Vec<R>, E>
where P: AsRef<Path>,
    E: Error + From<io::Error>,
    F: Fn(&Path) -> Result<Option<R>, E>
{
  // get an owned copy of the path
  let path = path.as_ref().to_path_buf();

  let mut results = Vec::new();
  let mut queue = LinkedList::new();
  queue.push_back(path);

  while let Some(path) = queue.pop_front() {
    let dir = read_dir(&path)?;
    for de in dir {
      let de = de?;
      let ft = de.file_type()?;

      if ft.is_dir() {
        queue.push_back(de.path());
      } else if let Some(r) = func(de.path().as_path())? {
        results.push(r);
      }
    }
  }

  Ok(results)
}

pub enum Src {
  Single(SingleStage),
  Pipe(Pipeline),
  Lock(LockState),
}

fn maybe_load(path: &Path) -> Result<Option<(PathBuf, Src)>, DVCError> {
  if let Some(name) = path.file_name() {
    let name = name.to_str().ok_or(DVCError::BadPath)?;
    if name == "dvc.yaml" {
      let read = File::open(path)?;
      let data = from_reader(read)?;
      Ok(Some((path.to_owned(), Src::Pipe(data))))
    } else if name == "dvc.lock" {
      let read = File::open(path)?;
      let data = from_reader(read)?;
      Ok(Some((path.to_owned(), Src::Lock(data))))
    } else if name.ends_with(".dvc") {
      let read = File::open(path)?;
      let data = from_reader(read)?;
      Ok(Some((path.to_owned(), Src::Single(data))))
    } else {
      Ok(None)
    }
  } else {
    Ok(None)
  }
}

pub fn scan_tree(path: &Path) -> Result<Vec<(PathBuf, Src)>, DVCError> {
  filter_map_directory(path, maybe_load)
}
