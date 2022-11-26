use std::collections::HashMap;
use serde::Deserialize;
use serde::de::Deserializer;

use crate::interpolate::{Interpolatable, InterpContext, InterpError};

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

#[derive(Debug, Clone, Default, Deserialize)]
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
  fn interpolate(&self, context: &InterpContext<'_>) -> Result<Self::Owned, InterpError>  {
    let mut stage = PipelineStage::default();
    stage.cmd = context.interpolate(&self.cmd)?;
    stage.wdir = self.wdir.as_ref().map(|s| context.interpolate(s.as_str())).transpose()?;
    for dep in &self.deps {
      stage.deps.push(dep.interpolate(context)?);
    }
    for out in &self.outs {
      stage.outs.push(out.interpolate(context)?);
    }
    for metric in &self.metrics {
      stage.metrics.push(metric.interpolate(context)?);
    }

    Ok(stage)
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
