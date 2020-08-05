//! Type-safe wrapper around the RenderDoc API.

use std::ffi::{CStr, CString};
use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};
use std::{ptr, time};

use float_cmp::approx_eq;

use crate::error::Error;
use crate::handles::{DevicePointer, WindowHandle};
use crate::settings::{CaptureOption, InputButton, OverlayBits};
use crate::version::{Entry, HasPrevious, Version, V100, V110, V111, V112, V120, V130, V140, V141};

/// An instance of the RenderDoc API with baseline version `V`.
#[repr(C)]
#[derive(Eq, Hash, PartialEq)]
pub struct RenderDoc<V>(*mut Entry, PhantomData<V>);

impl<V: Version> RenderDoc<V> {
    /// Initializes a new instance of the RenderDoc API.
    pub fn new() -> Result<Self, Error> {
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
    ///
    /// # Compatibility
    ///
    /// This process is only possible on Windows, and even then it is not well defined so may not
    /// be possible in all circumstances. This function is provided at your own risk.
    // FIXME: Need to move this to `RenderDoc<V100>` and add `remove_hooks()` to `RenderDoc<V141>`.
    // This is currently impossible to do until https://github.com/rust-lang/rfcs/issues/997 is
    // resolved, since `Deref` nor `DerefMut` is sufficient for the task.
    pub unsafe fn shutdown(self) {
        ((*self.0).__bindgen_anon_1.Shutdown.unwrap())();
    }
}

impl<V: HasPrevious> RenderDoc<V> {
    /// Downgrades the current API version to the version immediately preceding it.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use renderdoc::{Error, RenderDoc, V100, V111, V112};
    /// # fn main() -> Result<(), Error> {
    /// let current: RenderDoc<V112> = RenderDoc::new()?;
    /// let previous: RenderDoc<V111> = current.downgrade();
    /// // let older: RenderDoc<V100> = previous.downgrade(); // This line does not compile
    /// # Ok(())
    /// # }
    /// ```
    pub fn downgrade(self) -> RenderDoc<V::Previous> {
        let RenderDoc(entry, _) = self;
        RenderDoc(entry, PhantomData)
    }
}

#[doc(hidden)]
impl<V: HasPrevious> Deref for RenderDoc<V> {
    type Target = RenderDoc<V::Previous>;

    fn deref(&self) -> &Self::Target {
        // NOTE: This transmutation is safe because the entry point type and layout does not change
        // between API versions. This call only serves as type-level magic to expose the inherent
        // methods in a backwards-compatible way.
        unsafe { &*(self as *const RenderDoc<V> as *const RenderDoc<<V as HasPrevious>::Previous>) }
    }
}

impl<V: HasPrevious> DerefMut for RenderDoc<V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // NOTE: This transmutation is safe because the entry point type and layout does not change
        // between API versions. This call only serves as type-level magic to expose the inherent
        // methods in a backwards-compatible way.
        unsafe { &mut *(self as *mut RenderDoc<V> as *mut RenderDoc<<V as HasPrevious>::Previous>) }
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
    /// # use renderdoc::{Error, RenderDoc, V100};
    /// # fn main() -> Result<(), Error> {
    /// let renderdoc: RenderDoc<V100> = RenderDoc::new()?;
    /// let (major, minor, patch) = renderdoc.get_api_version();
    /// assert_eq!(major, 1);
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
        assert!(!approx_eq!(f32, val, -MAX));
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

    /// Sets the key bindings used in-application to trigger a capture on the current window.
    ///
    /// If the `keys` slice is empty, all existing capture key bindings are disabled.
    pub fn set_capture_keys<I: Into<InputButton> + Clone>(&mut self, keys: &[I]) {
        unsafe {
            let mut k: Vec<_> = keys.iter().cloned().map(|k| k.into() as u32).collect();
            ((*self.0).SetCaptureKeys.unwrap())(k.as_mut_ptr(), k.len() as i32)
        }
    }

    /// Sets the key bindings used in-application to switch focus between windows.
    ///
    /// If the `keys` slice is empty, all existing focus toggle key bindings are disabled.
    pub fn set_focus_toggle_keys<I: Into<InputButton> + Clone>(&mut self, keys: &[I]) {
        unsafe {
            let mut k: Vec<_> = keys.iter().cloned().map(|k| k.into() as u32).collect();
            ((*self.0).SetFocusToggleKeys.unwrap())(k.as_mut_ptr(), k.len() as i32)
        }
    }

    /// Removes RenderDoc's injected crash handler from the current process.
    ///
    /// This allows you to provide your own global crash handler with which to handle exceptions,
    /// if you so desire. After the crash handler has been removed, subsequent calls to this method
    /// will do nothing.
    pub fn unload_crash_handler(&mut self) {
        unsafe {
            ((*self.0).UnloadCrashHandler.unwrap())();
        }
    }

    /// Returns a bitmask representing which elements of the RenderDoc overlay are being rendered
    /// on each window.
    pub fn get_overlay_bits(&self) -> OverlayBits {
        let bits = unsafe { ((*self.0).GetOverlayBits.unwrap())() };
        OverlayBits::from_bits_truncate(bits)
    }

    /// Applies the given `and` and `or` bitflags to determine which elements of the RenderDoc
    /// overlay should be rendered on each window.
    ///
    /// This method applies `and` to the current mask with a bitwise-and, and then applies `or`
    /// using a bitwise-or on top.
    pub fn mask_overlay_bits(&mut self, and: OverlayBits, or: OverlayBits) {
        unsafe {
            ((*self.0).MaskOverlayBits.unwrap())(and.bits(), or.bits());
        }
    }

    /// Returns the path template where new captures will be stored.
    ///
    /// The template can either be a relative or absolute path, which determines where captures
    /// will be saved and how they will be named. Relative paths will be saved relative to the
    /// process' current working directory.
    ///
    /// By default, this will be in a folder controlled by the UI - initially the system temporary
    /// directory, and the filename is the executable's filename.
    ///
    /// # Examples
    ///
    /// ```
    /// # use renderdoc::{Error, RenderDoc, V100};
    /// # fn main() -> Result<(), Error> {
    /// let renderdoc: RenderDoc<V100> = RenderDoc::new()?;
    /// println!("{:?}", renderdoc.get_log_file_path_template()); // e.g. `my_captures/example`
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_log_file_path_template(&self) -> &Path {
        unsafe {
            let raw = ((*self.0).__bindgen_anon_3.GetLogFilePathTemplate.unwrap())();
            CStr::from_ptr(raw).to_str().map(Path::new).unwrap()
        }
    }

    /// Sets the path template where new capture files should be stored.
    ///
    /// The template can either be a relative or absolute path, which determines where captures
    /// will be saved and how they will be named. Relative paths will be saved relative to the
    /// process' current working directory.
    ///
    /// The default template is in a folder controlled by the UI - initially the system temporary
    /// directory, and the filename is the executable's filename.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use renderdoc::{Error, RenderDoc, V100};
    /// # fn main() -> Result<(), Error> {
    /// let mut renderdoc: RenderDoc<V100> = RenderDoc::new()?;
    /// renderdoc.set_log_file_path_template("my_captures/example");
    ///
    /// renderdoc.trigger_capture(); // Saved as `my_captures/example_frame123.rdc`
    /// renderdoc.trigger_capture(); // Saved as `my_captures/example_frame456.rdc`
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_log_file_path_template<P: Into<PathBuf>>(&mut self, path_template: P) {
        unsafe {
            let utf8 = path_template.into().into_os_string().into_string().ok();
            let path = utf8.and_then(|s| CString::new(s).ok()).unwrap();
            ((*self.0).__bindgen_anon_2.SetLogFilePathTemplate.unwrap())(path.as_ptr());
        }
    }

    /// Returns the number of frame captures that have been made.
    ///
    /// # Examples
    ///
    /// ```
    /// # use renderdoc::{Error, RenderDoc, V100};
    /// # fn main() -> Result<(), Error> {
    /// let renderdoc: RenderDoc<V100> = RenderDoc::new()?;
    /// assert_eq!(renderdoc.get_num_captures(), 0);
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_num_captures(&self) -> u32 {
        unsafe { ((*self.0).GetNumCaptures.unwrap())() }
    }

    /// Retrieves the path and capture time of a capture file indexed by the number `index`.
    ///
    /// Returns `Some` if the index was within `0..get_num_captures()`, otherwise returns `None`.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use renderdoc::{Error, RenderDoc, V100};
    /// # fn main() -> Result<(), Error> {
    /// let mut renderdoc: RenderDoc<V100> = RenderDoc::new()?;
    ///
    /// // Capture a frame.
    /// renderdoc.trigger_capture();
    ///
    /// // Get information about the previous capture.
    /// let (file_path, capture_time) = renderdoc.get_capture(0).unwrap();
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_capture(&self, index: u32) -> Option<(PathBuf, SystemTime)> {
        let mut len = self.get_log_file_path_template().as_os_str().len() as u32 + 128;
        let mut path = Vec::with_capacity(len as usize);
        let mut time = 0u64;

        unsafe {
            if ((*self.0).GetCapture.unwrap())(index, path.as_mut_ptr(), &mut len, &mut time) == 1 {
                let capture_time = time::UNIX_EPOCH + Duration::from_secs(time);
                let path = {
                    let raw_path = CString::from_raw(path.as_mut_ptr());
                    let mut path = raw_path.into_string().unwrap();
                    path.shrink_to_fit();
                    path
                };

                Some((path.into(), capture_time))
            } else {
                None
            }
        }
    }

    /// Captures the next frame from the currently active window and API device.
    ///
    /// Data is saved to a capture file at the location specified by
    /// `set_log_file_path_template()`.
    ///
    /// ```rust,no_run
    /// # use renderdoc::{Error, RenderDoc, V100};
    /// # fn main() -> Result<(), Error> {
    /// let mut renderdoc: RenderDoc<V100> = RenderDoc::new()?;
    ///
    /// // Capture the current frame and save to a file.
    /// renderdoc.trigger_capture();
    /// # Ok(())
    /// # }
    /// ```
    pub fn trigger_capture(&mut self) {
        unsafe {
            ((*self.0).TriggerCapture.unwrap())();
        }
    }

    /// Returns whether the RenderDoc UI is connected to this application.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use renderdoc::{Error, RenderDoc, V100};
    /// # fn main() -> Result<(), Error> {
    /// let renderdoc: RenderDoc<V100> = RenderDoc::new()?;
    /// assert!(!renderdoc.is_remote_access_connected());
    /// # Ok(())
    /// # }
    /// ```
    pub fn is_remote_access_connected(&self) -> bool {
        unsafe { ((*self.0).__bindgen_anon_4.IsRemoteAccessConnected.unwrap())() == 1 }
    }

    /// Launches the replay UI associated with the RenderDoc library injected into the running
    /// application.
    ///
    /// If `connect_immediately` is `true`, the replay window will automatically connect to this
    /// application once opened, ready to capture frames right away. Optional command-line
    /// arguments to the RenderDoc replay UI can be specified via the `extra_opts` parameter.
    ///
    /// Returns the PID of the RenderDoc replay process on success.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use renderdoc::{Error, RenderDoc, V100};
    /// # fn main() -> Result<(), Error> {
    /// let renderdoc: RenderDoc<V100> = RenderDoc::new()?;
    /// let pid = renderdoc.launch_replay_ui(true, None)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn launch_replay_ui<'a, O>(
        &self,
        connect_immediately: bool,
        extra_opts: O,
    ) -> Result<u32, Error>
    where
        O: Into<Option<&'a str>>,
    {
        let should_connect = connect_immediately as u32;
        let utf8 = extra_opts.into().and_then(|s| CString::new(s).ok());
        let extra_opts = utf8.as_ref().map(|s| s.as_ptr()).unwrap_or_else(ptr::null);

        unsafe {
            match ((*self.0).LaunchReplayUI.unwrap())(should_connect, extra_opts) {
                0 => Err(Error::launch_replay_ui()),
                pid => Ok(pid),
            }
        }
    }

    /// Explicitly set which window is considered "active" by RenderDoc.
    ///
    /// The active window is the one that will be captured when the keybinding to trigger a capture
    /// is pressed by the user, as well as when `trigger_capture()` is called.
    ///
    /// Both `dev` and `win` must be valid handles.
    pub fn set_active_window<D>(&mut self, dev: D, win: WindowHandle)
    where
        D: Into<DevicePointer>,
    {
        unsafe {
            let DevicePointer(dev) = dev.into();
            ((*self.0).SetActiveWindow.unwrap())(dev as *mut _, win as *mut _);
        }
    }

    /// Begins a frame capture for the specified device/window combination.
    ///
    /// If either or both `dev` and `win` are set to `std::ptr::null()`, then RenderDoc will
    /// perform a wildcard match.
    ///
    /// For example, you can specify `null(), null()` for the device to capture on if you have only
    /// one device and only one or zero windows, and RenderDoc will capture from that device.
    ///
    /// This function must be paired with a matching `start_frame_capture()` to succeed.
    ///
    /// ```rust,no_run
    /// # use renderdoc::{Error, RenderDoc, V100};
    /// # fn main() -> Result<(), Error> {
    /// let mut renderdoc: RenderDoc<V100> = RenderDoc::new()?;
    /// renderdoc.start_frame_capture(std::ptr::null(), std::ptr::null());
    ///
    /// // Capturing window from here onward...
    /// # Ok(())
    /// # }
    /// ```
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
    /// ```rust,no_run
    /// # use renderdoc::{Error, RenderDoc, V100};
    /// # fn main() -> Result<(), Error> {
    /// let renderdoc: RenderDoc<V100> = RenderDoc::new()?;
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

    /// Ends a frame capture for the specified device/window combination, saving results to disk.
    ///
    /// If either or both `dev` and `win` are set to `std::ptr::null()`, then RenderDoc will
    /// perform a wildcard match.
    ///
    /// For example, you can specify `null(), null()` for the device to capture on if you have only
    /// one device and only one or zero windows, and RenderDoc will capture from that device.
    ///
    /// This function must be paired with a matching `end_frame_capture()` to complete.
    ///
    /// ```rust,no_run
    /// # use renderdoc::{Error, RenderDoc, V100};
    /// # fn main() -> Result<(), Error> {
    /// let mut renderdoc: RenderDoc<V100> = RenderDoc::new()?;
    ///
    /// renderdoc.start_frame_capture(std::ptr::null(), std::ptr::null());
    /// // Do some rendering here...
    /// renderdoc.end_frame_capture(std::ptr::null(), std::ptr::null());
    /// # Ok(())
    /// # }
    /// ```
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
    /// Data is saved to _n_ separate capture files at the location specified via
    /// `set_log_file_path_template()`.
    pub fn trigger_multi_frame_capture(&mut self, num_frames: u32) {
        unsafe {
            ((*self.0).TriggerMultiFrameCapture.unwrap())(num_frames);
        }
    }
}

impl RenderDoc<V111> {
    /// Returns whether the RenderDoc UI is connected to this application.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use renderdoc::{Error, RenderDoc, V111};
    /// # fn main() -> Result<(), Error> {
    /// let renderdoc: RenderDoc<V111> = RenderDoc::new()?;
    /// assert!(!renderdoc.is_target_control_connected());
    /// # Ok(())
    /// # }
    /// ```
    pub fn is_target_control_connected(&self) -> bool {
        unsafe { ((*self.0).__bindgen_anon_4.IsTargetControlConnected.unwrap())() == 1 }
    }

    /// Returns whether the RenderDoc UI is connected to this application.
    #[deprecated(since = "1.1.1", note = "renamed to `is_target_control_connected`")]
    pub fn is_remote_access_connected(&self) -> bool {
        let v1: &RenderDoc<V100> = self.deref();
        v1.is_remote_access_connected()
    }
}

impl RenderDoc<V112> {
    /// Returns the path template where new captures will be stored.
    ///
    /// The template can either be a relative or absolute path, which determines where captures
    /// will be saved and how they will be named. Relative paths will be saved relative to the
    /// process' current working directory.
    ///
    /// By default, this will be in a folder controlled by the UI - initially the system temporary
    /// directory, and the filename is the executable's filename.
    ///
    /// # Examples
    ///
    /// ```
    /// # use renderdoc::{Error, RenderDoc, V112};
    /// # fn main() -> Result<(), Error> {
    /// let renderdoc: RenderDoc<V112> = RenderDoc::new()?;
    /// println!("{:?}", renderdoc.get_capture_file_path_template()); // e.g. `my_captures/example`
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_capture_file_path_template(&self) -> &Path {
        unsafe {
            let raw = ((*self.0)
                .__bindgen_anon_3
                .GetCaptureFilePathTemplate
                .unwrap())();
            CStr::from_ptr(raw).to_str().map(Path::new).unwrap()
        }
    }

    /// Sets the path template where new capture files should be stored.
    ///
    /// The template can either be a relative or absolute path, which determines where captures
    /// will be saved and how they will be named. Relative paths will be saved relative to the
    /// process' current working directory.
    ///
    /// The default template is in a folder controlled by the UI - initially the system temporary
    /// directory, and the filename is the executable's filename.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use renderdoc::{Error, RenderDoc, V112};
    /// # fn main() -> Result<(), Error> {
    /// let mut renderdoc: RenderDoc<V112> = RenderDoc::new()?;
    /// renderdoc.set_capture_file_path_template("my_captures/example");
    ///
    /// renderdoc.trigger_capture(); // Saved as `my_captures/example_frame123.rdc`
    /// renderdoc.trigger_capture(); // Saved as `my_captures/example_frame456.rdc`
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_capture_file_path_template<P: Into<PathBuf>>(&mut self, path_template: P) {
        let utf8 = path_template.into().into_os_string().into_string().ok();
        let cstr = utf8.and_then(|s| CString::new(s).ok()).unwrap();
        unsafe {
            ((*self.0)
                .__bindgen_anon_2
                .SetCaptureFilePathTemplate
                .unwrap())(cstr.as_ptr());
        }
    }

    /// Returns the path template where new captures will be stored.
    #[deprecated(since = "1.1.2", note = "renamed to `get_capture_file_path_template`")]
    pub fn get_log_file_path_template(&self) -> &Path {
        let v1: &RenderDoc<V100> = self.deref();
        v1.get_log_file_path_template()
    }

    /// Sets the path template where new capture files should be stored.
    #[deprecated(since = "1.1.2", note = "renamed to `set_capture_file_path_template`")]
    pub fn set_log_file_path_template<P: Into<PathBuf>>(&mut self, path_template: P) {
        let v1: &mut RenderDoc<V100> = self.deref_mut();
        v1.set_log_file_path_template(path_template)
    }
}

impl RenderDoc<V120> {
    /// Adds or sets an arbitrary comments field to an existing capture on disk, which will then be
    /// displayed in the UI to anyone opening the capture file.
    ///
    /// If the `path` argument is `None`, the most recent previous capture file is used.
    pub fn set_capture_file_comments<'a, P, C>(&mut self, path: P, comments: C)
    where
        P: Into<Option<&'a str>>,
        C: Into<String>,
    {
        let utf8 = path.into().and_then(|s| CString::new(s).ok());
        let path = utf8.as_ref().map(|s| s.as_ptr()).unwrap_or_else(ptr::null);
        let comments = CString::new(comments.into()).unwrap();

        unsafe {
            ((*self.0).SetCaptureFileComments.unwrap())(path, comments.as_ptr());
        }
    }
}

impl RenderDoc<V140> {
    /// Ends capturing immediately and discards any data without saving to disk.
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

impl<V: Version> Debug for RenderDoc<V> {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        fmt.debug_tuple(stringify!(RenderDoc))
            .field(&self.0)
            .field(&V::VERSION)
            .finish()
    }
}

unsafe impl<V> Send for RenderDoc<V> {}

/// Generates `From` implementations that permit downgrading of API versions.
///
/// Unlike the `downgrade()` method, these `From` implementations let any version to downgrade to
/// any other older backwards-compatible API version in a clean way.
///
/// This function takes a list of API versions sorted in descending order and recursively generates
/// `From` implementations for them. For instance, given the following three API versions
/// `[V200, V110, V100]`, these trait implementations will be generated:
///
/// ```rust,ignore
/// // V200 -> V110, V100
///
/// impl From<#name<V200>> for #name<V110>
/// where
///     Self: Sized,
/// {
///     fn from(newer: #name<V200>) -> Self {
///         // ...
///     }
/// }
///
/// impl From<#name<V200>> for #name<V100>
/// where
///     Self: Sized,
/// {
///     fn from(newer: #name<V200>) -> Self {
///         // ...
///     }
/// }
///
/// // V110 -> V100
///
/// impl From<#name<V110>> for #name<V100>
/// where
///     Self: Sized,
/// {
///     fn from(newer: #name<V200>) -> Self {
///         // ...
///     }
/// }
///
/// // V100 -> ()
/// ```
macro_rules! impl_from_versions {
    ($base_version:ident) => {};

    ($newer:ident, $($older:ident),+) => {
        $(
            impl From<RenderDoc<$newer>> for RenderDoc<$older>
            where
                Self: Sized,
            {
                fn from(newer: RenderDoc<$newer>) -> Self {
                    let RenderDoc(entry, _) = newer;
                    RenderDoc(entry, PhantomData)
                }
            }
        )+

        impl_from_versions!($($older),+);
    };
}

impl_from_versions!(V141, V140, V130, V120, V112, V111, V110, V100);
