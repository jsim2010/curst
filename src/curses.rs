//! Defines curses API compatible with pdcurses and ncurses.
#[cfg(windows)]
mod windows;
#[cfg(unix)]
use ncurses::*;
#[cfg(windows)]
pub(crate) use windows::*;
