//! Defines a safe curses API.
//!
//! Includes some FFI bindings missing from [`pdcurses`].
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::expl_impl_clone_on_copy,
    dead_code,
    unreachable_pub,
    unsafe_code,
)] // Exceptions to be made for ffi.

pub(crate) use panel::PANEL;

use {
    pdcurses::{self, WINDOW},
    std::os::raw::{c_char, c_int},
};

mod panel {
    use {
        pdcurses::WINDOW,
        std::os::raw::{c_int, c_void},
    };

    #[repr(C)]
    #[derive(Copy)]
    pub struct Struct_panelobs {
        pub above: *mut Struct_panelobs,
        pub pan: *mut Struct_panel,
    }

    impl Clone for Struct_panelobs {
        fn clone(&self) -> Self {
            *self
        }
    }

    impl Default for Struct_panelobs {
        fn default() -> Self {
            unsafe { std::mem::zeroed() }
        }
    }

    #[repr(C)]
    #[derive(Copy)]
    pub struct Struct_panel {
        pub win: *mut WINDOW,
        pub wstarty: c_int,
        pub wendy: c_int,
        pub wstartx: c_int,
        pub wendx: c_int,
        pub below: *mut Struct_panel,
        pub above: *mut Struct_panel,
        pub user: *const c_void,
        pub obscure: *mut Struct_panelobs,
    }

    impl Clone for Struct_panel {
        fn clone(&self) -> Self {
            *self
        }
    }

    impl Default for Struct_panel {
        fn default() -> Self {
            unsafe { std::mem::zeroed() }
        }
    }

    pub type PANEL = Struct_panel;

    extern "C" {
        pub fn bottom_panel(arg1: *mut PANEL) -> c_int;
        pub fn del_panel(arg1: *mut PANEL) -> c_int;
        pub fn hide_panel(arg1: *mut PANEL) -> c_int;
        pub fn move_panel(arg1: *mut PANEL, arg2: c_int, arg3: c_int) -> c_int;
        pub fn new_panel(arg1: *mut WINDOW) -> *mut PANEL;
        pub fn panel_above(arg1: *const PANEL) -> *mut PANEL;
        pub fn panel_below(arg1: *const PANEL) -> *mut PANEL;
        pub fn panel_hidden(arg1: *const PANEL) -> c_int;
        pub fn panel_userptr(arg1: *const PANEL) -> *const c_void;
        pub fn panel_window(arg1: *const PANEL) -> *mut WINDOW;
        pub fn replace_panel(arg1: *mut PANEL, arg2: *mut WINDOW) -> c_int;
        pub fn set_panel_userptr(arg1: *mut PANEL, arg2: *const c_void) -> c_int;
        pub fn show_panel(arg1: *mut PANEL) -> c_int;
        pub fn top_panel(arg1: *mut PANEL) -> c_int;
        pub fn update_panels();
    }
}

#[inline]
pub(crate) fn beep() -> c_int {
    unsafe { pdcurses::beep() }
}

#[inline]
pub(crate) fn curses_version() -> *const c_char {
    unsafe { pdcurses::curses_version() }
}

#[inline]
pub(crate) fn delwin(arg1: *mut WINDOW) -> c_int {
    unsafe { pdcurses::delwin(arg1) }
}

pub(crate) fn del_panel(arg1: *mut PANEL) -> c_int {
    unsafe { panel::del_panel(arg1) }
}

#[inline]
pub(crate) fn doupdate() -> c_int {
    unsafe { pdcurses::doupdate() }
}

#[inline]
pub(crate) fn echo() -> c_int {
    unsafe { pdcurses::echo() }
}

#[inline]
pub(crate) fn endwin() -> c_int {
    unsafe { pdcurses::endwin() }
}

#[inline]
pub(crate) fn flash() -> c_int {
    unsafe { pdcurses::flash() }
}

#[inline]
pub(crate) fn getmaxx(arg1: *mut WINDOW) -> c_int {
    unsafe { pdcurses::getmaxx(arg1) }
}

#[inline]
pub(crate) fn getmaxy(arg1: *mut WINDOW) -> c_int {
    unsafe { pdcurses::getmaxy(arg1) }
}

#[inline]
pub(crate) fn initscr() -> *mut WINDOW {
    unsafe { pdcurses::initscr() }
}

#[inline]
pub(crate) fn longname() -> *const c_char {
    unsafe { pdcurses::longname() }
}

#[inline]
pub(crate) fn newwin(arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int) -> *mut WINDOW {
    unsafe { pdcurses::newwin(arg1, arg2, arg3, arg4) }
}

#[inline]
pub(crate) fn new_panel(arg1: *mut WINDOW) -> *mut PANEL {
    unsafe { panel::new_panel(arg1) }
}

#[inline]
pub(crate) fn noecho() -> c_int {
    unsafe { pdcurses::noecho() }
}

#[inline]
pub(crate) fn panel_window(arg1: *mut PANEL) -> *mut WINDOW {
    unsafe { panel::panel_window(arg1) }
}

#[inline]
pub(crate) fn resize_term(lines: c_int, columns: c_int) -> c_int {
    unsafe { pdcurses::resize_term(lines, columns) }
}

#[inline]
pub(crate) fn termname() -> *const c_char {
    unsafe { pdcurses::termname() }
}

#[inline]
pub(crate) fn update_panels() {
    unsafe { panel::update_panels() }
}

#[inline]
pub(crate) fn waddstr(arg1: *mut WINDOW, arg2: *const c_char) -> c_int {
    unsafe { pdcurses::waddstr(arg1, arg2) }
}

#[inline]
pub(crate) fn wclrtoeol(arg1: *mut WINDOW) -> c_int {
    unsafe { pdcurses::wclrtoeol(arg1) }
}

#[inline]
pub(crate) fn wdelch(arg1: *mut WINDOW) -> c_int {
    unsafe { pdcurses::wdelch(arg1) }
}

#[inline]
pub(crate) fn wgetch(arg1: *mut WINDOW) -> c_int {
    unsafe { pdcurses::wgetch(arg1) }
}

#[inline]
pub(crate) fn wmove(arg1: *mut WINDOW, arg2: c_int, arg3: c_int) -> c_int {
    unsafe { pdcurses::wmove(arg1, arg2, arg3) }
}

#[inline]
pub(crate) fn wtimeout(arg1: *mut WINDOW, arg2: c_int) {
    unsafe { pdcurses::wtimeout(arg1, arg2) }
}
