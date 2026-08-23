use rand::Rng;
use rand::seq::SliceRandom;
use nix::unistd::sethostname;

use crate::errors::Errcode;

const HOSTNAME_NAMES: [&'static str; 8] = [
    "cat", "world", "coffee", "girl",
    "man", "book", "pinguin", "moon"];

const HOSTNAME_ADJ: [&'static str; 16] = [
    "blue", "red", "green", "yellow",
    "big", "small", "tall", "thin",
    "round", "square", "triangular", "weird",
    "noisy", "silent", "soft", "irregular"];

pub fn generate_hostname() -> Result<String, Errcode> {
  let mut rng: rand::prelude::ThreadRng = rand::thread_rng(); //Creates a random number generator.
  let num = rng.r#gen::<u8>(); //Creates a random number between 0 and 255.
  let name = HOSTNAME_NAMES.choose(&mut rng).ok_or(Errcode::RngError)?; //Picks one random string from H_N.
  let adj = HOSTNAME_ADJ.choose(&mut rng).ok_or(Errcode::RngError)?; //Picks one random string from H_A.
  Ok(format!("{}-{}-{}", adj, name, num))
}

pub fn set_container_hostname(hostname: &String) -> Result<(), Errcode> {
  match sethostname(hostname) {
    Ok(_) => {
        log::debug!("Container hostname is now {}", hostname);
        Ok(())
    },
    Err(_) => {
        log::error!("Cannot set hostname {} for container", hostname);
        Err(Errcode::HostnameError(0))
    }
  }
}