//! Defines pdcurses API missing from pdcurses-sys.
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::expl_impl_clone_on_copy,
    dead_code,
    unreachable_pub
)] // Exceptions to be made for ffi.

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
