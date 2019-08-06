//! Rust bindings to [RenderDoc], a popular graphics debugger.
//!
//! [RenderDoc]: https://renderdoc.org/
//!
//! RenderDoc is a free and open source graphics debugging tool. RenderDoc allows game developers
//! to take frame captures of their applications, replay them, examine the graphics pipeline state,
//! and potentially identify nasty graphics bugs.
//!
//! These bindings require that RenderDoc be installed on the target machine, with either
//! `renderdoc.dll` or `librenderdoc.so` visible from your`PATH`.
//!
//! For more details on how to use this API to integrate your game or renderer with the RenderDoc
//! profiler, consult the upstream [in-application API][in-app] documentation.
//!
//! [in-app]: https://renderdoc.org/docs/in_application_api.html

#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

#[cfg(any(target_os = "macos", target_os = "ios"))]
compile_error!("RenderDoc does not support this platform.");

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate float_cmp;
extern crate libloading;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate renderdoc_derive;
extern crate renderdoc_sys;

#[cfg(feature = "glutin")]
extern crate glutin;
#[cfg(target_os = "windows")]
extern crate winapi;
#[cfg(target_os = "windows")]
extern crate wio;

pub use self::handles::{DevicePointer, WindowHandle};
pub use self::renderdoc::RenderDoc;
pub use self::settings::{CaptureOption, InputButton, OverlayBits};
pub use self::version::{Entry, HasPrevious, Version, V100, V110, V111, V112, V120, V130, V140};

use std::os::raw::c_ulonglong;

#[cfg(windows)]
use winapi::shared::guiddef::GUID;
#[cfg(windows)]
use wio::com::ComPtr;

mod handles;
mod renderdoc;
mod settings;
mod version;

/// Magic value used for when applications pass a path where shader debug information can be found
/// to match up with a stripped shader.
///
/// Windows GUID representation intended for consumption by D3D.
#[cfg(windows)]
pub const SHADER_MAGIC_DEBUG_VALUE_STRUCT: GUID = GUID {
    Data1: 0xeab25520,
    Data2: 0x6670,
    Data3: 0x4865,
    Data4: [0x84, 0x29, 0x6c, 0x8, 0x51, 0x54, 0x00, 0xff],
};

/// Magic value used for when applications pass a path where shader debug information can be found
/// to match up with a stripped shader.
///
/// Raw byte array representation (assuming x86 endianness).
pub const SHADER_MAGIC_DEBUG_VALUE_BYTE_ARRAY: &[u8] = &[
    0x20, 0x55, 0xb2, 0xea, 0x70, 0x66, 0x65, 0x48, 0x84, 0x29, 0x6c, 0x8, 0x51, 0x54, 0x00, 0xff,
];

/// Magic value used for when applications pass a path where shader debug information can be found
/// to match up with a stripped shader.
///
/// Truncated version when only a `uint64_t` is available (e.g. Vulkan tags).
pub const SHADER_MAGIC_DEBUG_VALUE_TRUNCATED: c_ulonglong = 0x0485_6670_eab2_5520;

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(intls, 1u32);

        let cmds = rd.get_capture_option_u32(CaptureOption::CaptureAllCmdLists);
        assert_eq!(cmds, 0u32);

        let is_muted = rd.get_capture_option_u32(CaptureOption::DebugOutputMute);
        assert_eq!(is_muted, 1u32);
    }
}
