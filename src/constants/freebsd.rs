use bitflags::bitflags;
use libc::{c_uint, c_ushort};

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i16)]
pub enum EventFilter {
    EVFILT_READ = -1,
    EVFILT_WRITE = -2,
    EVFILT_AIO = -3,
    EVFILT_VNODE = -4,
    EVFILT_PROC = -5,
    EVFILT_SIGNAL = -6,
    EVFILT_TIMER = -7,
    EVFILT_PROCDESC = -8,
    EVFILT_FS = -9,
    EVFILT_LIO = -10,
    EVFILT_USER = -11,
    EVFILT_SENDFILE = -12,
    EVFILT_EMPTY = -13,
    EVFILT_SYSCOUNT = 13,
}

bitflags! {
    pub struct EventFlag: c_ushort {
        const EV_ADD          = 0x0001;
        const EV_DELETE       = 0x0002;
        const EV_ENABLE       = 0x0004;
        const EV_DISABLE      = 0x0008;
        const EV_FORCEONESHOT = 0x0100;
        const EV_ONESHOT      = 0x0010;
        const EV_CLEAR        = 0x0020;
        const EV_RECEIPT      = 0x0040;
        const EV_DISPATCH     = 0x0080;
        const EV_SYSFLAGS     = 0xF000;
        const EV_DROP         = 0x1000;
        const EV_FLAG1        = 0x2000;
        const EV_FLAG2        = 0x4000;
        const EV_EOF          = 0x8000;
        const EV_ERROR        = 0x4000;
    }
}

bitflags! {
    pub struct FilterFlag: c_uint {
        const NOTE_FFNOP                        = 0x00000000;
        const NOTE_FFAND                        = 0x40000000;
        const NOTE_FFOR                         = 0x80000000;
        const NOTE_FFCOPY                       = 0xc0000000;
        const NOTE_FFCTRLMASK                   = 0xc0000000;
        const NOTE_FFLAGSMASK                   = 0x00ffffff;
        const NOTE_TRIGGER                      = 0x01000000;

        const NOTE_LOWAT                        = 0x00000001;
        const NOTE_FILE_POLL                    = 0x00000002;

        const NOTE_DELETE                       = 0x00000001;
        const NOTE_WRITE                        = 0x00000002;
        const NOTE_EXTEND                       = 0x00000004;
        const NOTE_ATTRIB                       = 0x00000008;
        const NOTE_LINK                         = 0x00000010;
        const NOTE_RENAME                       = 0x00000020;
        const NOTE_REVOKE                       = 0x00000040;
        const NOTE_OPEN                         = 0x00000080;
        const NOTE_CLOSE                        = 0x00000100;
        const NOTE_CLOSE_WRITE                  = 0x00000200;
        const NOTE_READ                         = 0x00000400;

        const NOTE_EXIT                         = 0x80000000;
        const NOTE_FORK                         = 0x40000000;
        const NOTE_EXEC                         = 0x20000000;
        const NOTE_PCTRLMASK                    = 0xf0000000;
        const NOTE_PDATAMASK                    = 0x000fffff;

        const NOTE_TRACK                        = 0x00000001;
        const NOTE_TRACKERR                     = 0x00000002;
        const NOTE_CHILD                        = 0x00000004;

        const NOTE_SECONDS                      = 0x00000001;
        const NOTE_MSECONDS                     = 0x00000002;
        const NOTE_USECONDS                     = 0x00000004;
        const NOTE_NSECONDS                     = 0x00000008;
        const NOTE_ABSTIME                      = 0x00000010;
    }
}
