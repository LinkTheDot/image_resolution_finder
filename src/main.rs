#![allow(unused)]

use futures::FutureExt;
use image_resolution_finder::{logger, run};

#[tokio::main]
async fn main() {
  let _ = logger::setup_file_logger();

  if let Err(error) = run().await {
    log::error!("An error occurred when running the program: {:?}", error);

    std::process::exit(1);
  } else {
    std::process::exit(0);
  }
}
