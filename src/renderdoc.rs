//! Type-safe wrapper around the RenderDoc API.

use std::ffi::{CStr, CString};
use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::path::Path;
use std::ptr;

use float_cmp::ApproxEq;

use handles::{DevicePointer, WindowHandle};
use settings::{CaptureOption, InputButton, OverlayBits};
use version::{Entry, HasPrevious, Version, V100, V110, V111, V112, V120, V130, V140};

/// An instance of the RenderDoc API with baseline version `V`.
#[repr(C)]
#[derive(Eq, Hash, PartialEq, RenderDoc)]
#[renderdoc_convert(V100, V110, V111, V112, V120, V130, V140)]
pub struct RenderDoc<V>(*mut Entry, PhantomData<V>);

impl<V: Version> RenderDoc<V> {
    /// Initializes a new instance of the RenderDoc API.
    pub fn new() -> Result<Self, String> {
        let api = V::load()?;
        Ok(RenderDoc(api, PhantomData))
    }

    /// Returns the raw entry point of the API.
    ///
    /// # Safety
    ///
    /// Using the entry point structure directly will discard any thread safety provided by
    /// default with this library.
    pub unsafe fn raw_api(&self) -> *mut Entry {
        self.0
    }

    /// Attempts to shut down RenderDoc.
    ///
    /// # Safety
    ///
    /// Note that this will work correctly if done _immediately_ after the dynamic library is
    /// loaded, before any API work happens. At that point, RenderDoc will remove its injected
    /// hooks and shut down. Behavior is undefined if this is called after any API functions have
    /// been called.
    pub unsafe fn shutdown(self) {
        ((*self.0).Shutdown.unwrap())();
    }
}

impl<V: HasPrevious> RenderDoc<V> {
    /// Downgrades the current version of RenderDoc to the immediate previous one.
    pub fn downgrade(self) -> RenderDoc<V::Previous> {
        let RenderDoc(entry, _) = self;
        RenderDoc(entry, PhantomData)
    }
}

impl<V: HasPrevious> Deref for RenderDoc<V> {
    type Target = RenderDoc<V::Previous>;

    fn deref(&self) -> &Self::Target {
        // NOTE: This transmutation is actually safe because the underlying entry point exposed by
        // the RenderDoc API is the exact same structure. This call only serves to recursively
        // expose the methods in a statically guaranteed and backwards-compatible way.
        unsafe { &*(self as *const RenderDoc<V> as *const RenderDoc<<V as HasPrevious>::Previous>) }
    }
}

impl<V: HasPrevious> DerefMut for RenderDoc<V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // NOTE: This transmutation is actually safe because the underlying entry point exposed by
        // the RenderDoc API is the exact same structure. This call only serves to recursively
        // expose the methods in a statically guaranteed and backwards-compatible way.
        unsafe { &mut *(self as *mut RenderDoc<V> as *mut RenderDoc<<V as HasPrevious>::Previous>) }
    }
}

impl<V: Version> Debug for RenderDoc<V> {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        fmt.debug_tuple(stringify!(RenderDoc))
            .field(&self.0)
            .field(&V::VERSION)
            .finish()
    }
}

impl RenderDoc<V100> {
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
    pub fn get_api_version(&self) -> (u32, u32, u32) {
        unsafe {
            let (mut major, mut minor, mut patch) = (0, 0, 0);
            ((*self.0).GetAPIVersion.unwrap())(&mut major, &mut minor, &mut patch);
            (major as u32, minor as u32, patch as u32)
        }
    }

    /// Sets the specified `CaptureOption` to the given `f32` value.
    ///
    /// # Panics
    ///
    /// This method will panic if the option and/or the value are invalid.
    pub fn set_capture_option_f32(&mut self, opt: CaptureOption, val: f32) {
        let err = unsafe { ((*self.0).SetCaptureOptionF32.unwrap())(opt as u32, val) };
        assert_eq!(err, 1);
    }

    /// Sets the specified `CaptureOption` to the given `u32` value.
    ///
    /// # Panics
    ///
    /// This method will panic if the option and/or the value are invalid.
    pub fn set_capture_option_u32(&mut self, opt: CaptureOption, val: u32) {
        let err = unsafe { ((*self.0).SetCaptureOptionU32.unwrap())(opt as u32, val) };
        assert_eq!(err, 1);
    }

    /// Returns the value of the given `CaptureOption` as an `f32` value.
    ///
    /// # Panics
    ///
    /// This method will panic if the option is invalid.
    pub fn get_capture_option_f32(&self, opt: CaptureOption) -> f32 {
        use std::f32::MAX;
        let val = unsafe { ((*self.0).GetCaptureOptionF32.unwrap())(opt as u32) };
        assert!(val.approx_ne(&-MAX, std::f32::EPSILON, 2));
        val
    }

    /// Returns the value of the given `CaptureOption` as a `u32` value.
    ///
    /// # Panics
    ///
    /// This method will panic if the option is invalid.
    pub fn get_capture_option_u32(&self, opt: CaptureOption) -> u32 {
        use std::u32::MAX;
        let val = unsafe { ((*self.0).GetCaptureOptionU32.unwrap())(opt as u32) };
        assert_ne!(val, MAX);
        val
    }

    #[allow(missing_docs)]
    pub fn set_capture_keys<I: Into<InputButton> + Clone>(&mut self, keys: &[I]) {
        unsafe {
            let mut k: Vec<_> = keys.iter().cloned().map(|k| k.into() as u32).collect();
            ((*self.0).SetCaptureKeys.unwrap())(k.as_mut_ptr(), k.len() as i32)
        }
    }

    #[allow(missing_docs)]
    pub fn set_focus_toggle_keys<I: Into<InputButton> + Clone>(&mut self, keys: &[I]) {
        unsafe {
            let mut k: Vec<_> = keys.iter().cloned().map(|k| k.into() as u32).collect();
            ((*self.0).SetFocusToggleKeys.unwrap())(k.as_mut_ptr(), k.len() as i32)
        }
    }

    #[allow(missing_docs)]
    pub fn unload_crash_handler(&mut self) {
        unsafe {
            ((*self.0).UnloadCrashHandler.unwrap())();
        }
    }

    #[allow(missing_docs)]
    pub fn get_overlay_bits(&self) -> OverlayBits {
        let bits = unsafe { ((*self.0).GetOverlayBits.unwrap())() };
        OverlayBits::from_bits_truncate(bits)
    }

    #[allow(missing_docs)]
    pub fn mask_overlay_bits(&mut self, and: OverlayBits, or: OverlayBits) {
        unsafe {
            ((*self.0).MaskOverlayBits.unwrap())(and.bits(), or.bits());
        }
    }

    #[allow(missing_docs)]
    pub fn get_log_file_path_template(&self) -> &str {
        unsafe {
            let raw = ((*self.0).__bindgen_anon_2.GetLogFilePathTemplate.unwrap())();
            CStr::from_ptr(raw).to_str().unwrap()
        }
    }

    #[allow(missing_docs)]
    pub fn set_log_file_path_template<P: AsRef<Path>>(&mut self, path_template: P) {
        unsafe {
            let utf8 = path_template.as_ref().to_str();
            let path = utf8.and_then(|s| CString::new(s).ok()).unwrap();
            ((*self.0).__bindgen_anon_1.SetLogFilePathTemplate.unwrap())(path.as_ptr());
        }
    }

    #[allow(missing_docs)]
    pub fn get_num_captures(&self) -> u32 {
        unsafe { ((*self.0).GetNumCaptures.unwrap())() }
    }

    #[allow(missing_docs)]
    pub fn get_capture(&self, index: u32) -> Option<(String, u64)> {
        unsafe {
            let mut len = self.get_log_file_path_template().len() as u32 + 128;
            let mut path = Vec::with_capacity(len as usize);
            let mut time = 0u64;

            if ((*self.0).GetCapture.unwrap())(index, path.as_mut_ptr(), &mut len, &mut time) == 1 {
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
    pub fn trigger_capture(&mut self) {
        unsafe {
            ((*self.0).TriggerCapture.unwrap())();
        }
    }

    #[allow(missing_docs)]
    pub fn is_remote_access_connected(&self) -> bool {
        unsafe { ((*self.0).__bindgen_anon_3.IsRemoteAccessConnected.unwrap())() == 1 }
    }

    #[allow(missing_docs)]
    pub fn launch_replay_ui<'a, O>(
        &self,
        connect_immediately: bool,
        extra_opts: O,
    ) -> Result<u32, ()>
    where
        O: Into<Option<&'a str>>,
    {
        let should_connect = connect_immediately as u32;
        let utf8 = extra_opts.into().and_then(|s| CString::new(s).ok());
        let extra_opts = utf8.as_ref().map(|s| s.as_ptr()).unwrap_or_else(ptr::null);

        unsafe {
            match ((*self.0).LaunchReplayUI.unwrap())(should_connect, extra_opts) {
                0 => Err(()),
                pid => Ok(pid),
            }
        }
    }

    #[allow(missing_docs)]
    pub fn set_active_window<D>(&mut self, dev: D, win: WindowHandle)
    where
        D: Into<DevicePointer>,
    {
        unsafe {
            let DevicePointer(dev) = dev.into();
            ((*self.0).SetActiveWindow.unwrap())(dev as *mut _, win as *mut _);
        }
    }

    #[allow(missing_docs)]
    pub fn start_frame_capture<D>(&mut self, dev: D, win: WindowHandle)
    where
        D: Into<DevicePointer>,
    {
        unsafe {
            let DevicePointer(dev) = dev.into();
            ((*self.0).StartFrameCapture.unwrap())(dev as *mut _, win as *mut _);
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
    pub fn is_frame_capturing(&self) -> bool {
        unsafe { ((*self.0).IsFrameCapturing.unwrap())() == 1 }
    }

    #[allow(missing_docs)]
    pub fn end_frame_capture<D>(&mut self, dev: D, win: WindowHandle)
    where
        D: Into<DevicePointer>,
    {
        unsafe {
            let DevicePointer(dev) = dev.into();
            ((*self.0).EndFrameCapture.unwrap())(dev as *mut _, win as *mut _);
        }
    }
}

impl RenderDoc<V110> {
    /// Captures the next _n_ frames from the currently active window and API device.
    ///
    /// Data is saved to a capture log file at the location specified via
    /// `set_log_file_path_template()`.
    pub fn trigger_multi_frame_capture(&mut self, num_frames: u32) {
        unsafe {
            ((*self.0).TriggerMultiFrameCapture.unwrap())(num_frames);
        }
    }
}

impl RenderDoc<V111> {
    #[allow(missing_docs)]
    #[deprecated(since = "1.1.1", note = "renamed to `is_target_control_connected()`")]
    pub fn is_remote_access_connected(&self) -> bool {
        let v1: &RenderDoc<V100> = self.deref();
        v1.is_remote_access_connected()
    }

    #[allow(missing_docs)]
    pub fn is_target_control_connected(&self) -> bool {
        unsafe { ((*self.0).__bindgen_anon_3.IsTargetControlConnected.unwrap())() == 1 }
    }
}

impl RenderDoc<V112> {
    #[allow(missing_docs)]
    pub fn get_capture_file_path_template(&self) -> &str {
        unsafe {
            let raw = ((*self.0)
                .__bindgen_anon_2
                .GetCaptureFilePathTemplate
                .unwrap())();
            CStr::from_ptr(raw).to_str().unwrap()
        }
    }

    #[allow(missing_docs)]
    pub fn set_capture_file_path_template<P: AsRef<Path>>(&mut self, path_template: P) {
        let utf8 = path_template.as_ref().to_str();
        let cstr = utf8.and_then(|s| CString::new(s).ok()).unwrap();
        unsafe {
            ((*self.0)
                .__bindgen_anon_1
                .SetCaptureFilePathTemplate
                .unwrap())(cstr.as_ptr());
        }
    }
}

impl RenderDoc<V120> {
    #[allow(missing_docs)]
    pub fn set_capture_file_comments<'a, P, C>(&mut self, path: P, comments: C)
    where
        P: Into<Option<&'a str>>,
        C: AsRef<str>,
    {
        let utf8 = path.into().and_then(|s| CString::new(s).ok());
        let path = utf8.as_ref().map(|s| s.as_ptr()).unwrap_or_else(ptr::null);

        let comments = CString::new(comments.as_ref()).expect("string contains extra null bytes");

        unsafe {
            ((*self.0).SetCaptureFileComments.unwrap())(path, comments.as_ptr());
        }
    }
}

impl RenderDoc<V140> {
    /// Ends capturing immediately and discard any data without saving to disk.
    ///
    /// Returns `true` if the capture was discarded, or `false` if no capture is in progress.
    pub fn discard_frame_capture<D>(&mut self, dev: D, win: WindowHandle) -> bool
    where
        D: Into<DevicePointer>,
    {
        let DevicePointer(dev) = dev.into();
        unsafe { ((*self.0).DiscardFrameCapture.unwrap())(dev as *mut _, win as *mut _) == 1 }
    }
}
