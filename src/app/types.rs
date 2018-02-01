//! Common types used by the RenderDoc app API.

use super::ffi;

use std::os::raw::c_void;

use glutin::{self, VirtualKeyCode};
#[cfg(windows)]
use winapi;
#[cfg(windows)]
use wio::com::ComPtr;

/// RenderDoc capture options.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CaptureOption {
    /// Let the application enable vertical synchronization.
    AllowVSync,
    /// Let the application enter fullscreen mode.
    AllowFullscreen,
    /// Record API debugging events and messages.
    ///
    /// This option also goes by the deprecated name of `DebugDeviceMode`.
    ApiValidation,
    /// Capture CPU callstacks for API events.
    CaptureCallstacks,
    /// When capturing CPU callstacks, only capture them from drawcalls.
    ///
    /// This option does nothing without the above option being enabled.
    CaptureCallstacksOnlyDraws,
    /// Specify a delay, measured in seconds, to wait for a debugger to attach
    /// to the application after being injected.
    DelayForDebugger,
    /// Verify any writes to mapped buffers by checking the memory after the
    /// bounds of the returned pointer to detect any modification.
    VerifyMapWrites,
    /// Hooks any system API calls that create child processes and injects
    /// RenderDoc into them recursively with the same options.
    HookIntoChildren,
    /// Reference all resources available at the time of capture.
    ///
    /// By default, RenderDoc only includes resources in the final capture file
    /// necessary for that frame. This option allows you to override that
    /// behavior.
    RefAllResources,
    /// Save the initial state for all resources, regardless of usage.
    ///
    /// By default, RenderDoc skips saving initial states for resources where
    /// the previous contents don't appear to be used (assuming that writes
    /// before reads indicate the previous contents aren't used).
    SaveAllInitials,
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
    CaptureAllCmdLists,
    /// Mute API debug output when `CaptureOption::ApiValidation` is enabled.
    DebugOutputMute,
}

impl From<CaptureOption> for ffi::RENDERDOC_CaptureOption {
    fn from(opt: CaptureOption) -> Self {
        match opt {
            CaptureOption::AllowVSync => ffi::RENDERDOC_CaptureOption_eRENDERDOC_Option_AllowVSync,
            CaptureOption::AllowFullscreen => {
                ffi::RENDERDOC_CaptureOption_eRENDERDOC_Option_AllowFullscreen
            }
            CaptureOption::ApiValidation => {
                ffi::RENDERDOC_CaptureOption_eRENDERDOC_Option_DebugDeviceMode
            }
            CaptureOption::CaptureCallstacks => {
                ffi::RENDERDOC_CaptureOption_eRENDERDOC_Option_CaptureCallstacks
            }
            CaptureOption::CaptureCallstacksOnlyDraws => {
                ffi::RENDERDOC_CaptureOption_eRENDERDOC_Option_CaptureCallstacksOnlyDraws
            }
            CaptureOption::DelayForDebugger => {
                ffi::RENDERDOC_CaptureOption_eRENDERDOC_Option_DelayForDebugger
            }
            CaptureOption::VerifyMapWrites => {
                ffi::RENDERDOC_CaptureOption_eRENDERDOC_Option_VerifyMapWrites
            }
            CaptureOption::HookIntoChildren => {
                ffi::RENDERDOC_CaptureOption_eRENDERDOC_Option_HookIntoChildren
            }
            CaptureOption::RefAllResources => {
                ffi::RENDERDOC_CaptureOption_eRENDERDOC_Option_RefAllResources
            }
            CaptureOption::SaveAllInitials => {
                ffi::RENDERDOC_CaptureOption_eRENDERDOC_Option_SaveAllInitials
            }
            CaptureOption::CaptureAllCmdLists => {
                ffi::RENDERDOC_CaptureOption_eRENDERDOC_Option_CaptureAllCmdLists
            }
            CaptureOption::DebugOutputMute => {
                ffi::RENDERDOC_CaptureOption_eRENDERDOC_Option_DebugOutputMute
            }
        }
    }
}

/// User input key codes.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum InputButton {
    /// The '0' key over the letters.
    Key0,
    /// The '1' key over the letters.
    Key1,
    /// The '2' key over the letters.
    Key2,
    /// The '3' key over the letters.
    Key3,
    /// The '4' key over the letters.
    Key4,
    /// The '5' key over the letters.
    Key5,
    /// The '6' key over the letters.
    Key6,
    /// The '7' key over the letters.
    Key7,
    /// The '8' key over the letters.
    Key8,
    /// The '9' key over the letters.
    Key9,

    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    /// Leave the rest of the ASCII range free, in case the RenderDoc developers
    /// decide to use it later.
    NonPrintable,

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

impl From<InputButton> for ffi::RENDERDOC_InputButton {
    fn from(button: InputButton) -> Self {
        match button {
            InputButton::Key0 => ffi::RENDERDOC_InputButton_eRENDERDOC_Key_0,
            InputButton::Key1 => ffi::RENDERDOC_InputButton_eRENDERDOC_Key_1,
            InputButton::Key2 => ffi::RENDERDOC_InputButton_eRENDERDOC_Key_2,
            InputButton::Key3 => ffi::RENDERDOC_InputButton_eRENDERDOC_Key_3,
            InputButton::Key4 => ffi::RENDERDOC_InputButton_eRENDERDOC_Key_4,
            InputButton::Key5 => ffi::RENDERDOC_InputButton_eRENDERDOC_Key_5,
            InputButton::Key6 => ffi::RENDERDOC_InputButton_eRENDERDOC_Key_6,
            InputButton::Key7 => ffi::RENDERDOC_InputButton_eRENDERDOC_Key_7,
            InputButton::Key8 => ffi::RENDERDOC_InputButton_eRENDERDOC_Key_8,
            InputButton::Key9 => ffi::RENDERDOC_InputButton_eRENDERDOC_Key_9,
            InputButton::A => ffi::RENDERDOC_InputButton_eRENDERDOC_Key_A,
            InputButton::B => ffi::RENDERDOC_InputButton_eRENDERDOC_Key_B,
            InputButton::C => ffi::RENDERDOC_InputButton_eRENDERDOC_Key_C,
            InputButton::D => ffi::RENDERDOC_InputButton_eRENDERDOC_Key_D,
            InputButton::E => ffi::RENDERDOC_InputButton_eRENDERDOC_Key_E,
            InputButton::F => ffi::RENDERDOC_InputButton_eRENDERDOC_Key_F,
            InputButton::G => ffi::RENDERDOC_InputButton_eRENDERDOC_Key_G,
            InputButton::H => ffi::RENDERDOC_InputButton_eRENDERDOC_Key_H,
            InputButton::I => ffi::RENDERDOC_InputButton_eRENDERDOC_Key_I,
            InputButton::J => ffi::RENDERDOC_InputButton_eRENDERDOC_Key_J,
            InputButton::K => ffi::RENDERDOC_InputButton_eRENDERDOC_Key_K,
            InputButton::L => ffi::RENDERDOC_InputButton_eRENDERDOC_Key_L,
            InputButton::M => ffi::RENDERDOC_InputButton_eRENDERDOC_Key_M,
            InputButton::N => ffi::RENDERDOC_InputButton_eRENDERDOC_Key_N,
            InputButton::O => ffi::RENDERDOC_InputButton_eRENDERDOC_Key_O,
            InputButton::P => ffi::RENDERDOC_InputButton_eRENDERDOC_Key_P,
            InputButton::Q => ffi::RENDERDOC_InputButton_eRENDERDOC_Key_Q,
            InputButton::R => ffi::RENDERDOC_InputButton_eRENDERDOC_Key_R,
            InputButton::S => ffi::RENDERDOC_InputButton_eRENDERDOC_Key_S,
            InputButton::T => ffi::RENDERDOC_InputButton_eRENDERDOC_Key_T,
            InputButton::U => ffi::RENDERDOC_InputButton_eRENDERDOC_Key_U,
            InputButton::V => ffi::RENDERDOC_InputButton_eRENDERDOC_Key_V,
            InputButton::W => ffi::RENDERDOC_InputButton_eRENDERDOC_Key_W,
            InputButton::X => ffi::RENDERDOC_InputButton_eRENDERDOC_Key_X,
            InputButton::Y => ffi::RENDERDOC_InputButton_eRENDERDOC_Key_Y,
            InputButton::Z => ffi::RENDERDOC_InputButton_eRENDERDOC_Key_Z,
            InputButton::NonPrintable => ffi::RENDERDOC_InputButton_eRENDERDOC_Key_NonPrintable,
            InputButton::Divide => ffi::RENDERDOC_InputButton_eRENDERDOC_Key_Divide,
            InputButton::Multiply => ffi::RENDERDOC_InputButton_eRENDERDOC_Key_Multiply,
            InputButton::Subtract => ffi::RENDERDOC_InputButton_eRENDERDOC_Key_Subtract,
            InputButton::Plus => ffi::RENDERDOC_InputButton_eRENDERDOC_Key_Plus,
            InputButton::F1 => ffi::RENDERDOC_InputButton_eRENDERDOC_Key_F1,
            InputButton::F2 => ffi::RENDERDOC_InputButton_eRENDERDOC_Key_F2,
            InputButton::F3 => ffi::RENDERDOC_InputButton_eRENDERDOC_Key_F3,
            InputButton::F4 => ffi::RENDERDOC_InputButton_eRENDERDOC_Key_F4,
            InputButton::F5 => ffi::RENDERDOC_InputButton_eRENDERDOC_Key_F5,
            InputButton::F6 => ffi::RENDERDOC_InputButton_eRENDERDOC_Key_F6,
            InputButton::F7 => ffi::RENDERDOC_InputButton_eRENDERDOC_Key_F7,
            InputButton::F8 => ffi::RENDERDOC_InputButton_eRENDERDOC_Key_F8,
            InputButton::F9 => ffi::RENDERDOC_InputButton_eRENDERDOC_Key_F9,
            InputButton::F10 => ffi::RENDERDOC_InputButton_eRENDERDOC_Key_F10,
            InputButton::F11 => ffi::RENDERDOC_InputButton_eRENDERDOC_Key_F11,
            InputButton::F12 => ffi::RENDERDOC_InputButton_eRENDERDOC_Key_F12,
            InputButton::Home => ffi::RENDERDOC_InputButton_eRENDERDOC_Key_Home,
            InputButton::End => ffi::RENDERDOC_InputButton_eRENDERDOC_Key_End,
            InputButton::Insert => ffi::RENDERDOC_InputButton_eRENDERDOC_Key_Insert,
            InputButton::Delete => ffi::RENDERDOC_InputButton_eRENDERDOC_Key_Delete,
            InputButton::PageUp => ffi::RENDERDOC_InputButton_eRENDERDOC_Key_PageUp,
            InputButton::PageDn => ffi::RENDERDOC_InputButton_eRENDERDOC_Key_PageDn,
            InputButton::Backspace => ffi::RENDERDOC_InputButton_eRENDERDOC_Key_Backspace,
            InputButton::Tab => ffi::RENDERDOC_InputButton_eRENDERDOC_Key_Tab,
            InputButton::PrtScrn => ffi::RENDERDOC_InputButton_eRENDERDOC_Key_PrtScrn,
            InputButton::Pause => ffi::RENDERDOC_InputButton_eRENDERDOC_Key_Pause,
            InputButton::Max => ffi::RENDERDOC_InputButton_eRENDERDOC_Key_Max,
        }
    }
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
    pub struct OverlayBits: ffi::RENDERDOC_OverlayBits {
        /// Controls whether the overlay is enabled or disabled globally.
        const ENABLED = ffi::RENDERDOC_OverlayBits_eRENDERDOC_Overlay_Enabled;
        /// Shows the average, minimum, and maximum sampled frame rate.
        const FRAME_RATE = ffi::RENDERDOC_OverlayBits_eRENDERDOC_Overlay_FrameRate;
        /// Shows the current frame number.
        const FRAME_NUMBER = ffi::RENDERDOC_OverlayBits_eRENDERDOC_Overlay_FrameNumber;
        /// Shows a list of recent captures, out of the total captures made.
        const CAPTURE_LIST = ffi::RENDERDOC_OverlayBits_eRENDERDOC_Overlay_CaptureList;
        /// Sets the default configuration for the overlay.
        const DEFAULT = ffi::RENDERDOC_OverlayBits_eRENDERDOC_Overlay_Default;
        /// Enables all overlay configuration bits.
        const ALL = ffi::RENDERDOC_OverlayBits_eRENDERDOC_Overlay_All;
        /// Disables all overlay configuration bits.
        const NONE = ffi::RENDERDOC_OverlayBits_eRENDERDOC_Overlay_None;
    }
}

/// Root handle to a graphics device supported by RenderDoc.
///
/// For example, this could be a pointer to an `ID3D11Device`, `ID3D12Device`,
/// `HGLRC`/`GLXContext`/`EGLContext`, etc.
///
/// TODO: Need to add Vulkan support.
pub trait DevicePointer {
    /// Returns a raw pointer to the API's root handle.
    fn as_device_ptr(&self) -> *mut c_void;
}

impl DevicePointer for *const c_void {
    fn as_device_ptr(&self) -> *mut c_void {
        *self as *mut c_void
    }
}

impl DevicePointer for *mut c_void {
    fn as_device_ptr(&self) -> *mut c_void {
        *self
    }
}

#[cfg(windows)]
impl DevicePointer for winapi::shared::windef::HGLRC {
    fn as_device_ptr(&self) -> *mut c_void {
        *self as *mut _ as *mut c_void
    }
}

#[cfg(windows)]
impl DevicePointer for *mut winapi::um::d3d11::ID3D11Device {
    fn as_device_ptr(&self) -> *mut c_void {
        *self as *mut _ as *mut c_void
    }
}

#[cfg(windows)]
impl DevicePointer for ComPtr<winapi::um::d3d11::ID3D11Device> {
    fn as_device_ptr(&self) -> *mut c_void {
        unsafe { self.as_mut() as *mut _ as *mut c_void }
    }
}

#[cfg(windows)]
impl DevicePointer for *mut winapi::um::d3d12::ID3D12Device {
    fn as_device_ptr(&self) -> *mut c_void {
        *self as *mut _ as *mut c_void
    }
}

#[cfg(windows)]
impl DevicePointer for ComPtr<winapi::um::d3d12::ID3D12Device> {
    fn as_device_ptr(&self) -> *mut c_void {
        unsafe { self.as_mut() as *mut _ as *mut c_void }
    }
}

#[cfg(feature = "glutin")]
impl<'a> DevicePointer for &'a glutin::Context {
    fn as_device_ptr(&self) -> *mut c_void {
        use glutin::os::GlContextExt;

        #[cfg(unix)]
        unsafe {
            use glutin::os::unix::RawHandle;
            match self.raw_handle() {
                RawHandle::Glx(ctx) => ctx as *mut c_void,
                RawHandle::Egl(ctx) => ctx as *mut c_void,
            }
        }

        #[cfg(windows)]
        unsafe {
            use glutin::os::windows::RawHandle;
            match self.raw_handle() {
                RawHandle::Egl(ctx) => ctx as *mut c_void,
                RawHandle::Wgl(ctx) => ctx as *mut c_void,
            }
        }
    }
}

/// A window handle type supported by RenderDoc.
///
/// For example, this could be a pointer to an `HWND`, Xlib `Window`,
/// `xcb_window_t`, or `EGLSurface`.
pub trait WindowHandle {
    /// Returns the raw pointer to the window handle.
    fn as_window_handle(&self) -> *mut c_void;
}

impl WindowHandle for *mut c_void {
    fn as_window_handle(&self) -> *mut c_void {
        *self
    }
}

impl WindowHandle for *const c_void {
    fn as_window_handle(&self) -> *mut c_void {
        *self as *mut c_void
    }
}

#[cfg(windows)]
impl WindowHandle for winapi::shared::windef::HWND {
    fn as_window_handle(&self) -> *mut c_void {
        *self as *mut _ as *mut c_void
    }
}

#[cfg(feature = "glutin")]
impl<'a> WindowHandle for &'a glutin::Window {
    fn as_window_handle(&self) -> *mut c_void {
        #[cfg(unix)]
        {
            use glutin::os::unix::WindowExt;
            if let Some(id) = self.get_xlib_window() {
                let ptr: *const u64 = &id;
                return ptr as *mut c_void;
            } else if let Some(id) = self.get_wayland_surface() {
                return id as *mut c_void;
            } else {
                panic!("RenderDoc doesn't support this window handle type on Unix!");
            }
        }

        #[cfg(windows)]
        {
            use glutin::os::windows::WindowExt;
            self.get_hwnd() as *mut c_void
        }
    }
}
