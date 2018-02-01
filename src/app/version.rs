//! API versioning.

use super::ffi;

use std::path::Path;

use shared_library::dynamic_library::DynamicLibrary;

#[cfg(windows)]
fn get_path() -> &'static Path {
    Path::new("renderdoc.dll")
}

#[cfg(unix)]
fn get_path() -> &'static Path {
    Path::new("/home/ekalderon/renderdoc/build/bin/librenderdoc.so")
}

lazy_static! {
    static ref RD_LIB: Result<DynamicLibrary, String> = DynamicLibrary::open(Some(get_path()));
}

/// Requested version of the RenderDoc API.
#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum RawVersion {
    /// Version 1.0.0.
    V100,
    /// Version 1.0.1.
    V101,
    /// Version 1.0.2.
    V102,
    /// Version 1.1.0.
    V110,
    /// Version 1.1.1.
    V111,
}

impl From<RawVersion> for ffi::RENDERDOC_Version {
    fn from(ver: RawVersion) -> Self {
        match ver {
            RawVersion::V100 => ffi::RENDERDOC_Version_eRENDERDOC_API_Version_1_0_0,
            RawVersion::V101 => ffi::RENDERDOC_Version_eRENDERDOC_API_Version_1_0_1,
            RawVersion::V102 => ffi::RENDERDOC_Version_eRENDERDOC_API_Version_1_0_2,
            RawVersion::V110 => ffi::RENDERDOC_Version_eRENDERDOC_API_Version_1_1_0,
            RawVersion::V111 => ffi::RENDERDOC_Version_eRENDERDOC_API_Version_1_1_1,
        }
    }
}

/// Initializes a new instance of the RenderDoc API.
///
/// # Safety
///
/// This function is not thread-safe and should not be called on multiple
/// threads at once.
type GetApiFn<T> = unsafe extern "C" fn(ver: ffi::RENDERDOC_Version, out: *mut *mut T) -> i32;

/// Entry point into the RenderDoc API.
pub trait Version {
    /// Minimum compatible version number.
    const VERSION: RawVersion;

    /// Entry point struct.
    type Entry: Clone;

    /// Initializes a new instance of the RenderDoc API.
    ///
    /// # Safety
    ///
    /// This function is not thread-safe and should not be called on multiple
    /// threads at once.
    fn load() -> Result<Self::Entry, String> {
        use std::{mem, ptr};

        let api = unsafe {
            let get_api = match *RD_LIB {
                Ok(ref lib) => {
                    let f = lib.symbol::<()>("RENDERDOC_GetAPI")?;
                    Ok(mem::transmute::<_, GetApiFn<Self::Entry>>(f))
                }
                Err(ref err) => Err(err.to_string()),
            }?;

            let mut obj = ptr::null_mut();
            match get_api(Self::VERSION.into(), &mut obj) {
                1 => ptr::read(obj),
                _ => Err("Compatible API version not available.")?,
            }
        };

        Ok(api)
    }
}

/// Requests a minimum version number of 1.0.0.
pub enum V100 {}

impl Version for V100 {
    const VERSION: RawVersion = RawVersion::V100;

    type Entry = ffi::RENDERDOC_API_1_1_0;
}

/// Requests a minimum version number of 1.1.0.
pub enum V110 {}

impl Version for V110 {
    const VERSION: RawVersion = RawVersion::V110;

    type Entry = ffi::RENDERDOC_API_1_1_0;
}
