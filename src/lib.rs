//! Rustifies curses.
#![warn(
    absolute_paths_not_starting_with_crate,
    anonymous_parameters,
    bare_trait_objects,
    box_pointers,
    deprecated_in_future,
    elided_lifetimes_in_paths,
    ellipsis_inclusive_range_patterns,
    explicit_outlives_requirements,
    keyword_idents,
    macro_use_extern_crate,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    missing_doc_code_examples,
    private_doc_tests,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unstable_features,
    unused_extern_crates,
    unused_import_braces,
    unused_labels,
    unused_lifetimes,
    unused_qualifications,
    unused_results,
    unsafe_code,
    variant_size_differences,
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic,
    clippy::restriction
)]
#![allow(
    clippy::fallible_impl_from, // Not always valid; issues should be detected by tests or other lints.
    clippy::implicit_return, // Goes against rust convention and requires return calls in places it is not helpful (e.g. closures).
    clippy::suspicious_arithmetic_impl, // Not always valid; issues should be detected by tests or other lints.
    clippy::suspicious_op_assign_impl, // Not always valid; issues should be detected by tests or other lints.
    variant_size_differences, // Generally okay.
)]
// Temporary allows.
#![allow(
    clippy::missing_inline_in_public_items, // Flags methods in derived traits.
)]

mod curses;

use {
    core::{convert::TryFrom, num::NonZeroU32},
    curses::PANEL,
    pdcurses::{self, WINDOW},
    std::{
        ffi::{CStr, CString},
        os::raw::{c_char, c_int},
        str::Utf8Error,
    },
};

/// Represents a return value that can either be ok or an error.
type OkOrErr = Result<(), ()>;

/// The return value that indicates no errors occurred.
const OK: c_int = 0;
/// The return value that indicates an error occurred.
const ERR: c_int = -1;

/// The [`char`] that represents the `Backspace` key.
const CHAR_BACKSPACE: char = '\u{8}';
/// The [`char`] that represents the `Enter` key.
const CHAR_ENTER: char = '\n';
/// The [`char`] that represents the `Esc` key.
const CHAR_ESC: char = '\u{1b}';
/// The [`char`] that represents the `Tab` key.
const CHAR_TAB: char = '\t';

/// Converts `value` to a [`NonZeroU32`].
fn non_zero_u32(value: c_int) -> Result<NonZeroU32, ()> {
    u32::try_from(value)
        .map_err(|_| ())
        .and_then(|value| NonZeroU32::new(value).ok_or(()))
}

/// Converts `value` to a [`c_int`].
fn int(value: u32) -> c_int {
    c_int::try_from(value).unwrap_or(c_int::max_value())
}

/// Converts `ptr` to a [`&str`].
fn string(ptr: *const c_char) -> Result<&'static str, Utf8Error> {
    #[allow(unsafe_code)] // Required to create CStr.
    unsafe { CStr::from_ptr(ptr) }.to_str()
}

/// Returns a string describing the `PDCurses` version.
pub fn version() -> Result<&'static str, Utf8Error> {
    string(curses::curses_version())
}

/// Converts `value` into an [`OkOrErr`].
fn result(value: c_int) -> OkOrErr {
    if value == OK {
        Ok(())
    } else {
        Err(())
    }
}

/// Signifies a key on a keyboard.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Key {
    /// A key that is printable.
    Printable(char),
    /// The `Backspace` key.
    Backspace,
    /// The `Esc` key.
    Esc,
    /// The `Tab` key.
    Tab,
    /// The `Enter` key.
    Enter,
    /// An unknown key.
    Unknown(i32),
}

impl Key {
    /// Returns the [`Key`] from the user.
    fn get(window: Window) -> Option<Self> {
        match curses::wgetch(window.0) {
            ERR => None,
            value => Some(value.into()),
        }
    }
}

impl From<c_int> for Key {
    fn from(value: c_int) -> Self {
        match u32::try_from(value) {
            Ok(u32_value) => match std::char::from_u32(u32_value) {
                None => Self::Unknown(value),
                Some(c) => match c {
                    CHAR_BACKSPACE => Self::Backspace,
                    CHAR_ESC => Self::Esc,
                    CHAR_TAB => Self::Tab,
                    CHAR_ENTER => Self::Enter,
                    _ => Self::Printable(c),
                },
            },
            Err(_) => Self::Unknown(value),
        }
    }
}

/// Signifies an input from the user.
#[derive(Clone, Copy, Debug)]
pub struct Input {
    /// The [`Key`] from the user.
    key: Key,
}

impl Input {
    /// Returns an [`Input`] from the terminal.
    fn get(window: Window) -> Option<Self> {
        Key::get(window).map(|key| Self { key })
    }

    /// Returns the [`Key`] of `self`.
    pub const fn key(&self) -> &Key {
        &self.key
    }
}

/// Signifies the location of a cell in the terminal.
#[derive(Clone, Copy, Debug)]
pub struct Location {
    /// The index of the line.
    line: u32,
    /// The index of the column.
    column: u32,
}

impl Location {
    /// Creates a new location.
    pub const fn new(line: u32, column: u32) -> Self {
        Self { line, column }
    }

    /// Returns the index of the column that contains `self`.
    fn column(self) -> c_int {
        int(self.column)
    }

    /// Returns the index of the line that contains `self`.
    fn line(self) -> c_int {
        int(self.line)
    }
}

/// Signifies the size of the terminal.
#[derive(Clone, Copy, Debug)]
pub struct Size {
    /// The number of lines.
    lines: NonZeroU32,
    /// The number of columns.
    columns: NonZeroU32,
}

impl Size {
    /// Returns the number of columns.
    fn columns(self) -> c_int {
        int(self.columns.get())
    }

    /// Returns the number of lines.
    fn lines(self) -> c_int {
        int(self.lines.get())
    }
}

/// Signifies a curses panel.
#[derive(Debug)]
pub struct Panel(*mut PANEL);

impl Panel {
    /// Creates a new curses panel.
    pub fn new(window: Window) -> Result<Self, ()> {
        let panel = curses::new_panel(window.0);

        if panel.is_null() {
            Err(())
        } else {
            Ok(Self(panel))
        }
    }

    /// Returns the [`Window`] associated with `self`.
    pub fn window(&self) -> Result<Window, ()> {
        let win = curses::panel_window(self.0);

        if win.is_null() {
            Err(())
        } else {
            Ok(Window(win))
        }
    }
}

impl Drop for Panel {
    fn drop(&mut self) {
        if result(curses::del_panel(self.0)).is_err() {
            panic!("cannot free memory associated with panel");
        }
    }
}

/// Represents a curses window.
#[derive(Clone, Copy, Debug)]
pub struct Window(*mut WINDOW);

impl Window {
    /// Creates a new curses window.
    ///
    /// For now, the size and location of the window is defined statically.
    pub fn new() -> Result<Self, ()> {
        let window = curses::newwin(10, 30, 0, 0);

        if window.is_null() {
            Err(())
        } else {
            Ok(Self(window))
        }
    }

    /// Writes all the characters of `s` to `self`.
    pub fn add_string(self, s: String) -> OkOrErr {
        // Define local variable to hold lifetime throughout the function.
        let text = CString::new(s).map_err(|_| ())?;

        result(curses::waddstr(self.0, text.as_ptr()))
    }

    /// Clears `self` from the cursor to the end of the line.
    pub fn clear_to_line_end(self) -> OkOrErr {
        result(curses::wclrtoeol(self.0))
    }

    /// Returns the number of columns in `self`.
    pub fn columns(self) -> Result<NonZeroU32, ()> {
        non_zero_u32(curses::getmaxx(self.0))
    }

    /// Frees memory associated with `self`.
    fn delete(self) -> OkOrErr {
        result(curses::delwin(self.0))
    }

    /// Deletes the character under the cursor.
    ///
    /// All characters to right on the same line are moved to the left one position and the
    /// last character is filled with a blank. The cursor position does not change.
    #[inline]
    pub fn delete_char(self) -> OkOrErr {
        result(curses::wdelch(self.0))
    }

    /// Returns an [`Input`] from the terminal.
    ///
    /// [`None`] indicates no [`Input`] was found in the specified time.
    #[inline]
    pub fn get_input(self) -> Option<Input> {
        Input::get(self)
    }

    /// Moves the cursor to `location`.
    #[inline]
    pub fn move_to(self, location: Location) -> OkOrErr {
        result(curses::wmove(self.0, location.line(), location.column()))
    }

    /// Returns the number of rows in `self`.
    #[inline]
    pub fn rows(self) -> Result<NonZeroU32, ()> {
        non_zero_u32(curses::getmaxy(self.0))
    }

    /// Sets how curses will block when attempting to get an [`Input`].
    ///
    /// If `timeout` is [`None`], curses will not block.
    #[inline]
    pub fn set_block_timeout(self, timeout: Option<u32>) {
        let value = match timeout {
            None => -1,
            Some(ms) => c_int::try_from(ms).unwrap_or(c_int::max_value()),
        };

        curses::wtimeout(self.0, value);
    }
}

/// Represents the interface to the curses library.
#[derive(Debug)]
pub struct Curses {
    /// The default window that covers the entire physical screen.
    main_window: Window,
}

impl Curses {
    /// Sounds the audible bell on the terminal, if possible, otherwise executes `flash`.
    #[inline]
    pub fn beep(&self) {
        #[allow(unused_results)] // beep always returns OK when called after initscr.
        {
            curses::beep();
        }
    }

    /// Returns a verbose description of the current terminal.
    #[inline]
    pub fn description(&self) -> Result<&str, Utf8Error> {
        string(curses::longname())
    }

    /// Flashes the terminal screen.
    ///
    /// The action of flashing is specified as inverting the foreground and background of every
    /// cell, pausing, and then restoring.
    #[inline]
    pub fn flash(&self) {
        #[allow(unused_results)] // flash always returns OK when called after initscr.
        {
            curses::flash();
        }
    }

    /// Returns the default window for the terminal.
    pub const fn main_window(&self) -> &Window {
        &self.main_window
    }

    /// Returns a short description (14 characters) of the current terminal.
    #[inline]
    pub fn name(&self) -> Result<&str, Utf8Error> {
        string(curses::termname())
    }

    /// Refreshes the physical screen to match the virtual screen.
    pub fn refresh(&self) -> OkOrErr {
        curses::update_panels();
        result(curses::doupdate())
    }

    /// Resizes the physical screen to `size`.
    ///
    /// Only resizes screen to a non zero value. If attempting to synchronize curses to a new screen size, use [`sync_screen_size`].
    #[inline]
    pub fn resize_screen(&self, size: Size) -> OkOrErr {
        result(curses::resize_term(size.lines(), size.columns()))
    }

    /// Sets if typed characters are echoed.
    #[inline]
    pub fn set_echo(&self, is_enabled: bool) -> OkOrErr {
        result(if is_enabled {
            curses::echo()
        } else {
            curses::noecho()
        })
    }

    /// Synchronizes curses to match the current screen size.
    #[inline]
    pub fn sync_screen_size(&self) -> OkOrErr {
        result(curses::resize_term(0, 0))
    }
}

impl Default for Curses {
    /// Initializes curses.
    ///
    /// Ensures that initscr will be the first curses routine called. In case of error, will
    /// write a message to stderr and end the program.
    #[inline]
    fn default() -> Self {
        Self {
            main_window: Window(curses::initscr()),
        }
    }
}

impl Drop for Curses {
    #[inline]
    fn drop(&mut self) {
        if self.main_window.delete().is_err() {
            panic!("cannot free memory associated with standard window");
        }

        #[allow(unused_results)]
        {
            curses::endwin();
        }
    }
}
