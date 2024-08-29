use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Default, Clone, Copy)]
pub enum ImagePreference {
  Wide,
  Tall,
  #[default]
  None,
}

impl From<ImagePreference> for String {
  fn from(value: ImagePreference) -> Self {
    match value {
      ImagePreference::Wide => "wide",
      ImagePreference::Tall => "tall",
      ImagePreference::None => "none",
    }
    .to_string()
  }
}

impl From<String> for ImagePreference {
  fn from(value: String) -> Self {
    match value.to_lowercase().trim() {
      "wide" => Self::Wide,
      "tall" => Self::Tall,
      "none" | "nothing" | "nil" | "null" | "nul" | "" => Self::None,
      _ => {
        log::error!("Unknown format: {:?}", value);

        panic!();
      }
    }
  }
}
