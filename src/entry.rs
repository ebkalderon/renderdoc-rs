//! Entry points for the RenderDoc API.

use renderdoc_sys;

/// Entry point for RenderDoc API version 1.0.0.
pub type EntryV100 = renderdoc_sys::RENDERDOC_API_1_0_0;
/// Entry point for RenderDoc API version 1.1.0.
pub type EntryV110 = renderdoc_sys::RENDERDOC_API_1_1_0;
/// Entry point for RenderDoc API version 1.1.1.
pub type EntryV111 = renderdoc_sys::RENDERDOC_API_1_1_1;
/// Entry point for RenderDoc API version 1.1.2.
pub type EntryV112 = renderdoc_sys::RENDERDOC_API_1_1_2;
/// Entry point for RenderDoc API version 1.2.0.
pub type EntryV120 = renderdoc_sys::RENDERDOC_API_1_2_0;

use std::io::Error as IoError;
use std::path::Path;

use libloading::{Library, Symbol};

#[cfg(windows)]
fn get_path() -> &'static Path {
    Path::new("renderdoc.dll")
}

#[cfg(unix)]
fn get_path() -> &'static Path {
    Path::new("librenderdoc.so")
}

lazy_static! {
    static ref RD_LIB: Result<Library, IoError> = Library::new(get_path());
}

/// Available versions of the RenderDoc API.
#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Version {
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
}

/// Initializes a new instance of the RenderDoc API.
///
/// # Safety
///
/// This function is not thread-safe and should not be called on multiple
/// threads at once.
type GetApiFn<T> = unsafe extern "C" fn(ver: Version, out: *mut *mut T) -> i32;

/// Entry point into the RenderDoc API.
pub trait ApiVersion {
    /// Minimum compatible version number.
    const VERSION: Version;

    /// Entry point struct.
    type Entry: Clone;

    /// Initializes a new instance of the RenderDoc API.
    ///
    /// # Safety
    ///
    /// This function is not thread-safe and should not be called on multiple
    /// threads at once.
    fn load() -> Result<Self::Entry, String> {
        use std::ptr;

        let api = unsafe {
            let lib = RD_LIB.as_ref().map_err(|e| e.to_string())?;
            let get_api: Symbol<GetApiFn<Self::Entry>> =
                lib.get(b"RENDERDOC_GetAPI\0").map_err(|e| e.to_string())?;

            let mut obj = ptr::null_mut();
            match get_api(Self::VERSION, &mut obj) {
                1 => ptr::read(obj),
                _ => Err("Compatible API version not available.")?,
            }
        };

        Ok(api)
    }
}

/// Requests a minimum version number of 1.0.0.
pub enum V100 {}

impl ApiVersion for V100 {
    const VERSION: Version = Version::V100;

    type Entry = EntryV100;
}

/// Requests a minimum version number of 1.1.0.
pub enum V110 {}

impl ApiVersion for V110 {
    const VERSION: Version = Version::V110;

    type Entry = EntryV110;
}

/// Requests a minimum version number of 1.1.1.
pub enum V111 {}

impl ApiVersion for V111 {
    const VERSION: Version = Version::V111;

    type Entry = EntryV111;
}

/// Requests a minimum version number of 1.1.2.
pub enum V112 {}

impl ApiVersion for V112 {
    const VERSION: Version = Version::V112;

    type Entry = EntryV112;
}

/// Requests a minimum version number of 1.2.0.
pub enum V120 {}

impl ApiVersion for V120 {
    const VERSION: Version = Version::V120;

    type Entry = EntryV120;
}
