//! Traits providing statically guaranteed API version compatibility.

use std::ffi::{CStr, CString};
use std::path::Path;
use std::{mem, ptr};

use entry::{EntryV100, EntryV110, EntryV111, EntryV112, EntryV120};
use {CaptureOption, DevicePointer, InputButton, OverlayBits, WindowHandle};

/// Base implementation of API version 1.0.0.
pub trait RenderDocV100: Sized {
    /// Returns the raw `EntryV100` entry point struct.
    unsafe fn entry_v100(&self) -> &EntryV100;

    /// Returns the major, minor, and patch version numbers of the RenderDoc API currently in use.
    ///
    /// Note that RenderDoc will usually provide a higher API version than the one requested by
    /// the user if it's backwards compatible.
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

    /// Sets the specified `CaptureOption` to the given `f32` value.
    ///
    /// # Panics
    ///
    /// This method will panic if the option and/or the value are invalid.
    fn set_capture_option_f32(&mut self, opt: CaptureOption, val: f32) {
        let err = unsafe { (self.entry_v100().SetCaptureOptionF32.unwrap())(opt as u32, val) };
        assert_eq!(err, 1);
    }

    /// Sets the specified `CaptureOption` to the given `u32` value.
    ///
    /// # Panics
    ///
    /// This method will panic if the option and/or the value are invalid.
    fn set_capture_option_u32(&mut self, opt: CaptureOption, val: u32) {
        let err = unsafe { (self.entry_v100().SetCaptureOptionU32.unwrap())(opt as u32, val) };
        assert_eq!(err, 1);
    }

    /// Returns the value of the given `CaptureOption` as an `f32` value.
    ///
    /// # Panics
    ///
    /// This method will panic if the option is invalid.
    fn get_capture_option_f32(&self, opt: CaptureOption) -> f32 {
        use std::f32::MAX;
        let val = unsafe { (self.entry_v100().GetCaptureOptionF32.unwrap())(opt as u32) };
        assert_ne!(val, -MAX);
        val
    }

    /// Returns the value of the given `CaptureOption` as a `u32` value.
    ///
    /// # Panics
    ///
    /// This method will panic if the option is invalid.
    fn get_capture_option_u32(&self, opt: CaptureOption) -> u32 {
        use std::u32::MAX;
        let val = unsafe { (self.entry_v100().GetCaptureOptionU32.unwrap())(opt as u32) };
        assert_ne!(val, MAX);
        val
    }

    #[allow(missing_docs)]
    fn set_capture_keys<I: Into<InputButton> + Clone>(&mut self, keys: &[I]) {
        unsafe {
            let mut k: Vec<_> = keys.iter().cloned().map(|k| k.into() as u32).collect();
            (self.entry_v100().SetCaptureKeys.unwrap())(k.as_mut_ptr(), k.len() as i32)
        }
    }

    #[allow(missing_docs)]
    fn set_focus_toggle_keys<I: Into<InputButton> + Clone>(&mut self, keys: &[I]) {
        unsafe {
            let mut k: Vec<_> = keys.iter().cloned().map(|k| k.into() as u32).collect();
            (self.entry_v100().SetFocusToggleKeys.unwrap())(k.as_mut_ptr(), k.len() as i32)
        }
    }

    /// Attempts to shut down RenderDoc.
    ///
    /// # Safety
    ///
    /// Note that this will work correctly if done _immediately_ after the dynamic library is
    /// loaded, before any API work happens. At that point, RenderDoc will remove its injected
    /// hooks and shut down. Behavior is undefined if this is called after any API functions have
    /// been called.
    unsafe fn shutdown(self) {
        (self.entry_v100().Shutdown.unwrap())();
    }

    #[allow(missing_docs)]
    fn unload_crash_handler(&mut self) {
        unsafe {
            (self.entry_v100().UnloadCrashHandler.unwrap())();
        }
    }

    #[allow(missing_docs)]
    fn get_overlay_bits(&self) -> OverlayBits {
        let bits = unsafe { (self.entry_v100().GetOverlayBits.unwrap())() };
        OverlayBits::from_bits_truncate(bits)
    }

    #[allow(missing_docs)]
    fn mask_overlay_bits(&mut self, and: OverlayBits, or: OverlayBits) {
        unsafe {
            (self.entry_v100().MaskOverlayBits.unwrap())(and.bits(), or.bits());
        }
    }

    #[allow(missing_docs)]
    fn get_log_file_path_template(&self) -> &str {
        unsafe {
            let raw = (self
                .entry_v100()
                .__bindgen_anon_2
                .GetLogFilePathTemplate
                .unwrap())();
            CStr::from_ptr(raw).to_str().unwrap()
        }
    }

    #[allow(missing_docs)]
    fn set_log_file_path_template<P: AsRef<Path>>(&mut self, path_template: P) {
        unsafe {
            let bytes = mem::transmute(path_template.as_ref().as_os_str());
            let cstr = CStr::from_bytes_with_nul_unchecked(bytes);
            (self
                .entry_v100()
                .__bindgen_anon_1
                .SetLogFilePathTemplate
                .unwrap())(cstr.as_ptr());
        }
    }

    #[allow(missing_docs)]
    fn get_num_captures(&self) -> u32 {
        unsafe { (self.entry_v100().GetNumCaptures.unwrap())() }
    }

    #[allow(missing_docs)]
    fn get_capture(&self, index: u32) -> Option<(String, u64)> {
        unsafe {
            let mut len = self.get_log_file_path_template().len() as u32 + 128;
            let mut path = Vec::with_capacity(len as usize);
            let mut time = 0u64;

            if (self.entry_v100().GetCapture.unwrap())(
                index,
                path.as_mut_ptr(),
                &mut len,
                &mut time,
            ) == 1
            {
                let raw_path = CString::from_raw(path.as_mut_ptr());
                let mut path = raw_path.into_string().unwrap();
                path.shrink_to_fit();

                Some((path, time))
            } else {
                None
            }
        }
    }

    /// Captures the next frame from the currently active window and API device.
    ///
    /// Data is saved to a capture log file at the location specified via
    /// `set_log_file_path_template()`.
    fn trigger_capture(&mut self) {
        unsafe {
            (self.entry_v100().TriggerCapture.unwrap())();
        }
    }

    #[allow(missing_docs)]
    fn is_remote_access_connected(&self) -> bool {
        unsafe {
            (self
                .entry_v100()
                .__bindgen_anon_3
                .IsRemoteAccessConnected
                .unwrap())()
                == 1
        }
    }

    #[allow(missing_docs)]
    fn launch_replay_ui<'a, O>(&self, connect_immediately: bool, extra_opts: O) -> Result<u32, ()>
    where
        O: Into<Option<&'a str>>,
    {
        let extra_opts = extra_opts.into().and_then(|s| CString::new(s).ok());
        let should_connect = connect_immediately as u32;
        let command_str = extra_opts
            .as_ref()
            .map(|s| s.as_ptr())
            .unwrap_or_else(|| ptr::null());

        unsafe {
            match (self.entry_v100().LaunchReplayUI.unwrap())(should_connect, command_str) {
                0 => Err(()),
                pid => Ok(pid),
            }
        }
    }

    #[allow(missing_docs)]
    fn set_active_window<D>(&mut self, dev: D, win: WindowHandle)
    where
        D: Into<DevicePointer>,
    {
        unsafe {
            let DevicePointer(dev) = dev.into();
            (self.entry_v100().SetActiveWindow.unwrap())(dev as *mut _, win as *mut _);
        }
    }

    #[allow(missing_docs)]
    fn start_frame_capture<D>(&mut self, dev: D, win: WindowHandle)
    where
        D: Into<DevicePointer>,
    {
        unsafe {
            let DevicePointer(dev) = dev.into();
            (self.entry_v100().StartFrameCapture.unwrap())(dev as *mut _, win as *mut _);
        }
    }

    /// Returns whether or not a frame capture is currently ongoing anywhere.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use renderdoc::{RenderDoc, V100};
    /// # use renderdoc::prelude::*;
    /// # fn init() -> Result<(), String> {
    /// # let renderdoc: RenderDoc<V100> = RenderDoc::new()?;
    /// if renderdoc.is_frame_capturing() {
    ///     println!("Frames are being captured.");
    /// } else {
    ///     println!("No frame capture is occurring.");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    fn is_frame_capturing(&self) -> bool {
        unsafe { (self.entry_v100().IsFrameCapturing.unwrap())() == 1 }
    }

    #[allow(missing_docs)]
    fn end_frame_capture<D>(&mut self, dev: D, win: WindowHandle)
    where
        D: Into<DevicePointer>,
    {
        unsafe {
            let DevicePointer(dev) = dev.into();
            (self.entry_v100().EndFrameCapture.unwrap())(dev as *mut _, win as *mut _);
        }
    }
}

/// Additional features for API version 1.1.0.
pub trait RenderDocV110: RenderDocV100 {
    /// Returns the raw `EntryV110` entry point struct.
    unsafe fn entry_v110(&self) -> &EntryV110;

    /// Captures the next _n_ frames from the currently active window and API device.
    ///
    /// Data is saved to a capture log file at the location specified via
    /// `set_log_file_path_template()`.
    fn trigger_multi_frame_capture(&self, num_frames: u32) {
        unsafe {
            (self.entry_v110().TriggerMultiFrameCapture.unwrap())(num_frames);
        }
    }
}

/// Additional features for API version 1.1.1.
pub trait RenderDocV111: RenderDocV110 {
    /// Returns the raw `EntryV111` entry point struct.
    unsafe fn entry_v111(&self) -> &EntryV111;

    #[allow(missing_docs)]
    fn is_target_control_connected(&self) -> bool {
        unsafe {
            (self
                .entry_v111()
                .__bindgen_anon_3
                .IsTargetControlConnected
                .unwrap())()
                == 1
        }
    }
}

/// Additional features for API version 1.1.2.
pub trait RenderDocV112: RenderDocV111 {
    /// Returns the raw `EntryV112` entry point struct.
    unsafe fn entry_v112(&self) -> &EntryV112;

    #[allow(missing_docs)]
    fn get_capture_file_path_template(&self) -> &str {
        unsafe {
            let raw = (self
                .entry_v112()
                .__bindgen_anon_2
                .GetCaptureFilePathTemplate
                .unwrap())();
            CStr::from_ptr(raw).to_str().unwrap()
        }
    }

    #[allow(missing_docs)]
    fn set_capture_file_path_template<P: AsRef<Path>>(&mut self, path_template: P) {
        unsafe {
            let bytes = mem::transmute(path_template.as_ref().as_os_str());
            let cstr = CStr::from_bytes_with_nul_unchecked(bytes);
            (self
                .entry_v112()
                .__bindgen_anon_1
                .SetCaptureFilePathTemplate
                .unwrap())(cstr.as_ptr());
        }
    }
}

/// Additional features for API version 1.2.0.
pub trait RenderDocV120: RenderDocV112 {
    /// Returns the raw `EntryV210` entry point struct.
    unsafe fn entry_v120(&self) -> &EntryV120;

    #[allow(missing_docs)]
    fn set_capture_file_comments<P, C>(&self, path: P, comments: C)
    where
        P: Into<Option<&'static str>>,
        C: AsRef<str>,
    {
        unsafe {
            let with_path = path.into();
            let path = if let Some(ref path) = with_path {
                let bytes = path.as_bytes();
                CStr::from_bytes_with_nul_unchecked(bytes)
            } else {
                CStr::from_ptr(ptr::null())
            };

            let comments = {
                let bytes = comments.as_ref().as_bytes();
                CStr::from_bytes_with_nul_unchecked(bytes)
            };

            (self.entry_v120().SetCaptureFileComments.unwrap())(path.as_ptr(), comments.as_ptr());
        }
    }
}
