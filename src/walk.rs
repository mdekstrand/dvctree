use std::error::Error;
use std::path::Path;
use std::collections::LinkedList;
use std::io;
use std::fs::read_dir;

/// Recursively walk a directory and collect matching results.
pub fn walk_directory<P, R, E, F>(path: P, func: F) -> Result<Vec<R>, E>
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
