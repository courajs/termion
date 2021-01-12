use std::{io, mem};
use std::fs::File;
use std::os::unix::io::RawFd;
use std::os::unix::io::AsRawFd;

use super::cvt;
use super::libc::{c_ushort, ioctl, TIOCGWINSZ};
use crate::get_tty;

thread_local! {
    static TTY: io::Result<File> = get_tty();
}

fn tty_fd() -> io::Result<RawFd> {
    TTY.with(|f| match f.as_ref() {
        Ok(f) => Ok(f.as_raw_fd()),
        Err(_) => Err(io::Error::from_raw_os_error(25)),
    })
}

#[repr(C)]
struct TermSize {
    row: c_ushort,
    col: c_ushort,
    x: c_ushort,
    y: c_ushort,
}
/// Get the size of the terminal.
pub fn terminal_size() -> io::Result<(u16, u16)> {
    unsafe {
        let mut size: TermSize = mem::zeroed();
        cvt(ioctl(tty_fd()?, TIOCGWINSZ.into(), &mut size as *mut _))?;
        Ok((size.col as u16, size.row as u16))
    }
}

/// Get the size of the terminal, in pixels
pub fn terminal_size_pixels() -> io::Result<(u16, u16)> {
    unsafe {
        let mut size: TermSize = mem::zeroed();
        cvt(ioctl(tty_fd()?, TIOCGWINSZ.into(), &mut size as *mut _))?;
        Ok((size.x as u16, size.y as u16))
    }
}
