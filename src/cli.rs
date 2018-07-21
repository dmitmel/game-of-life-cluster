use clap;

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

pub struct Options {
  pub verbosity: u64,
  pub command: Command,
}

pub enum Command {
  Master { port: u16 },
  Slave { hostname: String, port: u16 },
  Gpu,
}

pub fn get_options() -> clap::Result<Options> {
  let matches = create_parser().get_matches_safe()?;

  let verbosity = matches.occurrences_of(VERBOSE_OPT);
  let command = match matches.subcommand() {
    (MASTER_COMMAND, Some(master_matches)) => {
      let port_str = master_matches.value_of(PORT_ARG).unwrap();
      let port = parse_port(port_str)?;

      Command::Master { port }
    }

    (SLAVE_COMMAND, Some(slave_matches)) => {
      let hostname = slave_matches.value_of(HOSTNAME_ARG).unwrap();
      let port_str = slave_matches.value_of(PORT_ARG).unwrap();
      let port = parse_port(port_str)?;

      Command::Slave {
        hostname: hostname.to_owned(),
        port,
      }
    }

    (GPU_COMMAND, Some(_gpu_matches)) => Command::Gpu,

    _ => unreachable!(),
  };

  Ok(Options { verbosity, command })
}

fn create_parser<'a, 'b>() -> clap::App<'a, 'b> {
  clap::App::new(APP_NAME)
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
}

fn parse_port(port_str: &str) -> clap::Result<u16> {
  port_str.parse::<u16>().map_err(|_| {
    clap::Error::value_validation_auto(format!(
      "'{}' isn't a valid port",
      port_str
    ))
  })
}
