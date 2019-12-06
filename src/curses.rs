//! Defines curses API compatible with pdcurses and ncurses.
#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub(crate) use windows::*;
#[cfg(unix)]
use ncurses::*;
