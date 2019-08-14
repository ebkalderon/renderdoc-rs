//! Types for configuring the behavior of RenderDoc frame captures.

use std::u32;

#[cfg(feature = "glutin")]
use glutin::VirtualKeyCode;

/// RenderDoc capture options.
#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CaptureOption {
    /// Let the application enable vertical synchronization.
    AllowVSync = renderdoc_sys::eRENDERDOC_Option_AllowVSync,
    /// Let the application enter fullscreen mode.
    AllowFullscreen = renderdoc_sys::eRENDERDOC_Option_AllowFullscreen,
    /// Record API debugging events and messages.
    ///
    /// This option also goes by the deprecated name of `DebugDeviceMode`.
    ApiValidation = renderdoc_sys::eRENDERDOC_Option_APIValidation,
    /// Capture CPU callstacks for API events.
    CaptureCallstacks = renderdoc_sys::eRENDERDOC_Option_CaptureCallstacks,
    /// When capturing CPU callstacks, only capture them from drawcalls.
    ///
    /// This option does nothing without the above option being enabled.
    CaptureCallstacksOnlyDraws = renderdoc_sys::eRENDERDOC_Option_CaptureCallstacksOnlyDraws,
    /// Specify a delay, measured in seconds, to wait for a debugger to attach to the application
    /// after being injected.
    DelayForDebugger = renderdoc_sys::eRENDERDOC_Option_DelayForDebugger,
    /// Verify any writes to mapped buffers by checking the memory after the bounds of the
    /// returned pointer to detect any modification.
    VerifyMapWrites = renderdoc_sys::eRENDERDOC_Option_VerifyMapWrites,
    /// Hooks any system API calls that create child processes and injects RenderDoc into them
    /// recursively with the same options.
    HookIntoChildren = renderdoc_sys::eRENDERDOC_Option_HookIntoChildren,
    /// Reference all resources available at the time of capture.
    ///
    /// By default, RenderDoc only includes resources in the final capture file necessary for that
    /// frame. This option allows you to override that behavior.
    RefAllResources = renderdoc_sys::eRENDERDOC_Option_RefAllResources,
    /// Save the initial state for all resources, regardless of usage.
    ///
    /// By default, RenderDoc skips saving initial states for resources where the previous
    /// contents don't appear to be used (assuming that writes before reads indicate the previous
    /// contents aren't used).
    SaveAllInitials = renderdoc_sys::eRENDERDOC_Option_SaveAllInitials,
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
    CaptureAllCmdLists = renderdoc_sys::eRENDERDOC_Option_CaptureAllCmdLists,
    /// Mute API debug output when `CaptureOption::ApiValidation` is enabled.
    DebugOutputMute = renderdoc_sys::eRENDERDOC_Option_DebugOutputMute,
    /// Allow all vendor extensions to be used, even when they may be incompatible with RenderDoc
    /// and could potentially cause corrupted replays or crashes.
    AllowUnsupportedVendorExtensions =
        renderdoc_sys::eRENDERDOC_Option_AllowUnsupportedVendorExtensions,
}

/// User input key codes.
#[allow(missing_docs)]
#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum InputButton {
    /// The '0' key over the letters.
    Key0 = renderdoc_sys::eRENDERDOC_Key_0,
    /// The '1' key over the letters.
    Key1 = renderdoc_sys::eRENDERDOC_Key_1,
    /// The '2' key over the letters.
    Key2 = renderdoc_sys::eRENDERDOC_Key_2,
    /// The '3' key over the letters.
    Key3 = renderdoc_sys::eRENDERDOC_Key_3,
    /// The '4' key over the letters.
    Key4 = renderdoc_sys::eRENDERDOC_Key_4,
    /// The '5' key over the letters.
    Key5 = renderdoc_sys::eRENDERDOC_Key_5,
    /// The '6' key over the letters.
    Key6 = renderdoc_sys::eRENDERDOC_Key_6,
    /// The '7' key over the letters.
    Key7 = renderdoc_sys::eRENDERDOC_Key_7,
    /// The '8' key over the letters.
    Key8 = renderdoc_sys::eRENDERDOC_Key_8,
    /// The '9' key over the letters.
    Key9 = renderdoc_sys::eRENDERDOC_Key_9,

    A = renderdoc_sys::eRENDERDOC_Key_A,
    B = renderdoc_sys::eRENDERDOC_Key_B,
    C = renderdoc_sys::eRENDERDOC_Key_C,
    D = renderdoc_sys::eRENDERDOC_Key_D,
    E = renderdoc_sys::eRENDERDOC_Key_E,
    F = renderdoc_sys::eRENDERDOC_Key_F,
    G = renderdoc_sys::eRENDERDOC_Key_G,
    H = renderdoc_sys::eRENDERDOC_Key_H,
    I = renderdoc_sys::eRENDERDOC_Key_I,
    J = renderdoc_sys::eRENDERDOC_Key_J,
    K = renderdoc_sys::eRENDERDOC_Key_K,
    L = renderdoc_sys::eRENDERDOC_Key_L,
    M = renderdoc_sys::eRENDERDOC_Key_M,
    N = renderdoc_sys::eRENDERDOC_Key_N,
    O = renderdoc_sys::eRENDERDOC_Key_O,
    P = renderdoc_sys::eRENDERDOC_Key_P,
    Q = renderdoc_sys::eRENDERDOC_Key_Q,
    R = renderdoc_sys::eRENDERDOC_Key_R,
    S = renderdoc_sys::eRENDERDOC_Key_S,
    T = renderdoc_sys::eRENDERDOC_Key_T,
    U = renderdoc_sys::eRENDERDOC_Key_U,
    V = renderdoc_sys::eRENDERDOC_Key_V,
    W = renderdoc_sys::eRENDERDOC_Key_W,
    X = renderdoc_sys::eRENDERDOC_Key_X,
    Y = renderdoc_sys::eRENDERDOC_Key_Y,
    Z = renderdoc_sys::eRENDERDOC_Key_Z,

    /// Leave the rest of the ASCII range free, in case the RenderDoc developers decide to use it
    /// later.
    NonPrintable = renderdoc_sys::eRENDERDOC_Key_NonPrintable,

    /// Division key on the numpad.
    Divide = renderdoc_sys::eRENDERDOC_Key_Divide,
    /// Multiplication key on the numpad.
    Multiply = renderdoc_sys::eRENDERDOC_Key_Multiply,
    /// Subtraction key on the numpad.
    Subtract = renderdoc_sys::eRENDERDOC_Key_Subtract,
    /// Addition key on the numpad.
    Plus = renderdoc_sys::eRENDERDOC_Key_Plus,

    F1 = renderdoc_sys::eRENDERDOC_Key_F1,
    F2 = renderdoc_sys::eRENDERDOC_Key_F2,
    F3 = renderdoc_sys::eRENDERDOC_Key_F3,
    F4 = renderdoc_sys::eRENDERDOC_Key_F4,
    F5 = renderdoc_sys::eRENDERDOC_Key_F5,
    F6 = renderdoc_sys::eRENDERDOC_Key_F6,
    F7 = renderdoc_sys::eRENDERDOC_Key_F7,
    F8 = renderdoc_sys::eRENDERDOC_Key_F8,
    F9 = renderdoc_sys::eRENDERDOC_Key_F9,
    F10 = renderdoc_sys::eRENDERDOC_Key_F10,
    F11 = renderdoc_sys::eRENDERDOC_Key_F11,
    F12 = renderdoc_sys::eRENDERDOC_Key_F12,

    Home = renderdoc_sys::eRENDERDOC_Key_Home,
    End = renderdoc_sys::eRENDERDOC_Key_End,
    Insert = renderdoc_sys::eRENDERDOC_Key_Insert,
    Delete = renderdoc_sys::eRENDERDOC_Key_Delete,
    PageUp = renderdoc_sys::eRENDERDOC_Key_PageUp,
    PageDn = renderdoc_sys::eRENDERDOC_Key_PageDn,

    Backspace = renderdoc_sys::eRENDERDOC_Key_Backspace,
    Tab = renderdoc_sys::eRENDERDOC_Key_Tab,
    PrtScrn = renderdoc_sys::eRENDERDOC_Key_PrtScrn,
    Pause = renderdoc_sys::eRENDERDOC_Key_Pause,

    Max = renderdoc_sys::eRENDERDOC_Key_Max,
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
        const ENABLED = renderdoc_sys::eRENDERDOC_Overlay_Enabled;
        /// Shows the average, minimum, and maximum sampled frame rate.
        const FRAME_RATE = renderdoc_sys::eRENDERDOC_Overlay_FrameRate;
        /// Shows the current frame number.
        const FRAME_NUMBER = renderdoc_sys::eRENDERDOC_Overlay_FrameNumber;
        /// Shows a list of recent captures, out of the total captures made.
        const CAPTURE_LIST = renderdoc_sys::eRENDERDOC_Overlay_CaptureList;
        /// Sets the default configuration for the overlay.
        const DEFAULT = renderdoc_sys::eRENDERDOC_Overlay_Default;
        /// Enables all overlay configuration bits.
        const ALL = u32::MAX;
        /// Disables all overlay configuration bits.
        const NONE = u32::MIN;
    }
}
