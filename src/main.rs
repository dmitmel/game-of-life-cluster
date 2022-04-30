#![feature(duration_as_u128)]

extern crate failure;
use failure::Error;

extern crate clap;

#[macro_use]
extern crate log;
mod logger;

mod cli;
mod gpu;
mod master;
mod slave;
mod utils;

fn main() {
  let options = cli::get_options().unwrap_or_else(|e| e.exit());

  logger::init(options.verbosity);
  run(options).unwrap_or_else(|error| error!("{}", error))
}

fn run(options: cli::Options) -> Result<(), Error> {
  match options.command {
    cli::Command::Master { port } => master::listen(port)?,
    cli::Command::Slave { hostname, port } => slave::connect(hostname, port)?,
    cli::Command::Gpu => gpu::run()?,
  }

  Ok(())
}
