use crate::errors::Errcode;
use std::os::fd::{IntoRawFd, RawFd};
use nix::sys::socket::{AddressFamily, MsgFlags, SockFlag, SockType, recv, send, socketpair};

pub fn generate_socketpair() -> Result<(RawFd, RawFd), Errcode> {
    socketpair(
        AddressFamily::Unix,
        SockType::SeqPacket,
        None,
        SockFlag::empty(),
    )
    .map(|(first, second)| (first.into_raw_fd(), second.into_raw_fd()))
    .map_err(|_| Errcode::SocketError(0))
}

pub fn send_boolean(fd: RawFd, boolean: bool) -> Result<(), Errcode> {
  let data: [u8; 1] = [boolean.into()]; //Converts bool to an array of size 1 and treats the bool as an integer.
  if let Err(e) = send(fd, &data, MsgFlags::empty()) {
    log::error!("Cannot send boolean through socket: {:?}", e);
    return Err(Errcode::SocketError(1));
  };

  Ok(())
}

pub fn recv_boolean(fd: RawFd, boolean: bool) -> Result<bool, Errcode> {
  let mut data: [u8; 1] = [0];
  if let Err(e) = recv(fd, &mut data, MsgFlags::empty()) {
    log::error!("Cannot receive boolean from socket: {:?}", e);
    return Err(Errcode::SocketError(2));
  };

  Ok(data[0] == 1)
}