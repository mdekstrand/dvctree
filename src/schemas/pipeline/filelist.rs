//! Parsing support for a list of files in a pipeline.
use std::marker::PhantomData;

use relative_path::RelativePathBuf;
use serde::de::{Deserializer, Visitor, Error};
use serde::Deserialize;

use crate::schemas::Artifact;

/// An entry in the file list.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum FLEntry<A> where A: Artifact + Default {
  /// An entry may be a bare filename.
  Bare(RelativePathBuf),
  /// An entry may be a mapping from a string to some path data.
  Mapped(SingleMap<A>),
}

impl <A: Artifact + Default> FLEntry<A> {
  pub fn to_artifact(self) -> A {
    match self {
      FLEntry::Bare(path) => {
        let mut art = A::default();
        art.set_path(&path);
        art
      },
      FLEntry::Mapped(sm) => sm.artifact,
    }
  }
}

/// Single-entry map type for map-style dep and output info.
#[derive(Debug)]
#[repr(transparent)]
pub struct SingleMap<A> where for<'a> A: Artifact + Default {
  pub artifact: A,
}

impl <'de, A> Deserialize<'de> for SingleMap<A> where A: Artifact + Default + Deserialize<'de> {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
      where
          D: Deserializer<'de> {
    let artifact = deserializer.deserialize_map(SMV {
      _ph: PhantomData::<A>
    })?;
    Ok(SingleMap { artifact })
  }
}

struct SMV<A> where A: Artifact + Default {
  _ph: PhantomData<A>
}

impl <'de, A> Visitor<'de> for SMV<A> where A: Artifact + Default + Deserialize<'de> {
  type Value = A;

  fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(formatter, "single-entry mapping")
  }

  fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
      where
          M: serde::de::MapAccess<'de>, {
    let entry = if let Some((name, entry)) = map.next_entry()? {
      let name: RelativePathBuf = name;
      let mut entry: A = entry;
      entry.set_path(&name);
      entry
    } else {
      return Err(Error::invalid_length(0, &self));
    };

    if let Some(key) = map.next_key()? {
      let _key: String = key;
      Err(Error::invalid_length(2, &self))
    } else {
      Ok(entry)
    }
  }
}
