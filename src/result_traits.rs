pub trait ResultTraits<T, E> {
  /// Converts any Error type into an anyhow::Result<T>
  fn coerce(self) -> anyhow::Result<T>
  where
    E: std::error::Error + Send + Sync + 'static;

  fn log_if_err(self, custom_message: &str) -> Option<T>
  where
    E: std::fmt::Debug;
}

impl<T, E> ResultTraits<T, E> for Result<T, E> {
  fn coerce(self) -> anyhow::Result<T>
  where
    E: std::error::Error + Send + Sync + 'static,
  {
    self.map_err(Into::into)
  }

  fn log_if_err(self, custom_message: &str) -> Option<T>
  where
    E: std::fmt::Debug,
  {
    if let Err(error) = self {
      log::error!("{}: '{:?}'", custom_message, error);

      return None;
    }

    self.ok()
  }
}
