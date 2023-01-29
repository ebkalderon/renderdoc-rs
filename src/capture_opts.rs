use std::fmt::{self, Debug, Formatter};
use std::marker::PhantomData;
use std::time::Duration;

use renderdoc_sys::RENDERDOC_CaptureOption;

use crate::{Below, DebugVersion, Error, Minimum, RawRenderDoc, Version, V100, V102, V110, V130};

/// A possible state of the "capture callstacks" option.
#[derive(Clone, Copy, Debug)]
pub enum CaptureCallstacksOption {
    /// Do not capture a CPU callstack on every API call.
    Disabled,
    /// Capture a CPU callstack on every API call.
    Enabled,
    /// Capture a CPU callstack only on "action" calls (draws, dispatches, clears, copies, etc).
    ///
    /// # Compatibility
    ///
    /// Prior to version 1.4.2, this setting captured callstacks only on draw calls.
    OnlyActions,
}

/// Configures how RenderDoc behaves on capture.
///
/// This struct is created by the [`set_capture_options`] method on [`RenderDoc<V>`].
///
/// [`set_capture_options`]: crate::RenderDoc::set_capture_options
/// [`RenderDoc<V>`]: crate::RenderDoc
pub struct SetCaptureOptions<'api, V> {
    pub(super) api: *mut RawRenderDoc,
    pub(super) _min_version: PhantomData<&'api mut V>,
}

impl<V> SetCaptureOptions<'_, V> {
    fn set_u32(&mut self, opt: RENDERDOC_CaptureOption, val: u32) {
        self.try_set_u32(opt, val).expect("SetCaptureOptions bug");
    }

    #[inline]
    fn try_set_u32(&mut self, opt: RENDERDOC_CaptureOption, val: u32) -> Result<(), Error> {
        let is_valid = unsafe { (self.api.SetCaptureOptionU32.unwrap())(opt, val) };
        if is_valid == 1 {
            Ok(())
        } else {
            Err(Error::set_capture_options(opt, val))
        }
    }
}

impl<V: Minimum<V100>> SetCaptureOptions<'_, V> {
    /// Allow the application to enable vertical synchronization at will (default: `true`).
    ///
    /// If this option is set to `false`, the application will be prevented from enabling vsync.
    pub fn allow_vsync(&mut self, allowed: bool) -> &mut Self {
        use renderdoc_sys::eRENDERDOC_Option_AllowVSync;
        self.set_u32(eRENDERDOC_Option_AllowVSync, allowed as u32);
        self
    }

    /// Allow the application to enable fullscreen mode at will (default: `true`).
    ///
    /// If this option is set to `false`, the application will be prevented from entering
    /// fullscreen mode.
    pub fn allow_fullscreen(&mut self, allowed: bool) -> &mut Self {
        use renderdoc_sys::eRENDERDOC_Option_AllowFullscreen;
        self.set_u32(eRENDERDOC_Option_AllowFullscreen, allowed as u32);
        self
    }

    /// Capture a CPU callstack on every API call (default: [`Disabled`]).
    ///
    /// See [`CaptureCallstacksOption`] documentation for details.
    ///
    /// [`Disabled`]: CaptureCallstacksOption::Disabled
    pub fn capture_callstacks(&mut self, state: CaptureCallstacksOption) -> &mut Self {
        use renderdoc_sys::{
            eRENDERDOC_Option_CaptureCallstacks, eRENDERDOC_Option_CaptureCallstacksOnlyDraws,
        };

        match state {
            CaptureCallstacksOption::Disabled => {
                self.set_u32(eRENDERDOC_Option_CaptureCallstacks, 0);
            }
            CaptureCallstacksOption::Enabled => {
                self.set_u32(eRENDERDOC_Option_CaptureCallstacks, 1);
                self.set_u32(eRENDERDOC_Option_CaptureCallstacksOnlyDraws, 0);
            }
            CaptureCallstacksOption::OnlyActions => {
                self.set_u32(eRENDERDOC_Option_CaptureCallstacks, 1);
                self.set_u32(eRENDERDOC_Option_CaptureCallstacksOnlyDraws, 1);
            }
        }

        self
    }

    /// Pause for `delay` seconds after launching a process to allow debuggers to attach (default:
    /// no delay).
    ///
    /// This only applies to child processes since the delay happens at process startup.
    pub fn delay_for_debugger(&mut self, delay: Duration) -> &mut Self {
        use renderdoc_sys::eRENDERDOC_Option_DelayForDebugger;
        // Q. Why not use `SetCaptureOptionF32` here to support sub-second precision?
        // A. Checking the RenderDoc source, it appears to only respect whole second values anyway.
        //    Might as well use `SetCaptureOptionU32` instead.
        self.set_u32(eRENDERDOC_Option_DelayForDebugger, delay.as_secs() as u32);
        self
    }

    /// Sets whether any child processes launched by the initial application should be hooked by
    /// RenderDoc as well (defaults to `false`).
    ///
    /// The child processes will inherit the same RenderDoc capture options as the parent. This
    /// setting is commonly used in cases where a launcher process is necessary to start the
    /// application.
    pub fn hook_into_children(&mut self, enabled: bool) -> &mut Self {
        use renderdoc_sys::eRENDERDOC_Option_HookIntoChildren;
        self.set_u32(eRENDERDOC_Option_HookIntoChildren, enabled as u32);
        self
    }

    /// Include _all_ live resources at the time of capture in the capture, even those that are not
    /// referenced by the frame (defaults to `false`).
    ///
    /// By default, RenderDoc only includes the resources necessary for that specific frame in the
    /// final capture. Enabling this option overrides this behavior so all live resources are
    /// available for inspection.
    pub fn ref_all_resources(&mut self, enabled: bool) -> &mut Self {
        use renderdoc_sys::eRENDERDOC_Option_RefAllResources;
        self.set_u32(eRENDERDOC_Option_RefAllResources, enabled as u32);
        self
    }

    /// Save all deferred command lists, even when idling (default: `false`).
    ///
    /// In APIs that allow for the recording of command lists to be replayed later, RenderDoc may
    /// choose to not capture command lists before a frame capture is triggered to reduce overhead.
    /// This means any command lists recorded once and replayed many times will not be available
    /// and may cause a failure to capture.
    ///
    /// Enabling this option may impose a performance overhead, but it ensures that any command
    /// list still being held by the application will be captured.
    ///
    /// # Compatibility
    ///
    /// With regards to the comment above about overhead: this is only true for APIs where
    /// multithreading is difficult or discouraged. Newer APIs like Vulkan and D3D12 will ignore
    /// this option and always capture all command lists. Such APIs were designed with
    /// multithreading in mind and overheads are low by design.
    pub fn capture_all_command_lists(&mut self, enabled: bool) -> &mut Self {
        use renderdoc_sys::eRENDERDOC_Option_CaptureAllCmdLists;
        self.set_u32(eRENDERDOC_Option_CaptureAllCmdLists, enabled as u32);
        self
    }

    /// Mute API debugging output when the API validation mode option is enabled (default: `true`).
    ///
    /// See documentation of [`debug_device_mode`] (below 1.0.2) or [`api_validation`] (1.0.2 and
    /// newer) for details.
    ///
    /// [`debug_device_mode`]: SetCaptureOptions::debug_device_mode
    /// [`api_validation`]: SetCaptureOptions::api_validation
    pub fn mute_debug_output(&mut self, enabled: bool) -> &mut Self {
        use renderdoc_sys::eRENDERDOC_Option_DebugOutputMute;
        self.set_u32(eRENDERDOC_Option_DebugOutputMute, enabled as u32);
        self
    }
}

impl<V: Minimum<V100> + Below<V102>> SetCaptureOptions<'_, V> {
    /// Initialize the graphics API with built-in validation enabled (default: `false`).
    ///
    /// If enabled, this allows capturing and reading of errors and warnings generated by the API's
    /// own debugging system.
    ///
    /// # Compatibility
    ///
    /// Since version 1.0.2, this capture option has been renamed to [`api_validation`].
    ///
    /// [`api_validation`]: SetCaptureOptions::api_validation
    pub fn debug_device_mode(&mut self, enabled: bool) -> &mut Self {
        use renderdoc_sys::eRENDERDOC_Option_DebugDeviceMode;
        self.set_u32(eRENDERDOC_Option_DebugDeviceMode, enabled as u32);
        self
    }
}

impl<V: Minimum<V100> + Below<V110>> SetCaptureOptions<'_, V> {
    /// Save the initial contents of all resources at the start of each frame, even if they are
    /// later overwritten or cleared before being used (default: `false`).
    ///
    /// By default, RenderDoc skips saving initial states for resources where the previous contents
    /// don't appear to have been used, assuming that a write followed by a read means that the
    /// resource had never been used before (and is therefore treated as empty).
    #[deprecated(
        since = "1.1.0",
        note = "`save_all_initials` is always enabled in version 1.1.0 and newer"
    )]
    pub fn save_all_initials(&mut self, enabled: bool) -> &mut Self {
        use renderdoc_sys::eRENDERDOC_Option_SaveAllInitials;
        self.set_u32(eRENDERDOC_Option_SaveAllInitials, enabled as u32);
        self
    }
}

impl<V: Minimum<V100> + Below<V130>> SetCaptureOptions<'_, V> {
    /// Verify any writes to mapped buffers (default: `false`).
    ///
    /// This option indicates mapped memory updates should be bounds-checked for overruns, and
    /// uninitialized buffers should be initialized to `0xdddddddd` to catch use of uninitialized
    /// data.
    ///
    /// # Compatibility
    ///
    /// Only supported on D3D11 and OpenGL.
    ///
    /// Since version 1.3.0, this capture option has been renamed to [`verify_buffer_access`].
    ///
    /// [`verify_buffer_access`]: SetCaptureOptions::verify_buffer_access
    pub fn verify_map_writes(&mut self, enabled: bool) -> &mut Self {
        use renderdoc_sys::eRENDERDOC_Option_VerifyMapWrites;
        self.set_u32(eRENDERDOC_Option_VerifyMapWrites, enabled as u32);
        self
    }
}

impl<V: Minimum<V102>> SetCaptureOptions<'_, V> {
    /// Initializes the graphics API with built-in validation enabled (default: `false`).
    ///
    /// If enabled, this allows capturing and reading of errors and warnings generated by the API's
    /// own debugging system.
    ///
    /// # Compatibility
    ///
    /// Prior to version 1.0.2, this capture option was named [`debug_device_mode`].
    ///
    /// [`debug_device_mode`]: SetCaptureOptions::debug_device_mode
    pub fn api_validation(&mut self, enabled: bool) -> &mut Self {
        use renderdoc_sys::eRENDERDOC_Option_APIValidation;
        self.set_u32(eRENDERDOC_Option_APIValidation, enabled as u32);
        self
    }
}

impl<V: Minimum<V130>> SetCaptureOptions<'_, V> {
    /// Verify any writes to mapped buffers (default: `false`).
    ///
    /// This option indicates mapped memory updates should be bounds-checked for overruns, and
    /// uninitialized buffers should be initialized to `0xdddddddd` to catch use of uninitialized
    /// data.
    ///
    /// # Compatibility
    ///
    /// Only supported on Direct3D 11 and OpenGL.
    ///
    /// Prior to version 1.3.0, this capture option was named [`verify_map_writes`].
    ///
    /// [`verify_map_writes`]: SetCaptureOptions::verify_map_writes
    pub fn verify_buffer_access(&mut self, enabled: bool) -> &mut Self {
        use renderdoc_sys::eRENDERDOC_Option_VerifyBufferAccess;
        self.set_u32(eRENDERDOC_Option_VerifyBufferAccess, enabled as u32);
        self
    }

    /// Enable unsupported vendor extensions (default: inactive).
    ///
    /// # Safety
    ///
    /// Vendor extensions may be incompatible with the current version of RenderDoc and cause
    /// corrupted replays or crashes. No values are documented, this option should only be used
    /// when absolutely necessary as directed by a RenderDoc developer.
    pub unsafe fn allow_unsupported_vendor_extensions(&mut self, ext: u32) -> Result<(), Error> {
        use renderdoc_sys::eRENDERDOC_Option_AllowUnsupportedVendorExtensions;
        self.try_set_u32(eRENDERDOC_Option_AllowUnsupportedVendorExtensions, ext)
    }
}

impl<V: Version> Debug for SetCaptureOptions<'_, V> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct(stringify!(SetCaptureOptions))
            .field("min_version", &DebugVersion(V::VERSION))
            .finish()
    }
}

/// Gets the values of RenderDoc capture options.
///
/// This struct is created by the [`capture_options`] method on [`RenderDoc<V>`].
///
/// [`capture_options`]: crate::RenderDoc::capture_options
/// [`RenderDoc<V>`]: crate::RenderDoc
pub struct CaptureOptions<'api, V> {
    pub(super) api: *mut RawRenderDoc,
    pub(super) _min_version: PhantomData<&'api V>,
}

impl<V> CaptureOptions<'_, V> {
    fn get_u32(&self, opt: RENDERDOC_CaptureOption) -> u32 {
        self.try_get_u32(opt).expect("CaptureOptions bug")
    }

    #[inline]
    fn try_get_u32(&self, opt: RENDERDOC_CaptureOption) -> Result<u32, Error> {
        let value = unsafe { (self.api.GetCaptureOptionU32.unwrap())(opt) };
        match value {
            u32::MAX => Err(Error::get_capture_options(opt)),
            val => Ok(val),
        }
    }
}

impl<V: Minimum<V100>> CaptureOptions<'_, V> {
    /// Returns whether the application is allowed to enable vertical synchronization at will.
    ///
    /// If this option is set to `false`, the application will be prevented from enabling vsync.
    pub fn allow_vsync(&self) -> bool {
        self.get_u32(renderdoc_sys::eRENDERDOC_Option_AllowVSync) == 1
    }

    /// Returns whether the application is allowed to enter fullscreen mode at will.
    ///
    /// If this option is set to `false`, the application will be prevented from entering
    /// fullscreen mode.
    pub fn allow_fullscreen(&self) -> bool {
        self.get_u32(renderdoc_sys::eRENDERDOC_Option_AllowFullscreen) == 1
    }

    /// Capture a CPU callstack on every API call.
    ///
    /// See [`CaptureCallstacksOption`] documentation for details.
    pub fn capture_callstacks(&self) -> CaptureCallstacksOption {
        if self.get_u32(renderdoc_sys::eRENDERDOC_Option_CaptureCallstacks) == 1 {
            if self.get_u32(renderdoc_sys::eRENDERDOC_Option_CaptureCallstacksOnlyDraws) == 1 {
                CaptureCallstacksOption::OnlyActions
            } else {
                CaptureCallstacksOption::Enabled
            }
        } else {
            CaptureCallstacksOption::Disabled
        }
    }

    /// Returns the `delay` RenderDoc will wait after launching a process to allow debuggers to
    /// attach.
    ///
    /// This only applies to child processes since the delay happens at process startup.
    pub fn delay_for_debugger(&self) -> Duration {
        let secs = self.get_u32(renderdoc_sys::eRENDERDOC_Option_DelayForDebugger);
        Duration::from_secs(secs as u64)
    }

    /// Returns whether any child processes launched by the initial application should be hooked by
    /// RenderDoc as well.
    ///
    /// The child processes will inherit the same RenderDoc capture options as the parent. This
    /// setting is commonly used in cases where a launcher process is necessary to start the
    /// application.
    pub fn hook_into_children(&self) -> bool {
        self.get_u32(renderdoc_sys::eRENDERDOC_Option_HookIntoChildren) == 1
    }

    /// Returns whether _all_ live resources at the time of capture are included in the capture,
    /// even those that are not referenced by the frame.
    ///
    /// By default, RenderDoc only includes the resources necessary for that specific frame in the
    /// final capture. Enabling this option overrides this behavior so all live resources are
    /// available for inspection.
    pub fn ref_all_resources(&self) -> bool {
        self.get_u32(renderdoc_sys::eRENDERDOC_Option_RefAllResources) == 1
    }

    /// Returns whether all deferred command lists are saved, even when idling.
    ///
    /// In APIs that allow for the recording of command lists to be replayed later, RenderDoc may
    /// choose to not capture command lists before a frame capture is triggered to reduce overhead.
    /// This means any command lists recorded once and replayed many times will not be available
    /// and may cause a failure to capture.
    ///
    /// Enabling this option may impose a performance overhead, but it ensures that any command
    /// list still being held by the application will be captured.
    ///
    /// # Compatibility
    ///
    /// With regards to the comment above about overhead: this is only true for APIs where
    /// multithreading is difficult or discouraged. Newer APIs like Vulkan and D3D12 will ignore
    /// this option and always capture all command lists. Such APIs were designed with
    /// multithreading in mind and overheads are low by design.
    pub fn capture_all_command_lists(&self) -> bool {
        self.get_u32(renderdoc_sys::eRENDERDOC_Option_CaptureAllCmdLists) == 1
    }

    /// Returns whether API debugging output is muted when the API validation mode option is
    /// enabled.
    ///
    /// See documentation of [`debug_device_mode`] (below 1.0.2) or [`api_validation`] (1.0.2 and
    /// newer) for details.
    ///
    /// [`debug_device_mode`]: CaptureOptions::debug_device_mode
    /// [`api_validation`]: CaptureOptions::api_validation
    pub fn mute_debug_output(&self) -> bool {
        self.get_u32(renderdoc_sys::eRENDERDOC_Option_DebugOutputMute) == 1
    }
}

impl<V: Minimum<V100> + Below<V102>> CaptureOptions<'_, V> {
    /// Returns whether the graphics API was initialized with built-in validation enabled.
    ///
    /// If enabled, this allows capturing and reading of errors and warnings generated by the API's
    /// own debugging system.
    ///
    /// # Compatibility
    ///
    /// Since version 1.0.2, this capture option has been renamed to [`api_validation`].
    ///
    /// [`api_validation`]: CaptureOptions::api_validation
    pub fn debug_device_mode(&self) -> bool {
        self.get_u32(renderdoc_sys::eRENDERDOC_Option_DebugDeviceMode) == 1
    }
}

impl<V: Minimum<V100> + Below<V110>> CaptureOptions<'_, V> {
    /// Returns whether the initial contents of all resources are saved at the start of each frame,
    /// even if they are later overwritten or cleared before being used.
    ///
    /// By default, RenderDoc skips saving initial states for resources where the previous contents
    /// don't appear to have been used, assuming that a write followed by a read means that the
    /// resource had never been used before (and is therefore treated as empty).
    #[deprecated(
        since = "1.1.0",
        note = "`save_all_initials` is always enabled in version 1.1.0 and newer"
    )]
    pub fn save_all_initials(&self) -> bool {
        self.get_u32(renderdoc_sys::eRENDERDOC_Option_SaveAllInitials) == 1
    }
}

impl<V: Minimum<V100> + Below<V130>> CaptureOptions<'_, V> {
    /// Returns whether RenderDoc will verify writes to mapped buffers.
    ///
    /// This option indicates mapped memory updates should be bounds-checked for overruns, and
    /// uninitialized buffers should be initialized to `0xdddddddd` to catch use of uninitialized
    /// data.
    ///
    /// # Compatibility
    ///
    /// Only supported on D3D11 and OpenGL.
    ///
    /// Since version 1.3.0, this capture option has been renamed to [`verify_buffer_access`].
    ///
    /// [`verify_buffer_access`]: CaptureOptions::verify_buffer_access
    pub fn verify_map_writes(&self) -> bool {
        self.get_u32(renderdoc_sys::eRENDERDOC_Option_VerifyMapWrites) == 1
    }
}

impl<V: Minimum<V102>> CaptureOptions<'_, V> {
    /// Returns whether the graphics API was initialized with built-in validation enabled.
    ///
    /// If enabled, this allows capturing and reading of errors and warnings generated by the API's
    /// own debugging system.
    ///
    /// # Compatibility
    ///
    /// Prior to version 1.0.2, this capture option was named [`debug_device_mode`].
    ///
    /// [`debug_device_mode`]: CaptureOptions::debug_device_mode
    pub fn api_validation(&self) -> bool {
        self.get_u32(renderdoc_sys::eRENDERDOC_Option_APIValidation) == 1
    }
}

impl<V: Minimum<V130>> CaptureOptions<'_, V> {
    /// Returns whether RenderDoc will verify writes to mapped buffers.
    ///
    /// This option indicates mapped memory updates should be bounds-checked for overruns, and
    /// uninitialized buffers should be initialized to `0xdddddddd` to catch use of uninitialized
    /// data.
    ///
    /// # Compatibility
    ///
    /// Only supported on Direct3D 11 and OpenGL.
    ///
    /// Prior to version 1.3.0, this capture option was named [`verify_map_writes`].
    ///
    /// [`verify_map_writes`]: CaptureOptions::verify_map_writes
    pub fn verify_buffer_access(&self) -> bool {
        self.get_u32(renderdoc_sys::eRENDERDOC_Option_VerifyBufferAccess) == 1
    }
}

impl<V: Version> Debug for CaptureOptions<'_, V> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct(stringify!(CaptureOptions))
            .field("min_version", &DebugVersion(V::VERSION))
            .finish()
    }
}
