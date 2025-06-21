#[cfg(not(target_os = "netbsd"))]
use core::ptr;

use crate::{EventFilter, EventFlag, FilterFlag};

#[cfg(all(not(target_os = "netbsd"), not(target_os = "freebsd")))]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
#[allow(non_camel_case_types)]
/// Corresponds to `kevent` struct
pub struct Event {
  pub ident: libc::uintptr_t,
  pub filter: EventFilter,
  pub flags: EventFlag,
  pub fflags: FilterFlag,
  pub data: i64,
  pub udata: *mut libc::c_void,
}

#[cfg(target_os = "netbsd")]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
/// Corresponds to `kevent` struct
pub struct Event {
  pub ident: libc::uintptr_t,
  pub filter: EventFilter,
  pub flags: EventFlag,
  pub fflags: FilterFlag,
  pub data: i64,
  pub udata: libc::intptr_t,
}

#[cfg(target_os = "freebsd")]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
/// Corresponds to `kevent` struct
pub struct Event {
  pub ident: libc::uintptr_t,
  pub filter: EventFilter,
  pub flags: EventFlag,
  pub fflags: FilterFlag,
  pub data: i64,
  pub udata: *mut libc::c_void,
  pub ext: [i64; 4],
}

impl Event {
  #[cfg(all(not(target_os = "netbsd"), not(target_os = "freebsd")))]
  pub fn new(
    ident: libc::uintptr_t,
    filter: EventFilter,
    flags: EventFlag,
    fflags: FilterFlag,
    data: i64,
  ) -> Event {
    Event {
      ident,
      filter,
      flags,
      fflags,
      data,
      udata: ptr::null_mut(),
    }
  }

  #[cfg(target_os = "netbsd")]
  pub fn new(
    ident: libc::uintptr_t,
    filter: EventFilter,
    flags: EventFlag,
    fflags: FilterFlag,
  ) -> Event {
    Event {
      ident,
      filter,
      flags,
      fflags,
      data: 0,
      udata: 0,
    }
  }

  #[cfg(target_os = "freebsd")]
  pub fn new(
    ident: libc::uintptr_t,
    filter: EventFilter,
    flags: EventFlag,
    fflags: FilterFlag,
  ) -> Event {
    Event {
      ident,
      filter,
      flags,
      fflags,
      data: 0,
      udata: ptr::null_mut(),
      ext: [0; 4],
    }
  }
}
