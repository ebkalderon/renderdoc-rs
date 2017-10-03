//! Entry point loading and API versioning.

use std::os::raw::{c_char, c_int};
use std::path::Path;

use shared_library::dynamic_library::DynamicLibrary;

use super::{CaptureOption, DevicePointer, InputButton, OverlayBits, WindowHandle};

pub mod version;

#[cfg(windows)]
fn get_path() -> &'static Path {
    Path::new("renderdoc.dll")
}

#[cfg(unix)]
fn get_path() -> &'static Path {
    Path::new("librenderdoc.so")
}

lazy_static! {
    static ref RD_LIB: Result<DynamicLibrary, String> = DynamicLibrary::open(Some(get_path()));
}

/// Provides the major, minor, and patch version numbers of the RenderDoc API
/// given to the application.
///
/// Note that RenderDoc will usually provide a higher API version than the one
/// requested by the user if it's backwards compatible. If a parameter is
/// `std::ptr::null_mut()`, it will be ignored while the others will be filled
/// out.
pub type GetApiVersionFn = unsafe extern "C" fn(major: *mut c_int,
                                                minor: *mut c_int,
                                                patch: *mut c_int);

/// Sets the specified `CaptureOption` to the given `u32` value.
///
/// Returns `1` if the option and the value are valid. Otherwise, returns `0`
/// and leaves the option unchanged.
pub type SetCaptureOptionU32Fn = unsafe extern "C" fn(opt: CaptureOption, val: u32) -> c_int;

/// Sets the specified `CaptureOption` to the given `f32` value.
///
/// Returns `1` if the option and the value are valid. Otherwise, returns `0`
/// and leaves the option unchanged.
pub type SetCaptureOptionF32Fn = unsafe extern "C" fn(opt: CaptureOption, val: f32) -> c_int;

/// Returns the current value of the given `CaptureOption` as a `u32` value.
///
/// If the option is invalid, then `std::u32::MAX` is returned instead.
pub type GetCaptureOptionU32Fn = unsafe extern "C" fn(opt: CaptureOption) -> u32;

/// Returns the current value of the given `CaptureOption` as a `u32` value.
///
/// If the option is invalid, then `std::f32::MAX * -1f32` is returned instead.
pub type GetCaptureOptionF32Fn = unsafe extern "C" fn(opt: CaptureOption) -> f32;

/// Sets which key(s) can be used to toggle focus between multiple windows.
///
/// If `keys` is `std::ptr::null_mut()`, then window toggling will be disabled.
pub type SetFocusToggleKeysFn = unsafe extern "C" fn(keys: *mut InputButton, num: c_int);

/// Sets which key(s) can be used to capture the next frame.
///
/// If `keys` is `std::ptr::null_mut()`, then frame capture functionality will
/// be disabled.
pub type SetCaptureKeysFn = unsafe extern "C" fn(keys: *mut InputButton, num: c_int);

/// Returns the `OverlayBits` that have been set.
pub type GetOverlayBitsFn = unsafe extern "C" fn() -> OverlayBits;

/// Sets the given `OverlayBits` with an AND and OR mask.
pub type MaskOverlayBitsFn = unsafe extern "C" fn(and: OverlayBits, or: OverlayBits);

/// Attempts to shut down RenderDoc.
///
/// Note that this will work correctly if done _immediately_ after the dynamic
/// library is loaded, before any API work happens. At that point, RenderDoc
/// will remove its injected hooks and shut down. Behavior is undefined if this
/// is called after any API functions have been called.
pub type ShutdownFn = unsafe extern "C" fn();

/// Unloads the RenderDoc crash handler from your application.
///
/// If you use your own crash handler and don't want RenderDoc's handler to
/// intercede, you may call this function to unload it and any unhandled
/// exceptions will pass to the next handler instead.
pub type UnloadCrashHandlerFn = unsafe extern "C" fn();

/// Sets the naming prefix to be used when saving frame capture files.
///
/// `path_template` is a UTF-8 string that gives a template for how captures can
/// be named and where they will be saved. Any extension is stripped off the
/// path and the captures are saved in the directory specified with the file
/// name and frame number appended. If the requested directory or directory
/// structure does not exist, it will be created recursively for you.
///
/// If `path_template` is `std::ptr::null()`, then the template will be left
/// unchanged.
///
/// # Example
///
/// ```c
/// SetLogPathTemplateFn("my_captures/example");
/// 
/// // This function call will result in the following captures:
/// //
/// // Capture #1 -> my_captures/example_frame123.rdc
/// // Capture #2 -> my_captures/example_frame456.rdc.
/// ```
pub type SetLogFilePathTemplateFn = unsafe extern "C" fn(path_template: *const c_char);

/// Returns the current frame capture file template as a raw UTF-8 string.
///
/// See the `SetLogFilePathTemplateFn` description for details.
pub type GetLogFilePathTemplateFn = unsafe extern "C" fn() -> *const c_char;

/// Returns the number of frame captures that have been made so far.
pub type GetNumCapturesFn = unsafe extern "C" fn() -> u32;

/// Retrieves the details of a frame capture with the given index `idx`.
///
/// If `idx` is a valid frame capture number, then `log_file` will be filled
/// with the absolute UTF-8 formatted path to the capture file; `path_len` will
/// be the length in bytes of the `log_file` string; and `timestamp` will be the
/// time of capture, measured in seconds passed since the UNIX epoch.
///
/// If a parameter is set to `std::ptr::null_mut()`, it will be skipped and the
/// rest will be filled out.
///
/// Returns `1` if the capture index is valid. Otherwise, returns `0` and leaves
/// the values of `log_file`, `path_len`, and `timestamp` all unchanged.
pub type GetCaptureFn = unsafe extern "C" fn(idx: u32,
                                             log_file: *mut c_char,
                                             path_len: *mut u32,
                                             timestamp: *mut u64)
                                             -> u32;

/// Captures the next frame from the currently active window and API device.
///
/// Data is saved to a capture log file at the location specified via the
/// `SetLogFilePathTemplateFn` function call.
///
/// If no supported APIs have been initialized, this function will do nothing.
pub type TriggerCaptureFn = unsafe extern "C" fn();

/// Returns whether the external RenderDoc UI is connected to this application.
///
/// # Compatibility
///
/// The older name of this function, `IsRemoteAccessConnected`, has been
/// deprecated since RenderDoc version 1.1.1. However, since its function
/// signature is binary compatible with this one, there is no need for us to add
/// another type definition.
pub type IsTargetControlConnectedFn = unsafe extern "C" fn() -> u32;

/// Launches the replay UI from within the injected application.
///
/// If `connect_target_control` is `1`, the replay UI will launch with a
/// command line parameter specified by `cmd_line`, a UTF-8 string. If
/// `cmd_line` is `std::ptr::null()`, then the command line will be empty.
///
/// Returns the PID of the replay UI if successful, otherwise returns `0`.
pub type LaunchReplayUiFn = unsafe extern "C" fn(connect_target_control: u32,
                                                 cmd_line: *const c_char)
                                                 -> u32;

/// Activates the RenderDoc in-app overlay inside the given window handle
/// `wnd_handle` and API device pointer `device`.
///
/// Neither parameter can be `std::ptr::null_mut()`.
pub type SetActiveWindowFn = unsafe extern "C" fn(device: DevicePointer,
                                                  wnd_handle: WindowHandle);

/// Immediately starts capturing API calls from the specified device pointer
/// and window handle.
///
/// If `device` is `std::ptr::null_mut()`, then all API calls outputting to
/// `wnd_handle` will be captured, regardless of API device(s). This is useful
/// if the API device being used isn't necessarily known at runtime.
///
/// If `wnd_handle` is `std::ptr::null_mut()`, then all API calls to `device` 
/// will be captured, regardless of its output window(s). This is useful for
/// headless rendering.
///
/// If both `device` and `wnd_handle` are set to `std::ptr::null_mut()`, then
/// _all_ API calls in this application will be captured, regardless of output
/// window(s) and/or API device(s).
///
/// If no supported APIs have been initialized, this function will do nothing.
///
/// If two or more started captures overlap each other, then this will result
/// in undefined behavior (including crashes).
pub type StartFrameCaptureFn = unsafe extern "C" fn(device: DevicePointer,
                                                    wnd_handle: WindowHandle);

/// Returns whether or not a frame capture is currently ongoing anywhere.
pub type IsFrameCapturingFn = unsafe extern "C" fn() -> u32;

/// Ends the ongoing capture on the given device pointer and window handle.
///
/// Data is saved to a capture log file at the location specified via the
/// `SetLogFilePathTemplateFn` function call. Returns `1` if the capture
/// succeeded, otherwise returns `0`.
pub type EndFrameCaptureFn = unsafe extern "C" fn(device: DevicePointer,
                                                  wnd_handle: WindowHandle)
                                                  -> u32;

/// Captures the next _n_ frames from the currently active window and API device.
///
/// Data is saved to a capture log file at the location specified via the
/// `SetLogFilePathTemplateFn` function call.
///
/// If no supported APIs have been initialized, this function will do nothing.
pub type TriggerMultiFrameCaptureFn = unsafe extern "C" fn(num_frames: u32);

/// Entry point for RenderDoc API version 1.0.
#[allow(missing_docs)]
#[repr(C)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EntryV100 {
    pub get_api_version: GetApiVersionFn,
    pub set_capture_option_u32: SetCaptureOptionU32Fn,
    pub set_capture_option_f32: SetCaptureOptionF32Fn,
    pub get_capture_option_u32: GetCaptureOptionU32Fn,
    pub get_capture_option_f32: GetCaptureOptionF32Fn,
    pub set_focus_toggle_keys: SetFocusToggleKeysFn,
    pub set_capture_keys: SetCaptureKeysFn,
    pub get_overlay_bits: GetOverlayBitsFn,
    pub mask_overlay_bits: MaskOverlayBitsFn,
    pub shutdown: ShutdownFn,
    pub unload_crash_handler: UnloadCrashHandlerFn,
    pub set_log_file_path_template: SetLogFilePathTemplateFn,
    pub get_log_file_path_template: GetLogFilePathTemplateFn,
    pub get_num_captures: GetNumCapturesFn,
    pub get_capture: GetCaptureFn,
    pub trigger_capture: TriggerCaptureFn,
    pub is_target_control_connected: IsTargetControlConnectedFn,
    pub launch_replay_ui: LaunchReplayUiFn,
    pub set_active_window: SetActiveWindowFn,
    pub start_frame_capture: StartFrameCaptureFn,
    pub is_frame_capturing: IsFrameCapturingFn,
    pub end_frame_capture: EndFrameCaptureFn,
}

/// Entry point for RenderDoc API version 1.1.
#[allow(missing_docs)]
#[repr(C)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EntryV110 {
    pub get_api_version: GetApiVersionFn,
    pub set_capture_option_u32: SetCaptureOptionU32Fn,
    pub set_capture_option_f32: SetCaptureOptionF32Fn,
    pub get_capture_option_u32: GetCaptureOptionU32Fn,
    pub get_capture_option_f32: GetCaptureOptionF32Fn,
    pub set_focus_toggle_keys: SetFocusToggleKeysFn,
    pub set_capture_keys: SetCaptureKeysFn,
    pub get_overlay_bits: GetOverlayBitsFn,
    pub mask_overlay_bits: MaskOverlayBitsFn,
    pub shutdown: ShutdownFn,
    pub unload_crash_handler: UnloadCrashHandlerFn,
    pub set_log_file_path_template: SetLogFilePathTemplateFn,
    pub get_log_file_path_template: GetLogFilePathTemplateFn,
    pub get_num_captures: GetNumCapturesFn,
    pub get_capture: GetCaptureFn,
    pub trigger_capture: TriggerCaptureFn,
    pub is_target_control_connected: IsTargetControlConnectedFn,
    pub launch_replay_ui: LaunchReplayUiFn,
    pub set_active_window: SetActiveWindowFn,
    pub start_frame_capture: StartFrameCaptureFn,
    pub is_frame_capturing: IsFrameCapturingFn,
    pub end_frame_capture: EndFrameCaptureFn,
    pub trigger_multi_frame_capture: TriggerMultiFrameCaptureFn,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn entry_v1_0_0_layout() {
        assert_eq!(
            mem::size_of::<EntryV100>(),
            176usize,
            concat!("Size of: ", stringify!(EntryV100))
        );

        assert_eq!(
            mem::align_of::<EntryV100>(),
            8usize,
            concat!("Alignment of ", stringify!(EntryV100))
        );
    }

    #[test]
    fn entry_v1_1_0_layout() {
        assert_eq!(
            mem::size_of::<EntryV110>(),
            184usize,
            concat!("Size of: ", stringify!(EntryV110))
        );

        assert_eq!(
            mem::align_of::<EntryV110>(),
            8usize,
            concat!("Alignment of ", stringify!(EntryV110))
        );
    }
}
