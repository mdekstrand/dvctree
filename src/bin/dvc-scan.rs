//! Scan a DVC repository in the current directory.
use std::path::PathBuf;
use dvctree::{DVCTree, DVCError};

fn main() -> Result<(), DVCError> {
  let root = PathBuf::from(".");
  let tree = DVCTree::scan_tree(&root)?;
  print!("Stages:\n");
  for stage in &tree.dvcfiles {
    print!("- {:?}\n", stage);
  }

  print!("Pipelines:\n");
  for pipe in &tree.pipelines {
    print!("- {:?}\n", pipe);
  }

  Ok(())
}
