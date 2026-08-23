use std::os::fd::RawFd;

use crate::{cli::Args, config::ContainerOpts, errors::Errcode::{self, ArgumentInvalid}};
use nix::{unistd::close, sys::utsname::uname};

pub struct Container {
  sockets: (RawFd, RawFd),
  config: ContainerOpts,
}

impl Container {
  //Constructor, creates ContainerOpts from args.
  pub fn new(args: Args) -> Result<Container, Errcode>{
    let (config, sockets) = ContainerOpts::new(
      args.command, 
      args.uid, 
      args.mount_dir)?; 
    
    Ok(Container { sockets, config })
  }

  //Handles the creation process.
  pub fn create(&mut self) -> Result<(), Errcode> {
    log::debug!("Creation finished");
    Ok(())
  }

  //Handles the exit.
  pub fn clean_exit(&mut self) -> Result<(), Errcode> {
    log::debug!("Cleaning container");

    if let Err(e) = close(self.sockets.0) {
      log::error!("Unable to close write socket: {:?}", e);
      return Err(Errcode::SocketError(3));
    }
    if let Err(e) = close(self.sockets.1) {
      log::error!("Unable to close read socket: {:?}", e);
      return Err(Errcode::SocketError(4));      
    }
    
    Ok(())
  }
}

pub fn start(args: Args) -> Result<(), Errcode> {
  check_linux_version()?; //Followed by ? so that the error is immediately returned, else the function's result would be ignored.

  let mut container = Container::new(args)?;
  if let Err(e) = container.create() {
    container.clean_exit()?;
    log::error!("Error while creating container: {:?}", e);
    return Err(e);
  }
  log::debug!("Finished, cleaning & exit");
  container.clean_exit()
}

pub const MIN_KERNEL_VERSION: f32 = 4.8;

pub fn check_linux_version() -> Result<(), Errcode> {
  //We retrieve the system's info.
  let host = uname().map_err(|_| ArgumentInvalid("uname failed."))?;
  //The 'ok_or' function maps Option<T> to Result<T,E>.
  let release = host.release().to_str().ok_or(ArgumentInvalid("invalid kernel version"))?;

  log::debug!("Linux release: {}", release);

  //We look for the version part in the release string.
  if let Ok(version) = scan_fmt::scan_fmt!(release, "{f}.{}", f32) {
      if version < MIN_KERNEL_VERSION {
          return Err(Errcode::NotSupported(0));
      }
  } else {
      return Err(Errcode::ContainerError(0));
  }

  if host.machine() != "x86_64" {
      return Err(Errcode::NotSupported(1));
  }
  Ok(())
}