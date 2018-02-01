//! In-app API bindings.

pub use self::types::{CaptureOption, DevicePointer, InputButton, OverlayBits, WindowHandle};
pub use self::version::{V100, V110, Version};
pub use renderdoc_sys::app as ffi;

pub mod api;
pub mod prelude;
pub mod version;

mod types;

use std::mem;
use std::os::raw::c_ulonglong;

#[cfg(windows)]
use winapi::shared::guiddef::GUID;

/// Magic value used for when applications pass a path where shader debug
/// information can be found to match up with a stripped shader.
///
/// Windows GUID representation intended for consumption by D3D.
#[cfg(windows)]
pub const SHADER_MAGIC_DEBUG_VALUE_STRUCT: GUID = GUID {
    Data1: 0xeab25520,
    Data2: 0x6670,
    Data3: 0x4865,
    Data4: [0x84, 0x29, 0x6c, 0x8, 0x51, 0x54, 0x00, 0xff],
};

/// Magic value used for when applications pass a path where shader debug
/// information can be found to match up with a stripped shader.
///
/// Raw byte array representation (assuming x86 endianness).
pub const SHADER_MAGIC_DEBUG_VALUE_BYTE_ARRAY: &[u8] = &[
    0x20, 0x55, 0xb2, 0xea, 0x70, 0x66, 0x65, 0x48, 0x84, 0x29, 0x6c, 0x8, 0x51, 0x54, 0x00, 0xff
];

/// Magic value used for when applications pass a path where shader debug
/// information can be found to match up with a stripped shader.
///
/// Truncated version when only a `uint64_t` is available (e.g. Vulkan tags).
pub const SHADER_MAGIC_DEBUG_VALUE_TRUNCATED: c_ulonglong = 0x4856670eab25520;

/// An instance of the RenderDoc API with baseline version `V`.
#[derive(Debug)]
pub struct RenderDoc<V: Version>(V::Entry);

impl<V: Version> RenderDoc<V> {
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
        RenderDoc(newer.0)
    }
}

impl api::RenderDocV100 for RenderDoc<V100> {
    unsafe fn entry_v100(&self) -> &ffi::RENDERDOC_API_1_0_0 {
        &self.0
    }
}

impl api::RenderDocV100 for RenderDoc<V110> {
    unsafe fn entry_v100(&self) -> &ffi::RENDERDOC_API_1_0_0 {
        &self.0
    }
}

impl api::RenderDocV110 for RenderDoc<V110> {
    unsafe fn entry_v110(&self) -> &ffi::RENDERDOC_API_1_1_0 {
        mem::transmute(&self.0)
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
