#![feature(duration_as_u128)]

extern crate clap;

#[macro_use]
extern crate log;
mod logger;

use std::fmt;

mod cli;
mod gpu;
mod master;
mod slave;

fn main() {
  match cli::get_options() {
    Ok(options) => {
  logger::init(options.verbosity).unwrap_or_else(|e| handle_error(e));

  match options.command {
    cli::Command::Master { port } => {
      master::listen(port).unwrap_or_else(|e| handle_error(e))
    }

    cli::Command::Slave { hostname, port } => {
      slave::connect(hostname, port).unwrap_or_else(|e| handle_error(e))
    }

    cli::Command::Gpu => gpu::run().unwrap_or_else(|e| handle_error(e)),
  }
}

    Err(error) => error.exit(),
  }
}

fn handle_error<E: fmt::Display>(error: E) -> ! {
  let description = error.to_string();
  let error = clap::Error::with_description(&description, clap::ErrorKind::Io);
  error.exit()
}
