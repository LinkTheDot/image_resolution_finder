use chrono::Utc;
use log::LevelFilter;
use log4rs::{
  append::file::FileAppender,
  config::{Appender, Config, Root},
  encode::pattern::PatternEncoder,
};
use std::env;
use std::str::FromStr;

const DEFAULT_LOG_LEVEL: &str = "info";
const DEFAULT_LOG_FORMAT: &str = "short";

/// Creates a new log file in "crate/logs/".
/// The new log file will be named after the current time and date based on UTC.
/// The name format is as such Y-M-D-H:M:S-UTC or Year-Month-Day-Hour:Minute:Second-TimeZone.
///
/// The logging level will be configured with the `LOG_FORMAT` environment variable.
/// If that doesn't exist, the default will be `short`.
///
/// The logging level will be configured with the `LOG_LEVEL` environment variable.
/// If that doesn't exist, the default is `info`.
///
/// long: "(Hour:Minute:Second)(TimeZone) | FilePath: Line | Level - Message".
/// short: "FilePath: Line | Level - Message".
/// shortest: Level - "Message"
pub fn setup_file_logger() -> Result<log4rs::Handle, Box<dyn std::error::Error>> {
  let log_level = get_logging_level()?;
  let logging_format = get_logging_format();

  let date = Utc::now().to_string().replace(':', "-");
  let log_file_path = format!("logs/{date}.log").replace(' ', "-");

  let logfile = FileAppender::builder()
    .encoder(Box::new(PatternEncoder::new(&logging_format)))
    .build(log_file_path)?;

  let config = Config::builder()
    .appender(Appender::builder().build("logfile", Box::new(logfile)))
    .build(Root::builder().appender("logfile").build(log_level))?;

  log4rs::init_config(config).map_err(Into::into)
}

fn get_logging_level() -> anyhow::Result<LevelFilter> {
  let level_string = env::var("LOG_LEVEL").unwrap_or(DEFAULT_LOG_LEVEL.to_string());

  LevelFilter::from_str(&level_string).map_err(Into::into)
}

/// To get the list of possible fields refer to the docs listed below:
/// https://docs.rs/log4rs/1.2.0/log4rs/encode/pattern/index.html#formatters
fn get_logging_format() -> String {
  let format_level = if let Ok(format_level) = env::var("LOG_FORMAT") {
    format_level
  } else if let Ok(format_level) = env::var("LOG_FORMAT") {
    format_level
  } else {
    DEFAULT_LOG_FORMAT.to_string()
  };

  match format_level.trim() {
    "long" => "{d(%H:%M:%S %Z)(utc)} | {f}: {L} | {l} - {m}\n",
    "short" => "{d(%H:%M:%S %Z)(utc)} | {l} - {m}\n",
    "shortest" => "{m}\n",
    _ => "{d(%H:%M:%S %Z)(utc)} | {l} - {m}\n",
  }
  .into()
}
