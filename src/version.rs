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

pub trait HasShutdown {}

pub trait HasRemoveHooks {}

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
    ($($name:ident => $version_code:ident $( ( $($mixin:ident),* ) )?),+) => {
        $(define_versions!(@version $name $version_code $( $($mixin)+ )?);)+

        define_versions!(@minimum $($name)+);
    };

    (@version $name:ident $version_code:ident $($mixin:ident)*) => {
        impl Version for $name {
            const VERSION: RENDERDOC_Version = renderdoc_sys::$version_code;
        }

        $(impl $mixin for $name {})*
    };

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
}

define_versions! {
    V100 => eRENDERDOC_API_Version_1_0_0 (HasShutdown),
    V101 => eRENDERDOC_API_Version_1_0_1 (HasShutdown),
    V102 => eRENDERDOC_API_Version_1_0_2 (HasShutdown),
    V110 => eRENDERDOC_API_Version_1_1_0 (HasShutdown),
    V111 => eRENDERDOC_API_Version_1_1_1 (HasShutdown),
    V112 => eRENDERDOC_API_Version_1_1_2 (HasShutdown),
    V120 => eRENDERDOC_API_Version_1_2_0 (HasShutdown),
    V130 => eRENDERDOC_API_Version_1_3_0 (HasShutdown),
    V140 => eRENDERDOC_API_Version_1_4_0 (HasShutdown),
    V141 => eRENDERDOC_API_Version_1_4_1 (HasRemoveHooks),
    V142 => eRENDERDOC_API_Version_1_4_2 (HasRemoveHooks),
    V150 => eRENDERDOC_API_Version_1_5_0 (HasRemoveHooks),
    V160 => eRENDERDOC_API_Version_1_6_0 (HasRemoveHooks)
}
