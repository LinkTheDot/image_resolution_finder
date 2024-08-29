use crate::image_preference::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
  pub min_width: Option<u32>,
  pub min_height: Option<u32>,
  pub copy_destination: Option<PathBuf>,
  pub preference: Option<ImagePreference>,
  pub directories: Option<Vec<PathBuf>>,
}

impl Default for Config {
  fn default() -> Self {
    Self {
      min_width: Some(0),
      min_height: Some(0),
      copy_destination: Some("image_data".into()),
      preference: Some(ImagePreference::None),
      directories: Some(vec!["First".into(), "second".into(), "the third".into()]),
    }
  }
}
