//! RenderDoc application bindings for Rust

#![deny(missing_docs)]

#[cfg(any(target_os = "macos", target_os = "ios"))]
compile_error!("RenderDoc does not support this platform.");

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate lazy_static;
extern crate shared_library;

#[cfg(target_os = "windows")]
extern crate winapi;
#[cfg(feature = "winit")]
extern crate winit;

pub use self::entry::version::{ApiVersion, V100, V110};

use std::os::raw::{c_ulonglong, c_void};
use std::u32;

#[cfg(windows)]
use winapi::guiddef::GUID;
#[cfg(feature = "winit")]
use winit::VirtualKeyCode;

pub mod api;
pub mod entry;
pub mod prelude;

/// Magic value used for when applications pass a path where shader debug
/// information can be found to match up with a stripped shader.
///
/// Windows GUID representation intended for consumption by D3D.
#[cfg(windows)]
pub const SHADER_MAGIC_DEBUG_VALUE_STRUCT: GUID = GUID {
    Data1: 0xeab25520,
    Data2: 0x6670,
    Data3: 0x4865,
    Data4: [0x84, 0x29, 0x6c, 0x8, 0x51, 0x00, 0xff],
};

/// Magic value used for when applications pass a path where shader debug
/// information can be found to match up with a stripped shader.
///
/// Raw byte array representation (assuming x86 endianness).
pub const SHADER_MAGIC_DEBUG_VALUE_BYTE_ARRAY: &[u8] = &[
    0x20,
    0x55,
    0xb2,
    0xea,
    0x70,
    0x66,
    0x65,
    0x48,
    0x84,
    0x29,
    0x6c,
    0x8,
    0x51,
    0x54,
    0x00,
    0xff,
];

/// Magic value used for when applications pass a path where shader debug
/// information can be found to match up with a stripped shader.
///
/// Truncated version when only a `uint64_t` is available (e.g. Vulkan tags).
pub const SHADER_MAGIC_DEBUG_VALUE_TRUNCATED: c_ulonglong = 0x4856670eab25520;

/// RenderDoc capture options.
#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CaptureOption {
    /// Let the application enable vertical synchronization.
    AllowVSync = 0,
    /// Let the application enter fullscreen mode.
    AllowFullscreen = 1,
    /// Record API debugging events and messages.
    ///
    /// This option also goes by the deprecated name of `DebugDeviceMode`.
    ApiValidation = 2,
    /// Capture CPU callstacks for API events.
    CaptureCallstacks = 3,
    /// When capturing CPU callstacks, only capture them from drawcalls.
    ///
    /// This option does nothing without the above option being enabled.
    CaptureCallstacksOnlyDraws = 4,
    /// Specify a delay, measured in seconds, to wait for a debugger to attach
    /// to the application after being injected.
    DelayForDebugger = 5,
    /// Verify any writes to mapped buffers by checking the memory after the
    /// bounds of the returned pointer to detect any modification.
    VerifyMapWrites = 6,
    /// Hooks any system API calls that create child processes and injects
    /// RenderDoc into them recursively with the same options.
    HookIntoChildren = 7,
    /// Reference all resources available at the time of capture.
    ///
    /// By default, RenderDoc only includes resources in the final capture file
    /// necessary for that frame. This option allows you to override that
    /// behavior.
    RefAllResources = 8,
    /// Save the initial state for all resources, regardless of usage.
    ///
    /// By default, RenderDoc skips saving initial states for resources where
    /// the previous contents don't appear to be used (assuming that writes
    /// before reads indicate the previous contents aren't used).
    SaveAllInitials = 9,
    /// Capture all command lists generated from the start of the application.
    ///
    /// In APIs that allow for recording of command lists to be replayed later,
    /// RenderDoc may choose to not capture command lists before a frame capture
    /// is triggered to reduce overhead. This means any command lists that are
    /// recorded one and replayed many times will not be available, potentially
    /// causing a failure to capture.
    ///
    /// Note that this is only true for APIs where multithreading is difficult
    /// or otherwise discouraged. Newer APIs, e.g. Vulkan and D3D12, will ignore
    /// this option and always capture all command lists since they are heavily
    /// oriented around them and the associated overhead is mostly reduced due
    /// to superior API design.
    CaptureAllCmdLists = 10,
    /// Mute API debug output when `CaptureOption::ApiValidation` is enabled.
    DebugOutputMute = 11,
}

/// Raw mutable pointer to the API's root handle.
///
/// For example, this could be a pointer to an `ID3D11Device`,
/// `HGLRC`/`GLXContext`, `ID3D12Device`, etc.
pub type DevicePointer = *const c_void;

/// User input key codes.
#[allow(missing_docs)]
#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum InputButton {
    /// The '0' key over the letters.
    Key0 = 0x30,
    /// The '1' key over the letters.
    Key1 = 0x31,
    /// The '2' key over the letters.
    Key2 = 0x32,
    /// The '3' key over the letters.
    Key3 = 0x33,
    /// The '4' key over the letters.
    Key4 = 0x34,
    /// The '5' key over the letters.
    Key5 = 0x35,
    /// The '6' key over the letters.
    Key6 = 0x36,
    /// The '7' key over the letters.
    Key7 = 0x37,
    /// The '8' key over the letters.
    Key8 = 0x38,
    /// The '9' key over the letters.
    Key9 = 0x39,

    A = 0x41,
    B = 0x42,
    C = 0x43,
    D = 0x44,
    E = 0x45,
    F = 0x46,
    G = 0x47,
    H = 0x48,
    I = 0x49,
    J = 0x4A,
    K = 0x4B,
    L = 0x4C,
    M = 0x4D,
    N = 0x4E,
    O = 0x4F,
    P = 0x50,
    Q = 0x51,
    R = 0x52,
    S = 0x53,
    T = 0x54,
    U = 0x55,
    V = 0x56,
    W = 0x57,
    X = 0x58,
    Y = 0x59,
    Z = 0x5A,

    /// Leave the rest of the ASCII range free, in case the RenderDoc developers
    /// decide to use it later.
    NonPrintable = 0x100,

    /// Division key on the numpad.
    Divide,
    /// Multiplication key on the numpad.
    Multiply,
    /// Subtraction key on the numpad.
    Subtract,
    /// Addition key on the numpad.
    Plus,

    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,

    Home,
    End,
    Insert,
    Delete,
    PageUp,
    PageDn,

    Backspace,
    Tab,
    PrtScrn,
    Pause,

    Max,
}

#[cfg(feature = "winit")]
impl From<winit::VirtualKeyCode> for InputButton {
    fn from(code: winit::VirtualKeyCode) -> InputButton {
        match code {
            VirtualKeyCode::Key1 => InputButton::Key1,
            VirtualKeyCode::Key2 => InputButton::Key2,
            VirtualKeyCode::Key3 => InputButton::Key3,
            VirtualKeyCode::Key4 => InputButton::Key4,
            VirtualKeyCode::Key5 => InputButton::Key5,
            VirtualKeyCode::Key6 => InputButton::Key6,
            VirtualKeyCode::Key7 => InputButton::Key7,
            VirtualKeyCode::Key8 => InputButton::Key8,
            VirtualKeyCode::Key9 => InputButton::Key9,
            VirtualKeyCode::Key0 => InputButton::Key0,
            VirtualKeyCode::A => InputButton::A,
            VirtualKeyCode::B => InputButton::B,
            VirtualKeyCode::C => InputButton::C,
            VirtualKeyCode::D => InputButton::D,
            VirtualKeyCode::E => InputButton::E,
            VirtualKeyCode::F => InputButton::F,
            VirtualKeyCode::G => InputButton::G,
            VirtualKeyCode::H => InputButton::H,
            VirtualKeyCode::I => InputButton::I,
            VirtualKeyCode::J => InputButton::J,
            VirtualKeyCode::K => InputButton::K,
            VirtualKeyCode::L => InputButton::L,
            VirtualKeyCode::M => InputButton::M,
            VirtualKeyCode::N => InputButton::N,
            VirtualKeyCode::O => InputButton::O,
            VirtualKeyCode::P => InputButton::P,
            VirtualKeyCode::Q => InputButton::Q,
            VirtualKeyCode::R => InputButton::R,
            VirtualKeyCode::S => InputButton::S,
            VirtualKeyCode::T => InputButton::T,
            VirtualKeyCode::U => InputButton::U,
            VirtualKeyCode::V => InputButton::V,
            VirtualKeyCode::W => InputButton::W,
            VirtualKeyCode::X => InputButton::X,
            VirtualKeyCode::Y => InputButton::Y,
            VirtualKeyCode::Z => InputButton::Z,
            VirtualKeyCode::Divide => InputButton::Divide,
            VirtualKeyCode::Multiply => InputButton::Multiply,
            VirtualKeyCode::Subtract => InputButton::Subtract,
            VirtualKeyCode::Add => InputButton::Plus,
            VirtualKeyCode::F1 => InputButton::F1,
            VirtualKeyCode::F2 => InputButton::F2,
            VirtualKeyCode::F3 => InputButton::F3,
            VirtualKeyCode::F4 => InputButton::F4,
            VirtualKeyCode::F5 => InputButton::F5,
            VirtualKeyCode::F6 => InputButton::F6,
            VirtualKeyCode::F7 => InputButton::F7,
            VirtualKeyCode::F8 => InputButton::F8,
            VirtualKeyCode::F9 => InputButton::F9,
            VirtualKeyCode::F10 => InputButton::F10,
            VirtualKeyCode::F11 => InputButton::F11,
            VirtualKeyCode::F12 => InputButton::F12,
            VirtualKeyCode::Home => InputButton::Home,
            VirtualKeyCode::End => InputButton::End,
            VirtualKeyCode::Insert => InputButton::Insert,
            VirtualKeyCode::Delete => InputButton::Delete,
            VirtualKeyCode::PageUp => InputButton::PageUp,
            VirtualKeyCode::PageDown => InputButton::PageDn,
            VirtualKeyCode::Back => InputButton::Backspace,
            VirtualKeyCode::Tab => InputButton::Tab,
            VirtualKeyCode::Snapshot => InputButton::PrtScrn,
            VirtualKeyCode::Pause => InputButton::Pause,
            _ => InputButton::Max,
        }
    }
}

bitflags! {
    /// Bit flags for customizing the RenderDoc overlay.
    pub struct OverlayBits: u32 {
        /// Controls whether the overlay is enabled or disabled globally.
        const ENABLED = 0x1;
        /// Shows the average, minimum, and maximum sampled frame rate.
        const FRAME_RATE = 0x2;
        /// Shows the current frame number.
        const FRAME_NUMBER = 0x4;
        /// Shows a list of recent captures, out of the total captures made.
        const CAPTURE_LIST = 0x8;
        /// Sets the default configuration for the overlay.
        const DEFAULT = (0x1 | 0x2 | 0x4 | 0x8);
        /// Enables all overlay configuration bits.
        const ALL = u32::MAX;
        /// Disables all overlay configuration bits.
        const NONE = u32::MIN;
    }
}

/// Raw mutable pointer to the OS-provided window handle.
pub type WindowHandle = *const c_void;

/// An instance of the RenderDoc API with baseline version `V`.
#[derive(Clone, Debug)]
pub struct RenderDoc<V: ApiVersion>(V::Entry);

impl<V: ApiVersion> RenderDoc<V> {
    /// Initializes a new instance of the RenderDoc API.
    pub fn new() -> Result<RenderDoc<V>, String> {
        let api = V::load()?;
        Ok(RenderDoc(api))
    }

    /// Returns the raw entry point of the API.
    ///
    /// # Safety
    ///
    /// Using the entry point structure directly will discard any thread safety
    /// provided by default with this library.
    pub unsafe fn raw_api(&self) -> V::Entry {
        self.0.clone()
    }
}

impl From<RenderDoc<V110>> for RenderDoc<V100> {
    fn from(newer: RenderDoc<V110>) -> RenderDoc<V100> {
        RenderDoc(newer.0.entry_v100)
    }
}

impl api::RenderDocV100 for RenderDoc<V100> {
    unsafe fn entry_v100(&self) -> &self::entry::EntryV100 {
        &self.0
    }
}

impl api::RenderDocV100 for RenderDoc<V110> {
    unsafe fn entry_v100(&self) -> &self::entry::EntryV100 {
        &self.0.entry_v100
    }
}

impl api::RenderDocV110 for RenderDoc<V110> {
    unsafe fn entry_v110(&self) -> &self::entry::EntryV110 {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::api::*;

    #[test]
    fn get_set_capture_option_f32() {
        let mut rd: RenderDoc<V110> = RenderDoc::new().expect("Failed to init");

        let delay = rd.get_capture_option_f32(CaptureOption::DelayForDebugger);
        assert_eq!(delay, 0.0f32);

        rd.set_capture_option_f32(CaptureOption::DelayForDebugger, 2.5f32);
        let delay = rd.get_capture_option_f32(CaptureOption::DelayForDebugger);
        assert_eq!(delay, 2.0f32);
    }

    #[test]
    fn get_set_capture_option_u32() {
        let rd: RenderDoc<V110> = RenderDoc::new().expect("Failed to init");

        let vsync = rd.get_capture_option_u32(CaptureOption::AllowVSync);
        assert_eq!(vsync, 1u32);

        let is_full = rd.get_capture_option_u32(CaptureOption::AllowFullscreen);
        assert_eq!(is_full, 1u32);

        let api_val_mode = rd.get_capture_option_u32(CaptureOption::ApiValidation);
        let debug_mode = rd.get_capture_option_u32(CaptureOption::ApiValidation);
        assert_eq!(api_val_mode, 0u32);
        assert_eq!(api_val_mode, debug_mode);

        let cc = rd.get_capture_option_u32(CaptureOption::CaptureCallstacks);
        assert_eq!(cc, 0u32);

        let cc_draw = rd.get_capture_option_u32(CaptureOption::CaptureCallstacksOnlyDraws);
        assert_eq!(cc_draw, 0u32);

        let ver_map = rd.get_capture_option_u32(CaptureOption::VerifyMapWrites);
        assert_eq!(ver_map, 0u32);

        let hook_in = rd.get_capture_option_u32(CaptureOption::HookIntoChildren);
        assert_eq!(hook_in, 0u32);

        let ref_all = rd.get_capture_option_u32(CaptureOption::RefAllResources);
        assert_eq!(ref_all, 0u32);

        let intls = rd.get_capture_option_u32(CaptureOption::SaveAllInitials);
        assert_eq!(intls, 0u32);

        let cmds = rd.get_capture_option_u32(CaptureOption::CaptureAllCmdLists);
        assert_eq!(cmds, 0u32);

        let is_muted = rd.get_capture_option_u32(CaptureOption::DebugOutputMute);
        assert_eq!(is_muted, 1u32);
    }
}
