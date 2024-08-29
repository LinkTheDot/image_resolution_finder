use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Default, Clone, Copy)]
pub enum DataFormat {
  #[default]
  Names,
  Copies,
}

impl From<DataFormat> for String {
  fn from(value: DataFormat) -> Self {
    match value {
      DataFormat::Names => "names",
      DataFormat::Copies => "copies",
    }
    .to_string()
  }
}

impl From<String> for DataFormat {
  fn from(value: String) -> Self {
    match value.to_lowercase().trim() {
      "names" | "name" => Self::Names,
      "copies" | "copy" => Self::Copies,
      _ => {
        log::error!("Unknown format: {:?}", value);

        panic!();
      }
    }
  }
}
