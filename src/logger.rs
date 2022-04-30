use std::env;

use super::log;
extern crate env_logger;
extern crate pretty_env_logger;

pub fn init(verbosity: u64) {
  let mut builder = pretty_env_logger::formatted_builder().unwrap();

  builder.filter_level(match verbosity {
    0 => log::LevelFilter::Warn,
    1 => log::LevelFilter::Info,
    _ => log::LevelFilter::Trace,
  });

  if let Ok(s) = env::var(env_logger::DEFAULT_FILTER_ENV) {
    builder.parse(&s);
  }

  if let Ok(s) = env::var(env_logger::DEFAULT_WRITE_STYLE_ENV) {
    builder.parse_write_style(&s);
  }

  builder.init()
}
