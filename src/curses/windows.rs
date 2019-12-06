#![allow(
    clippy::expl_impl_clone_on_copy,
    clippy::missing_docs_in_private_items,
    unreachable_pub,
    unsafe_code,
)] // Allowable for ffi bindings.

use {
    pdcurses,
    std::{ffi::{CStr, CString}, os::raw::c_char},
};

// Bindings missing from pdcurses.
mod ll {
    use std::os::raw::{c_int, c_void};

    pub(crate) type WINDOW = *mut pdcurses::WINDOW;
    pub(crate) type PANEL = *mut Struct_panel;
    pub(crate) type PANELOBS = *mut Struct_panelobs;

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

    #[repr(C)]
    #[derive(Copy)]
    pub struct Struct_panel {
        pub win: WINDOW,
        pub wstarty: c_int,
        pub wendy: c_int,
        pub wstartx: c_int,
        pub wendx: c_int,
        pub below: PANEL,
        pub above: PANEL,
        pub user: *const c_void,
        pub obscure: PANELOBS,
    }

    impl Clone for Struct_panel {
        fn clone(&self) -> Self {
            *self
        }
    }

    extern "C" {
        pub fn del_panel(_: PANEL) -> c_int;
        pub fn new_panel(_: WINDOW) -> PANEL;
        pub fn panel_window(_: PANEL) -> WINDOW;
        pub fn update_panels();
    }
}

pub(crate) type PANEL = ll::PANEL;
pub(crate) type WINDOW = ll::WINDOW;

unsafe fn from_c_str(s: *const c_char) -> String {
    let bytes = CStr::from_ptr(s).to_bytes();
    String::from_utf8_unchecked(bytes.to_vec())
}

trait ToCStr {
    fn to_c_str(&self) -> CString;
}

impl ToCStr for &str {
    fn to_c_str(&self) -> CString {
        CString::new(*self).expect("creating `CString`")
    }
}

pub(crate) fn beep() -> i32 {
    unsafe { pdcurses::beep() }
}

pub(crate) fn delwin(w: WINDOW) -> i32 {
    unsafe { pdcurses::delwin(w) }
}

pub(crate) fn del_panel(panel: PANEL) -> i32 {
    unsafe { ll::del_panel(panel) }
}

pub(crate) fn doupdate() -> i32 {
    unsafe { pdcurses::doupdate() }
}

pub(crate) fn echo() -> i32 {
    unsafe { pdcurses::echo() }
}

pub(crate) fn endwin() -> i32 {
    unsafe { pdcurses::endwin() }
}

pub(crate) fn flash() -> i32 {
    unsafe { pdcurses::flash() }
}

pub(crate) fn getmaxx(w: WINDOW) -> i32 {
    unsafe { pdcurses::getmaxx(w) }
}

pub(crate) fn getmaxy(w: WINDOW) -> i32 {
    unsafe { pdcurses::getmaxy(w) }
}

pub(crate) fn initscr() -> WINDOW {
    unsafe { pdcurses::initscr() }
}

pub(crate) fn longname() -> String {
    unsafe { from_c_str(pdcurses::longname()) }
}

pub(crate) fn newwin(lines: i32, cols: i32, y: i32, x: i32) -> WINDOW {
    unsafe { pdcurses::newwin(lines, cols, y, x) }
}

pub(crate) fn new_panel(window: WINDOW) -> PANEL {
    unsafe { ll::new_panel(window) }
}

pub(crate) fn noecho() -> i32 {
    unsafe { pdcurses::noecho() }
}

pub(crate) fn panel_window(panel: PANEL) -> WINDOW {
    unsafe { ll::panel_window(panel) }
}

pub(crate) fn resize_term(lines: i32, cols: i32) -> i32 {
    unsafe { pdcurses::resize_term(lines, cols) }
}

pub(crate) fn termname() -> String {
    unsafe { from_c_str(pdcurses::termname()) }
}

pub(crate) fn update_panels() {
    unsafe { ll::update_panels() }
}

pub(crate) fn waddstr(w: WINDOW, s: &str) -> i32 {
    unsafe { pdcurses::waddstr(w, s.to_c_str().as_ptr()) }
}

pub(crate) fn wclrtoeol(w: WINDOW) -> i32 {
    unsafe { pdcurses::wclrtoeol(w) }
}

pub(crate) fn wdelch(w: WINDOW) -> i32 {
    unsafe { pdcurses::wdelch(w) }
}

pub(crate) fn wgetch(w: WINDOW) -> i32 {
    unsafe { pdcurses::wgetch(w) }
}

pub(crate) fn wmove(w: WINDOW, y: i32, x: i32) -> i32 {
    unsafe { pdcurses::wmove(w, y, x) }
}

pub(crate) fn wtimeout(w: WINDOW, delay: i32) {
    unsafe { pdcurses::wtimeout(w, delay) }
}
