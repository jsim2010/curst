//! Defines curses API compatible with pdcurses and ncurses.
#[cfg(windows)]
mod windows;
#[cfg(unix)]
pub(crate) use ncurses::*;
#[cfg(windows)]
pub(crate) use windows::*;
