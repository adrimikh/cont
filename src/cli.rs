use std::path::PathBuf; //PathBuf is the type that represents a filesystem path in a way that can be modified.
use structopt::StructOpt;

use crate::errors::Errcode; 

//In a Rust #[] means "apply an attribute to the thing that follows".
//Struct level attributes:
#[derive(Debug, StructOpt)] // Here we derive Debug so that StructOpt can be printed/debugged.
#[structopt(name = "cont", about = "A very concise container to practice Rust and container theory.")] //Metadata for CLI. (cont --help).
pub struct Args {
  //Activate debug mode by creating a -d flag (cont -d).
  #[structopt(short, long)] //Creates both short and long flags.
  debug: bool,

  //Generates a -c flag for the command to be executed (cont -c run).
  #[structopt(short, long)]
  pub command: String, 

  //Generates a -u flag for the ID to create inside the container (-u 1000).
  #[structopt(short, long)]
  pub uid: u32,

  //Generates a -m flag for the directory to mount as root of the container.
  #[structopt(parse(from_os_str), short = "m", long = "mount")] //parse(from_os_str) will convert the string into a PathBuf.
  pub mount_dir: PathBuf,
}

//A small logger initializer.
pub fn setup_log(level: log::LevelFilter) {
  env_logger::Builder::from_default_env() //Creates a logger configured from environment variables.
  .format_timestamp_secs() //Adds a UNIX timestamp to each log line in seconds.
  .filter(None, level) //Sets the logging threshold to the level passed as an arg.
  .init(); //Initializes the logger as the program's global logger.
}

pub fn parse_args() -> Result<Args, Errcode> {
  let args: Args = Args::from_args(); //Builds the struct from the command line arguments.

  //Check if debug flag is activated.
  if args.debug {
      setup_log(log::LevelFilter::Debug);
  } else {
      setup_log(log::LevelFilter::Info);
  }

  //Check if the mount directory is correct.
  if !args.mount_dir.exists() || !args.mount_dir.is_dir() {
    return Err(Errcode::ArgumentInvalid("mount"));
  }

  Ok(args)
} 	