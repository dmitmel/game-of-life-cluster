#![feature(duration_as_u128)]

extern crate clap;

#[macro_use]
extern crate log;
mod logger;

use std::fmt;

mod gpu;
mod master;
mod slave;

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
const APP_AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
const APP_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

const VERBOSE_OPT: &str = "verbose";
const VERBOSE_OPT_SHORT: &str = "v";

const MASTER_COMMAND: &str = "master";
const SLAVE_COMMAND: &str = "slave";
const GPU_COMMAND: &str = "gpu";

const PORT_ARG: &str = "PORT";
const HOSTNAME_ARG: &str = "HOSTNAME";

fn main() {
  let matches = clap::App::new(APP_NAME)
    .setting(clap::AppSettings::SubcommandRequired)
    .setting(clap::AppSettings::GlobalVersion)
    .version(APP_VERSION)
    .author(APP_AUTHOR)
    .about(APP_DESCRIPTION)
    .arg(
      clap::Arg::with_name(VERBOSE_OPT)
        .short(VERBOSE_OPT_SHORT)
        .multiple(true)
        .global(true)
        .help("Sets the level of verbosity"),
    )
    .subcommand(
      clap::SubCommand::with_name(MASTER_COMMAND)
        .arg(clap::Arg::with_name(PORT_ARG).required(true)),
    )
    .subcommand(
      clap::SubCommand::with_name(SLAVE_COMMAND)
        .arg(clap::Arg::with_name(HOSTNAME_ARG).required(true))
        .arg(clap::Arg::with_name(PORT_ARG).required(true)),
    )
    .subcommand(clap::SubCommand::with_name(GPU_COMMAND))
    .get_matches();

  let verbosity = matches.occurrences_of(VERBOSE_OPT);
  logger::init(verbosity).unwrap_or_else(|e| handle_error(e));

  match matches.subcommand() {
    (MASTER_COMMAND, Some(master_matches)) => {
      let port_str = master_matches.value_of(PORT_ARG).unwrap();
      let port = parse_port(port_str);

      master::listen(port).unwrap_or_else(|e| handle_error(e))
    }

    (SLAVE_COMMAND, Some(slave_matches)) => {
      let hostname = slave_matches.value_of(HOSTNAME_ARG).unwrap();
      let port_str = slave_matches.value_of(PORT_ARG).unwrap();
      let port = parse_port(port_str);

      slave::connect(hostname, port).unwrap_or_else(|e| handle_error(e))
    }

    (GPU_COMMAND, Some(_gpu_matches)) => {
      gpu::run().unwrap_or_else(|e| handle_error(e))
    }

    _ => unreachable!(),
  }
}

fn parse_port(port_str: &str) -> u16 {
  port_str.parse::<u16>().unwrap_or_else(|_| {
    clap::Error::value_validation_auto(format!(
      "The argument '{}' isn't a valid port",
      port_str
    )).exit()
  })
}

fn handle_error<E: fmt::Display>(error: E) -> ! {
  let description = error.to_string();
  let error = clap::Error::with_description(&description, clap::ErrorKind::Io);
  error.exit()
}
