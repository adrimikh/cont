use nix::unistd::Pid;
use nix::sched::clone;
use nix::sys::signal::Signal;
use nix::sched::CloneFlags;

use crate::hostname::set_container_hostname;
use crate::mounts::setmountpoint;
use crate::config::ContainerOpts;
use crate::errors::Errcode;

const STACK_SIZE: usize = 1024 * 1024;

fn setup_container_configurations(config: &ContainerOpts) -> Result<(), Errcode> {
  set_container_hostname(&config.hostname)?;
  setmountpoint(&config.mount_dir)?;
  Ok(())
}

fn child(config: ContainerOpts) -> isize {
  match setup_container_configurations(&config) {
    Ok(_) => log::info!("Container set up successfully"),
    Err(e) => {
        log::error!("Error while configuring container: {:?}", e);
        return -1;
    }
  }
  log::info!("Starting container with command {} and args {:?}", config.path.to_str().unwrap(), config.argv);
  0
}

pub fn generate_child_process(config: ContainerOpts) -> Result<Pid, Errcode> {
  let mut tmp_stack: [u8; STACK_SIZE] = [0; STACK_SIZE]; //A buffer that holds the child's stack. (1 KiB)
  let mut flags = CloneFlags::empty(); //We set the flags we want to activate.
  flags.insert(CloneFlags::CLONE_NEWNS); //Makes the child see a different set of mounted filesystems.
  flags.insert(CloneFlags::CLONE_NEWCGROUP); //New CGroup namespace.
  flags.insert(CloneFlags::CLONE_NEWPID); //The child gets its own process numbering.
  flags.insert(CloneFlags::CLONE_NEWIPC); //New IPC namespace.
  flags.insert(CloneFlags::CLONE_NEWNET); //Separates network interfaces.
  flags.insert(CloneFlags::CLONE_NEWUTS); //New UTS namespace.


  //We call the clone syscall.
  let result = unsafe {
    clone(
      //Box is a smart pointer that allocated data on the heap.
      Box::new(|| child(config.clone())), //Runs the child with a cloned config.
      &mut tmp_stack, //Memory region reserved for the child's stack.
      flags,
      Some(Signal::SIGCHLD as i32), //Signals to the kernel.
    )
  };

  match result {
    Ok(pid) => Ok(pid),
    Err(_) => Err(Errcode::ChildProcessError(0)),
  }
}