use std::collections::HashMap;
use serde::Deserialize;
use serde::de::Deserializer;

use crate::interpolate::{Interpolatable, InterpContext};

use super::{DepFile, OutFile};

mod filelist;

use filelist::FLEntry;

/// Reprsentation for pipeline files (`dvc.yaml`).
#[derive(Debug, Clone, Deserialize)]
pub struct Pipeline {
  pub stages: HashMap<String,StageEntry>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum StageEntry {
  SingleStage(PipelineStage),
  MultiStage {
    foreach: Vec<String>,
    #[serde(rename="do")]
    stage: PipelineStage,
  }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PipelineStage {
  pub cmd: String,
  #[serde(default)]
  pub wdir: Option<String>,
  #[serde(deserialize_with="deserialize_deps", default)]
  pub deps: Vec<DepFile>,
  #[serde(deserialize_with="deserialize_outs", default)]
  pub outs: Vec<OutFile>,
  #[serde(deserialize_with="deserialize_outs", default)]
  pub metrics: Vec<OutFile>,
}

impl Interpolatable for PipelineStage {
  fn interpolate(&self, context: &InterpContext<'_>) -> Self::Owned {
    PipelineStage {
      cmd: context.interpolate(&self.cmd),
      wdir: self.wdir.as_ref().map(|s| context.interpolate(s.as_str())),
      deps: self.deps.iter().map(|d| d.interpolate(context)).collect(),
      outs: self.outs.iter().map(|d| d.interpolate(context)).collect(),
      metrics: self.metrics.iter().map(|d| d.interpolate(context)).collect(),
    }
  }
}

fn deserialize_deps<'de, D>(de: D) -> Result<Vec<DepFile>, D::Error> where D: Deserializer<'de> {
  let deps: Vec<FLEntry<DepFile>> = Deserialize::deserialize(de)?;
  let deps: Vec<_> = deps.into_iter().map(|f| f.to_artifact()).collect();
  Ok(deps)
}

fn deserialize_outs<'de, D>(de: D) -> Result<Vec<OutFile>, D::Error> where D: Deserializer<'de> {
  let deps: Vec<FLEntry<OutFile>> = Deserialize::deserialize(de)?;
  let deps: Vec<_> = deps.into_iter().map(|f| f.to_artifact()).collect();
  Ok(deps)
}
