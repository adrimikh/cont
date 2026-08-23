use crate::errors::Errcode;
use crate::ipc::generate_socketpair;

use std::ffi::CString; //CString is a C-compatible string (null-terminated).
use std::path::PathBuf;
use std::os::unix::io::RawFd;

#[derive(Clone)] //Implements the Clone trait for the struct -> it can cloned. Clone is useful because it also copies the heap data and not only the stack data.
//path and argv are needed to perform an execve syscall.
pub struct ContainerOpts {
  pub path: CString, //The path of the exec. to run in the container.
  pub argv: Vec<CString>,
  pub uid: u32, //The ID of the user inside the container.
  pub fd: RawFd, //The file descriptor
  pub mount_dir: PathBuf
}

//Constructor
//Note: |s| is the notation for an anonymous function in Rust.
impl ContainerOpts {
  pub fn new(command: String, uid: u32, mount_dir: PathBuf) -> Result<(ContainerOpts, (RawFd, RawFd)), Errcode> {
    let argv: Vec<CString> = command.split_ascii_whitespace() // Builds an argument vector from the command string.
      .map(|s| CString::new(s).expect("Cannot read arg")).collect(); //Converts each split token into a CString, expect is for error handling: if Err it panics with that message.
    let path = argv[0].clone(); //Takes the executable path and clones it into 'path'.
    let sockets = generate_socketpair()?;

    Ok((ContainerOpts {
            path,
            argv,
            uid,
            fd: sockets.1,
            mount_dir,
        }, 
        sockets)
    )

}
}