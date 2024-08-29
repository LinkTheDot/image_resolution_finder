use crate::config::*;
use crate::result_traits::*;
use image::GenericImageView;
use image_preference::*;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::RwLock;
use tokio::task::JoinHandle;

pub mod config;
pub mod format;
pub mod image_preference;
pub mod logger;
pub mod result_traits;

const CONFIG_FILE_NAME: &str = "image_finder_config.yml";

pub async fn run() -> anyhow::Result<()> {
  log::info!("Creating default configuration.");
  let mut config = Config::default();
  let mut config_file = OpenOptions::new()
    .write(true)
    .read(false)
    .create(true)
    .truncate(true)
    .open(CONFIG_FILE_NAME)?;

  log::info!("Serializing config.");
  let serialized_default_config = serde_yml::to_string(&config)?;

  log::info!("Writing config to `{:?}`", CONFIG_FILE_NAME);
  write!(config_file, "{}", serialized_default_config)?;
  // Drop the file handle because no shared write access.
  drop(config_file);

  log::info!("Waiting for user input.");
  let input = wait_for_input();
  log::debug!("Selected input: {:?}", input);

  match input {
    rfd::MessageDialogResult::Ok => {
      log::info!("Updating config.");
      let config_file_contents = fs::read_to_string(CONFIG_FILE_NAME)?;
      config = serde_yml::from_str::<Config>(&config_file_contents)?;

      log::info!("Running with config data: {:?}", config);

      log::info!("Beginning image search.");

      let directories = match config.directories.take() {
        Some(directories) => directories,
        None => vec![std::env::current_dir()?],
      };

      let image_paths = get_image_paths_from_directories_recursively(directories).await;

      act_upon_desired_images(config, image_paths).await;

      log::info!("Process finished.");
    }
    rfd::MessageDialogResult::Cancel => {
      log::info!("Operation canceled.");
      log::warn!("Deleting config file {:?}", CONFIG_FILE_NAME);
    }
    _ => unreachable!(),
  }

  fs::remove_file(CONFIG_FILE_NAME).coerce()
}

fn wait_for_input() -> rfd::MessageDialogResult {
  rfd::MessageDialog::new()
    .set_title("Image resolution finder config prompt.")
    .set_description(
      include_str!(concat!(env!("PWD"), "/message_prompt_description.txt"))
        .replace("{CONFIG_FILE_NAME}", CONFIG_FILE_NAME),
    )
    .set_buttons(rfd::MessageButtons::OkCancel)
    .show()
}

#[async_recursion::async_recursion]
async fn get_image_paths_from_directories_recursively(directories: Vec<PathBuf>) -> Vec<PathBuf> {
  let mut future_handles = vec![];
  let mut image_paths = vec![];

  for path in directories {
    if !path.exists() || !path.is_dir() {
      log::warn!(
        "Attempted to read a path that doesn't exist or isn't a directory: {:?}",
        path
      );

      continue;
    }

    let directory_contents = match fs::read_dir(&path) {
      Ok(contents) => contents,
      Err(error) => {
        log::error!("Failed to read path {:?}. Reason: {:?}", path, error);

        continue;
      }
    };

    for entry in directory_contents {
      let path = match entry {
        Ok(entry) => entry.path(),
        Err(error) => {
          log::error!("Failed to read path {:?}. Reason: {:?}", path, error);

          continue;
        }
      };

      if path.is_dir() {
        log::info!("Reading directory {:?}", path);
        let handle = tokio::spawn(get_image_paths_from_directories_recursively(vec![path]));

        future_handles.push(handle);
      } else if path_is_image(&path) {
        image_paths.push(path);
      }
    }
  }

  for result in future_handles {
    match result.await {
      Ok(mut result) => image_paths.append(&mut result),
      Err(error) => log::error!("Failed to get contents of a directory. Reason: {:?}", error),
    }
  }

  image_paths
}

fn path_is_image(path: &Path) -> bool {
  let mime = mime_guess::MimeGuess::from_path(path).first();

  match mime {
    Some(mime) => mime.type_() == "image",
    None => false,
  }
}

async fn act_upon_desired_images(config: Config, paths: Vec<PathBuf>) {
  let mut future_handles: Vec<JoinHandle<()>> = vec![];
  let min_width = config.min_width.unwrap();
  let min_height = config.min_height.unwrap();
  let preference = config.preference.unwrap_or_default();
  let Some(destination) = config.copy_destination else {
    log::error!("Attempted to copy images to a destination that wasn't defined.");

    return;
  };

  if !destination.exists() {
    if let Err(error) = fs::create_dir_all(&destination) {
      log::error!(
        "Error creating image copy destination: {:?}. Operation will still continue.",
        error
      );
    }
  }

  let destination = Arc::new(RwLock::new(destination));

  for path in paths {
    let destination = destination.clone();

    let handle = tokio::spawn(async move {
      let image = match image::open(&path) {
        Ok(image) => image,
        Err(error) => {
          log::warn!("Failed to open image {:?}. Reason: {:?}", path, error);

          return;
        }
      };

      let dimensions = image.dimensions();
      let Some(file_name) = &path.file_name() else {
        log::error!("Failed to get file name of {:?}", path);

        return;
      };
      let destination = destination.read().unwrap().join(file_name);

      drop(image);

      if dimensions.0 < min_width || dimensions.1 < min_height {
        return;
      }

      match preference {
        ImagePreference::None => (),
        ImagePreference::Tall => {
          if dimensions.0 > dimensions.1 {
            return;
          }
        }
        ImagePreference::Wide => {
          if dimensions.0 < dimensions.1 {
            return;
          }
        }
      }

      if let Err(error) = fs::copy(&path, &destination) {
        log::error!(
          "Failed to copy file {:?} to {:?}. Reason: {:?}",
          path,
          destination,
          error
        );
      } else {
        log::debug!("Copying to {:?}", destination);
      }
    });

    future_handles.push(handle);
  }

  for result in future_handles {
    if let Err(error) = result.await {
      log::error!("An error occurred when handling a task: {:?}", error);
    }
  }
}
