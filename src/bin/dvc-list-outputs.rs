//! Scan a DVC repository in the current directory.
use std::path::PathBuf;
use dvctree::{DVCTree, DVCError};

fn main() -> Result<(), DVCError> {
  let root = PathBuf::from(".");
  let tree = DVCTree::scan_tree(&root)?;
  for out in tree.outputs() {
    println!("output: {:?}", out);
  }

  Ok(())
}
