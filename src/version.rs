//! Entry points for the RenderDoc API.

use std::os::raw::c_void;
use std::path::Path;

use libloading::{Library, Symbol};
use renderdoc_sys::RENDERDOC_API_1_4_0;

/// Entry point for the RenderDoc API.
pub type Entry = RENDERDOC_API_1_4_0;

#[cfg(windows)]
fn get_path() -> &'static Path {
    Path::new("renderdoc.dll")
}

#[cfg(all(unix, not(target_os = "android")))]
fn get_path() -> &'static Path {
    Path::new("librenderdoc.so")
}

#[cfg(target_os = "android")]
fn get_path() -> &'static Path {
    Path::new("libVkLayer_GLES_RenderDoc.so")
}

lazy_static! {
    static ref RD_LIB: Result<Library, libloading::Error> = Library::new(get_path());
}

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
}

/// Initializes a new instance of the RenderDoc API.
///
/// # Safety
///
/// This function is not thread-safe and should not be called on multiple threads at once.
type GetApiFn = unsafe extern "C" fn(ver: VersionCode, out: *mut *mut c_void) -> i32;

/// Entry point into the RenderDoc API.
pub trait Version {
    /// Minimum compatible version number.
    const VERSION: VersionCode;

    /// Initializes a new instance of the RenderDoc API.
    ///
    /// # Safety
    ///
    /// This function is not thread-safe and should not be called on multiple threads at once.
    fn load() -> Result<*mut Entry, String> {
        use std::ptr;

        unsafe {
            let lib = RD_LIB.as_ref().map_err(ToString::to_string)?;
            let get_api: Symbol<GetApiFn> =
                lib.get(b"RENDERDOC_GetAPI\0").map_err(|e| e.to_string())?;

            let mut obj = ptr::null_mut();
            match get_api(Self::VERSION, &mut obj) {
                1 => Ok(obj as *mut Entry),
                _ => Err("Compatible API version not available.".into()),
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
