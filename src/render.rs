//! Support for rendering pipeline elements.
use std::collections::HashMap;

use crate::DVCError;
use crate::{schemas::pipeline::{StageEntry, PipelineStage}, interpolate::{InterpContext, Interpolatable}};

pub fn expand_entries(specs: &HashMap<String,StageEntry>) -> Result<HashMap<String, PipelineStage>, DVCError>  {
  let mut out = HashMap::new();

  for (name, entry) in specs {
    match entry {
      StageEntry::SingleStage(stage) => {
        out.insert(name.to_owned(), stage.clone());
      },
      StageEntry::MultiStage { foreach, stage } => {
        for label in foreach {
          let key = format!("{}@{}", name, label);
          let mut map = HashMap::new();
          map.insert("item", label.as_str());
          let ctx = InterpContext::create(&map);
          let stage = stage.interpolate(&ctx)?;
          out.insert(key, stage);
        }
      },
    }
  }

  Ok(out)
}
