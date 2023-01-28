use std::ffi::{c_uint, c_void};
use std::ptr;

use libloading::{Library, Symbol};
use once_cell::sync::OnceCell;
use renderdoc_sys::RENDERDOC_Version;

use crate::Error;

pub type RawRenderDoc = renderdoc_sys::RENDERDOC_API_1_6_0;

pub trait Version {
    const VERSION: RENDERDOC_Version;

    /// Initializes a new instance of the RenderDoc API.
    ///
    /// # Safety
    ///
    /// This function is not thread-safe and should not be called on multiple threads at once.
    fn load() -> Result<*mut RawRenderDoc, Error> {
        static LIBRARY: OnceCell<Library> = OnceCell::new();

        type GetApiFn = unsafe extern "C" fn(ver: c_uint, out: *mut *mut c_void) -> i32;

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
            match get_api(Self::VERSION as c_uint, &mut obj) {
                1 => Ok(obj as *mut RawRenderDoc),
                _ => Err(Error::no_compatible_api()),
            }
        }
    }
}

pub trait Minimum<V: Version> {}
pub trait Below<V: Version> {}

pub enum V100 {}
pub enum V101 {}
pub enum V102 {}
pub enum V110 {}
pub enum V111 {}
pub enum V112 {}
pub enum V120 {}
pub enum V130 {}
pub enum V140 {}
pub enum V141 {}
pub enum V142 {}
pub enum V150 {}
pub enum V160 {}

macro_rules! define_versions {
    // Entry point into the macro.
    ($($name:ident => $version_code:ident),+) => {
        $(define_versions!(@version $name $version_code);)+

        define_versions!(@minimum $($name)+);
        define_versions!(@below $($name)+);
    };

    // Implements the `Version` trait for `$name`.
    (@version $name:ident $version_code:ident) => {
        impl Version for $name {
            const VERSION: RENDERDOC_Version = renderdoc_sys::$version_code;
        }
    };

    // Implements the `Minimum<V>` trait for all version enums.
    (@minimum $first:ident $($rest:ident)*) => {
        impl<V: Version> Minimum<$first> for V {}

        define_versions!(@minimum_rec $($rest)*);
    };

    (@minimum_rec $first:ident $second:ident $($rest:ident)*) => {
        impl Minimum<$first> for $first {}

        impl<V: Minimum<$second>> Minimum<$first> for V {}

        define_versions!(@minimum_rec $second $($rest)*);
    };

    (@minimum_rec $single:ident) => {
        impl Minimum<$single> for $single {}
    };

    // Implements the `Below<V>` trait for all version enums.
    (@below $($name:ident)*) => {
        define_versions!(@below_rev [$($name)*]);
    };

    (@below_rev [$first:ident $($rest:ident)*] $($reversed:ident)*) => {
        define_versions!(@below_rev [$($rest)*] $first $($reversed)*);
    };

    (@below_rev [] $($reversed:ident)*) => {
        define_versions!(@below_rec $($reversed)*);
    };

    (@below_rec $first:ident $second:ident $($rest:ident)+) => {
        impl Below<$first> for $second {}

        impl<V: Below<$second>> Below<$first> for V {}

        define_versions!(@below_rec $second $($rest)*);
    };

    (@below_rec $first:ident $second:ident) => {
        impl Below<$first> for $second {}
    };
}

define_versions! {
    V100 => eRENDERDOC_API_Version_1_0_0,
    V101 => eRENDERDOC_API_Version_1_0_1,
    V102 => eRENDERDOC_API_Version_1_0_2,
    V110 => eRENDERDOC_API_Version_1_1_0,
    V111 => eRENDERDOC_API_Version_1_1_1,
    V112 => eRENDERDOC_API_Version_1_1_2,
    V120 => eRENDERDOC_API_Version_1_2_0,
    V130 => eRENDERDOC_API_Version_1_3_0,
    V140 => eRENDERDOC_API_Version_1_4_0,
    V141 => eRENDERDOC_API_Version_1_4_1,
    V142 => eRENDERDOC_API_Version_1_4_2,
    V150 => eRENDERDOC_API_Version_1_5_0,
    V160 => eRENDERDOC_API_Version_1_6_0
}

/// Encodes the given RenderDoc version into a valid `RENDERDOC_Version`.
#[inline]
pub const fn from_digits(major: u8, minor: u8, patch: u8) -> RENDERDOC_Version {
    let mut version = major as c_uint * 10_000;
    version += minor as c_uint * 100;
    version += patch as c_uint;
    version
}

#[cfg(test)]
mod tests {
    use renderdoc_sys::*;

    use super::*;

    #[test]
    fn encodes_version_from_digits() {
        let versions = [
            ((1, 0, 0), eRENDERDOC_API_Version_1_0_0),
            ((1, 0, 1), eRENDERDOC_API_Version_1_0_1),
            ((1, 0, 2), eRENDERDOC_API_Version_1_0_2),
            ((1, 1, 0), eRENDERDOC_API_Version_1_1_0),
            ((1, 1, 1), eRENDERDOC_API_Version_1_1_1),
            ((1, 1, 2), eRENDERDOC_API_Version_1_1_2),
            ((1, 2, 0), eRENDERDOC_API_Version_1_2_0),
            ((1, 3, 0), eRENDERDOC_API_Version_1_3_0),
            ((1, 4, 0), eRENDERDOC_API_Version_1_4_0),
            ((1, 4, 1), eRENDERDOC_API_Version_1_4_1),
            ((1, 4, 2), eRENDERDOC_API_Version_1_4_2),
            ((1, 5, 0), eRENDERDOC_API_Version_1_5_0),
            ((1, 6, 0), eRENDERDOC_API_Version_1_6_0),
        ];

        for (input_digits, expected_version) in versions {
            let (major, minor, patch) = input_digits;
            let computed_version = from_digits(major, minor, patch);
            assert_eq!(computed_version, expected_version);
        }
    }
}
