//! Traits providing compile-time API functionality.

use super::ffi;
use super::types::{CaptureOption, DevicePointer, InputButton, OverlayBits, WindowHandle};

use std::ffi::{CStr, CString};
use std::mem;
use std::path::Path;

/// Base implementation of API version 1.0.0.
pub trait RenderDocV100: Sized {
    /// Returns the raw `RENDERDOC_API_1_0_0` entry point struct.
    unsafe fn entry_v100(&self) -> &ffi::RENDERDOC_API_1_0_0;

    /// Provides the major, minor, and patch version numbers of the RenderDoc
    /// API given to the application.
    ///
    /// Note that RenderDoc will usually provide a higher API version than the
    /// one requested by the user if it's backwards compatible.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use renderdoc::{RenderDoc, V100};
    /// # use renderdoc::prelude::*;
    /// # fn init() -> Result<(), String> {
    /// # let renderdoc: RenderDoc<V100> = RenderDoc::new()?;
    /// let (major, minor, patch) = renderdoc.get_api_version();
    /// assert_eq!(major, 1u32);
    /// assert_eq!(minor, 0u32);
    /// # Ok(())
    /// # }
    /// ```
    fn get_api_version(&self) -> (u32, u32, u32) {
        unsafe {
            let (mut major, mut minor, mut patch) = (0, 0, 0);
            (self.entry_v100().GetAPIVersion.unwrap())(&mut major, &mut minor, &mut patch);
            (major as u32, minor as u32, patch as u32)
        }
    }

    /// Changes the key bindings in-application for triggering a capture on the
    /// current window.
    fn set_capture_keys<I: Into<InputButton> + Clone>(&mut self, keys: &[I]) {
        unsafe {
            let mut k: Vec<_> = keys.iter().cloned().map(|k| k.into() as u32).collect();
            (self.entry_v100().SetCaptureKeys.unwrap())(k.as_mut_ptr(), k.len() as i32)
        }
    }

    /// Changes the key bindings in-application for changing the focused window.
    fn set_focus_toggle_keys<K, I: Into<InputButton> + Clone>(&mut self, keys: &[I]) {
        unsafe {
            let mut k: Vec<_> = keys.iter().cloned().map(|k| k.into() as u32).collect();
            (self.entry_v100().SetFocusToggleKeys.unwrap())(k.as_mut_ptr(), k.len() as i32)
        }
    }

    /// Attempts to shut down RenderDoc.
    ///
    /// # Safety
    ///
    /// Note that this will work correctly if done _immediately_ after the
    /// dynamic library is loaded, before any API work happens. At that point,
    /// RenderDoc will remove its injected hooks and shut down. Behavior is
    /// undefined if this is called after any API functions have been called.
    unsafe fn shutdown(self) {
        (self.entry_v100().Shutdown.unwrap())();
    }

    #[allow(missing_docs)]
    fn set_active_window<D: DevicePointer, W: WindowHandle>(&mut self, dev: D, win: W) {
        unsafe {
            let device = dev.as_device_ptr();
            let window = win.as_window_handle();
            (self.entry_v100().SetActiveWindow.unwrap())(device, window);
        }
    }
}

/// Additional features for API version 1.1.0.
pub trait RenderDocV110: RenderDocV100 {
    /// Returns the raw `EntryV110` entry point struct.
    unsafe fn entry_v110(&self) -> &ffi::RENDERDOC_API_1_1_0;

    /// Captures the next _n_ frames from the currently active window and API
    /// device.
    ///
    /// Data is saved to a capture log file at the location specified via
    /// `set_log_file_path_template()`.
    fn trigger_multi_frame_capture(&self, num_frames: u32) {
        unsafe {
            (self.entry_v110().TriggerMultiFrameCapture.unwrap())(num_frames);
        }
    }
}
