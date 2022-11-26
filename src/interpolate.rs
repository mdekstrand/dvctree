//! String interpolation support.
use std::collections::HashMap;

use relative_path::{RelativePath, RelativePathBuf};
use text_template::{Template, TemplateError};

pub type InterpError = TemplateError;

/// Trait for objects that can have string interpolation applied.
pub trait Interpolatable: ToOwned {
  fn interpolate(&self, context: &InterpContext<'_>) -> Result<Self::Owned, InterpError>;
}

pub struct InterpContext<'a> {
  vars: HashMap<&'a str, &'a str>,
}

impl <'a> InterpContext<'a> {
  pub fn create<K, V>(map: &'a HashMap<K, V>) -> InterpContext<'a>
  where K: AsRef<str>, V: AsRef<str>
  {
    let vars = map.iter().map(|(k,v)| (k.as_ref(), v.as_ref())).collect();
    InterpContext { vars }
  }

  pub fn interpolate(&self, instr: &str) -> Result<String, InterpError> {
    let tmpl = Template::from(instr);
    let text = tmpl.try_fill_in(&self.vars)?;
    Ok(text.to_string())
  }
}

impl Interpolatable for RelativePath {
  fn interpolate(&self, context: &InterpContext<'_>) -> Result<RelativePathBuf, InterpError> {
    let text = self.to_string();
    let text = context.interpolate(&text)?;
    Ok(RelativePathBuf::from(text))
  }
}
