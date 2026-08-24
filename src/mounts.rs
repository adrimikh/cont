use std::path::Path;
use std::path::PathBuf;
use std::fs::create_dir_all;
use nix::mount::mount;
use nix::mount::MsFlags;

use crate::errors::Errcode;

use rand::Rng;

//Taken from https://rust-lang-nursery.github.io/rust-cookbook/algorithms/randomness.html
pub fn random_string(n: usize) -> String {
  const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                          abcdefghijklmnopqrstuvwxyz\
                          0123456789";
  let mut rng = rand::thread_rng();

  let name: String = (0..n)
      .map(|_| {
          let idx = rng.gen_range(0..CHARSET.len());
          CHARSET[idx] as char
      })
      .collect();

  name
}

//Creates a directory and all parent directories needed for it.
pub fn create_directory(path: &PathBuf) -> Result<(), Errcode> {
  match create_dir_all(path) {
     Err(e) => {
      log::error!("Cannot create directory {}: {}", path.to_str().unwrap(), e);
      Err(Errcode::MountsError(2))
     },
     Ok(_) => Ok(())
  }
}

//Performs a Linux mount operation, attaching a source directory or filesystem to a mount point with some mount flags.
pub fn mount_directory(path: Option<&PathBuf>, mount_point: &PathBuf, flags: Vec<MsFlags>) -> Result<(), Errcode> {
  //Combine all the flags in a single flag using bitwise OR.
  let mut ms_flags = MsFlags::empty();
  for f in flags.iter() {
    ms_flags.insert(*f);
  }
  //We call the mount syscall.
  match mount::<PathBuf, PathBuf, PathBuf, PathBuf>
  (path, mount_point, None, ms_flags, None) {
    Ok(_) => Ok(()),
    Err(e) => {
      if let Some(p) = path {
        log::error!("Cannot mount {} to {}: {}",
          p.to_str().unwrap(), mount_point.to_str().unwrap(), e);
      }else{
        log::error!("Cannot remount {}: {}",
        mount_point.to_str().unwrap(), e);
      }
      Err(Errcode::MountsError(3))
     }
  }
}

//Makes the root mount tree private and non-propagating, so the container's mounts do not leak into or out of the host namespace.
pub fn setmountpoint(mount_dir: &PathBuf) -> Result<(), Errcode> {
  log::debug!("Setting mount points ...");
  mount_directory(None, &PathBuf::from("/"), 
    vec![MsFlags::MS_REC, MsFlags::MS_PRIVATE])?; //MS_REC applies the flag recursively to all mounts under the path, MS_PRIVATE avoids propagation to other mount namespaces.
  
  let new_root = PathBuf::from(format!("/tmp/cont.{}", random_string(12)));
  log::debug!("Mounting temp directory {}", new_root.as_path().to_str().unwrap());
  create_directory(&new_root)?;
  mount_directory(Some(&mount_dir), &new_root, vec![MsFlags::MS_BIND, MsFlags::MS_PRIVATE])?;
  
  log::debug!("Pivoting root...");
  let old_root_tail = format!("oldroot.{}", random_string(6));
  let put_old = new_root.join(PathBuf::from(old_root_tail.clone()));
  create_directory(&put_old);
  if let Err(_) = pivot_root(&new_root, &put_old) {
    return Err(Errcode::MountsError(4));
  }


  Ok(())
}

pub fn clean_mounts(_rootpath: &PathBuf) -> Result<(), Errcode> {
  Ok(())
}