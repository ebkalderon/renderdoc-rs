//! Types for configuring the behavior of RenderDoc frame captures.

use std::u32;

#[cfg(feature = "glutin")]
use glutin::VirtualKeyCode;

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
    /// Specify a delay, measured in seconds, to wait for a debugger to attach to the application
    /// after being injected.
    DelayForDebugger = 5,
    /// Verify any writes to mapped buffers by checking the memory after the bounds of the
    /// returned pointer to detect any modification.
    VerifyMapWrites = 6,
    /// Hooks any system API calls that create child processes and injects RenderDoc into them
    /// recursively with the same options.
    HookIntoChildren = 7,
    /// Reference all resources available at the time of capture.
    ///
    /// By default, RenderDoc only includes resources in the final capture file necessary for that
    /// frame. This option allows you to override that behavior.
    RefAllResources = 8,
    /// Save the initial state for all resources, regardless of usage.
    ///
    /// By default, RenderDoc skips saving initial states for resources where the previous
    /// contents don't appear to be used (assuming that writes before reads indicate the previous
    /// contents aren't used).
    SaveAllInitials = 9,
    /// Capture all command lists generated from the start of the application.
    ///
    /// In APIs that allow for recording of command lists to be replayed later, RenderDoc may
    /// choose to not capture command lists before a frame capture is triggered to reduce
    /// overhead. This means any command lists that are recorded one and replayed many times will
    /// not be available, potentially causing a failure to capture.
    ///
    /// Note that this is only true for APIs where multithreading is difficult or otherwise
    /// discouraged. Newer APIs, e.g. Vulkan and D3D12, will ignore this option and always capture
    /// all command lists since they are heavily oriented around them and the associated overhead
    /// is mostly reduced due to superior API design.
    CaptureAllCmdLists = 10,
    /// Mute API debug output when `CaptureOption::ApiValidation` is enabled.
    DebugOutputMute = 11,
    /// Allow all vendor extensions to be used, even when they may be incompatible with RenderDoc
    /// and could potentially cause corrupted replays or crashes.
    AllowUnsupportedVendorExtensions = 12,
}

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

    /// Leave the rest of the ASCII range free, in case the RenderDoc developers decide to use it
    /// later.
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

#[cfg(feature = "glutin")]
impl From<glutin::VirtualKeyCode> for InputButton {
    fn from(code: glutin::VirtualKeyCode) -> InputButton {
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
