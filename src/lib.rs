#![no_std]

mod constants;
mod kevent_struct;

pub use constants::*;
pub use kevent_struct::Event;

#[cfg(not(target_os = "netbsd"))]
pub type EventListSize = libc::c_int;

#[cfg(target_os = "netbsd")]
pub type EventListSize = libc::size_t;

#[allow(improper_ctypes)]
unsafe extern "C" {
  pub unsafe fn kqueue() -> libc::c_int;

  pub unsafe fn kevent(
    kq: libc::c_int,
    changelist: *const Event,
    nchanges: EventListSize,
    eventlist: *mut Event,
    nevents: EventListSize,
    timeout: *const libc::timespec,
  ) -> libc::c_int;

  #[cfg(target_os = "netbsd")]
  pub unsafe fn kqueue1(flags: libc::c_int) -> libc::c_int;
}

#[cfg(test)]
mod test {
  use super::kqueue;

  #[test]
  fn test_kqueue() {
    unsafe {
      assert!(kqueue() > 0);
    }
  }
}
