use std::collections::HashMap;
use serde::Deserialize;
use serde::de::Deserializer;

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
  pub wdir: Option<String>,
  #[serde(deserialize_with="deserialize_deps")]
  pub deps: Vec<DepFile>,
  #[serde(deserialize_with="deserialize_outs")]
  pub outs: Vec<OutFile>,
  #[serde(deserialize_with="deserialize_outs")]
  pub metrics: Vec<OutFile>,
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
