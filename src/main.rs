mod cli;
mod errors;
mod config;
mod container;
mod ipc;
mod child;

use std::process::exit;
use errors::exit_with_errcode;

fn main() {
  match cli::parse_args() {
      Ok(args) => {
          log::info!("{:?}", args);
          exit_with_errcode(container::start(args));
      }
      Err(e) => {
          log::error!("Error while parsing arguments:\n\t{}", e);
          exit(e.get_retcode());
      }
  }
}
