//! Copied and modified from the [crossterm](https://github.com/crossterm-rs/crossterm).
//! Making it a little more convenient and safe to query whether
//! something is a terminal teletype or not.
//! This module defines the IsTty trait and the is_tty method to
//! return true if the item represents a terminal.

#[cfg(unix)]
use std::os::unix::io::AsRawFd;
#[cfg(windows)]
use std::os::windows::io::AsRawHandle;

#[cfg(windows)]
use winapi::um::consoleapi::GetConsoleMode;

/// Adds the `is_tty` method to types that might represent a terminal
///
/// ```rust
/// use std::io::stdout;
/// use erg_common::tty::IsTty;
///
/// let is_tty: bool = stdout().is_tty();
/// ```
pub trait IsTty {
    /// Returns true when an instance is a terminal teletype, otherwise false.
    fn is_tty(&self) -> bool;
}

/// On UNIX, the `isatty()` function returns true if a file
/// descriptor is a terminal.
#[cfg(unix)]
impl<S: AsRawFd> IsTty for S {
    fn is_tty(&self) -> bool {
        let fd = self.as_raw_fd();
        unsafe { libc::isatty(fd) == 1 }
    }
}

#[cfg(target_os = "hermit")]
impl<S: AsRawFd> IsTty for S {
    fn is_tty(&self) -> bool {
        let fd = self.as_raw_fd();
        hermit_abi::isatty(fd)
    }
}

/// returns true if this is a tty
#[cfg(any(target_arch = "wasm32", target_env = "sgx"))]
impl<S: AsRawFd> IsTty for S {
    fn is_tty(&self) -> bool {
        false
    }
}

/// On windows, `GetConsoleMode` will return true if we are in a terminal.
/// Otherwise false.
#[cfg(windows)]
impl<S: AsRawHandle> IsTty for S {
    fn is_tty(&self) -> bool {
        let mut mode = 0;
        let ok = unsafe { GetConsoleMode(self.as_raw_handle() as *mut _, &mut mode) };
        ok == 1
    }
}
