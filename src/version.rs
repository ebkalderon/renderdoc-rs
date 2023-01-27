//! Entry points for the RenderDoc API.

use std::ffi::c_void;
use std::ptr;

use libloading::{Library, Symbol};
use once_cell::sync::OnceCell;
use renderdoc_sys::RENDERDOC_API_1_4_1;

use crate::error::Error;

/// Entry point for the RenderDoc API.
pub type Entry = RENDERDOC_API_1_4_1;

/// Available versions of the RenderDoc API.
#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum VersionCode {
    /// Version 1.0.0.
    V100 = 10000,
    /// Version 1.0.1.
    V101 = 10001,
    /// Version 1.0.2.
    V102 = 10002,
    /// Version 1.1.0.
    V110 = 10100,
    /// Version 1.1.1.
    V111 = 10101,
    /// Version 1.1.2.
    V112 = 10102,
    /// Version 1.2.0.
    V120 = 10200,
    /// Version 1.3.0.
    V130 = 10300,
    /// Version 1.4.0.
    V140 = 10400,
    /// Version 1.4.1.
    V141 = 10401,
}

/// Entry point into the RenderDoc API.
pub trait Version {
    /// Minimum compatible version number.
    const VERSION: VersionCode;

    /// Initializes a new instance of the RenderDoc API.
    ///
    /// # Safety
    ///
    /// This function is not thread-safe and should not be called on multiple threads at once.
    fn load() -> Result<*mut Entry, Error> {
        static LIBRARY: OnceCell<Library> = OnceCell::new();

        type GetApiFn = unsafe extern "C" fn(ver: u32, out: *mut *mut c_void) -> i32;

        #[cfg(windows)]
        let lib_path = "renderdoc.dll";
        #[cfg(all(unix, not(target_os = "android")))]
        let lib_path = "librenderdoc.so";
        #[cfg(target_os = "android")]
        let lib_path = "libVkLayer_GLES_RenderDoc.so";

        unsafe {
            #[cfg(not(feature = "ci"))]
            #[cfg(unix)]
            let lib = LIBRARY
                .get_or_try_init(|| {
                    // TODO: Use constant from `libloading`, once added upstream.
                    const RTLD_NOLOAD: i32 = 0x4;

                    let flags = libloading::os::unix::RTLD_NOW | RTLD_NOLOAD;
                    libloading::os::unix::Library::open(Some(lib_path), flags).map(Into::into)
                })
                .map_err(Error::library)?;

            #[cfg(not(feature = "ci"))]
            #[cfg(windows)]
            let lib = LIBRARY
                .get_or_try_init(|| {
                    libloading::os::windows::Library::open_already_loaded(lib_path).map(Into::into)
                })
                .map_err(Error::library)?;

            #[cfg(feature = "ci")]
            let lib = LIBRARY
                .get_or_try_init(|| Library::new(lib_path))
                .map_err(Error::library)?;

            let get_api: Symbol<GetApiFn> =
                lib.get(b"RENDERDOC_GetAPI\0").map_err(Error::symbol)?;

            let mut obj = ptr::null_mut();
            match get_api(Self::VERSION as u32, &mut obj) {
                1 => Ok(obj as *mut Entry),
                _ => Err(Error::no_compatible_api()),
            }
        }
    }
}

/// Trait for statically enforcing backwards compatibility.
pub trait HasPrevious: Version {
    /// API version which immediately precedes this one.
    type Previous: Version;
}

/// Requests a minimum version number of 1.0.0.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum V100 {}

impl Version for V100 {
    const VERSION: VersionCode = VersionCode::V100;
}

/// Requests a minimum version number of 1.1.0.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum V110 {}

impl Version for V110 {
    const VERSION: VersionCode = VersionCode::V110;
}

impl HasPrevious for V110 {
    type Previous = V100;
}

/// Requests a minimum version number of 1.1.1.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum V111 {}

impl Version for V111 {
    const VERSION: VersionCode = VersionCode::V111;
}

impl HasPrevious for V111 {
    type Previous = V110;
}

/// Requests a minimum version number of 1.1.2.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum V112 {}

impl Version for V112 {
    const VERSION: VersionCode = VersionCode::V112;
}

impl HasPrevious for V112 {
    type Previous = V111;
}

/// Requests a minimum version number of 1.2.0.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum V120 {}

impl Version for V120 {
    const VERSION: VersionCode = VersionCode::V120;
}

impl HasPrevious for V120 {
    type Previous = V112;
}

/// Requests a minimum version number of 1.3.0.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum V130 {}

impl Version for V130 {
    const VERSION: VersionCode = VersionCode::V130;
}

impl HasPrevious for V130 {
    type Previous = V120;
}

/// Requests a minimum version number of 1.4.0.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum V140 {}

impl Version for V140 {
    const VERSION: VersionCode = VersionCode::V140;
}

impl HasPrevious for V140 {
    type Previous = V130;
}

/// Requests a minimum version number of 1.4.1.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum V141 {}

impl Version for V141 {
    const VERSION: VersionCode = VersionCode::V141;
}

impl HasPrevious for V141 {
    type Previous = V140;
}
