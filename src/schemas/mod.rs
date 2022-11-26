//! Data structures representing schemas of the various DVC files.
pub mod dep;
pub mod out;
pub mod dvcfile;
pub mod pipeline;
pub mod lockfile;

pub use dep::DepFile;
pub use out::OutFile;
pub use dvcfile::SingleStage;
pub use lockfile::LockState;
pub use pipeline::Pipeline;
use relative_path::RelativePath;

/// Traits implemented by DVC artifacts.
pub trait Artifact {
  /// Get the path of this artifact (as stored).
  fn path(&self) -> &RelativePath;

  /// Set the path of the artifact.
  fn set_path(&mut self, path: &RelativePath);

  /// Get the MD5 checksum of the artifact.
  fn md5(&self) -> Option<&str>;
}
