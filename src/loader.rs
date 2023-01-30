use std::ffi::{c_uint, c_void};
use std::ops::{Deref, DerefMut};
use std::ptr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

use libloading::{Library, Symbol};
use once_cell::sync::OnceCell;
use renderdoc_sys::RENDERDOC_Version;

use crate::Error;

pub type RawRenderDoc = renderdoc_sys::RENDERDOC_API_1_6_0;

static LIBRARY: OnceCell<Mutex<Library>> = OnceCell::new();
static INSTANCE_ACTIVE: AtomicBool = AtomicBool::new(false);

/// Initializes the RenderDoc API and returns a pointer to its entry point.
///
/// This function guarantees the library will be loaded once and only once. Additionally, only one
/// `FunctionTable` instance may exist at any given time. Any subsequent attempts to call this
/// function while a `FunctionTable` is still live will result in an error.
///
/// Concurrent calls to `RENDERDOC_GetAPI` are synchronized and are therefore safe to call from
/// multiple threads (not that you'd intentionally want to).
///
/// Returns `Err` if the application is not running inside RenderDoc, the library could not be
/// found in `$PATH`, or another error occurred while opening the API.
pub fn load(version: RENDERDOC_Version) -> Result<FunctionTable, Error> {
    type GetApiFn = unsafe extern "C" fn(ver: c_uint, out: *mut *mut c_void) -> i32;

    #[cfg(windows)]
    let lib_path = "renderdoc.dll";
    #[cfg(all(unix, not(target_os = "android")))]
    let lib_path = "librenderdoc.so";
    #[cfg(target_os = "android")]
    let lib_path = "libVkLayer_GLES_RenderDoc.so";

    let lib = LIBRARY
        .get_or_try_init(|| unsafe { load_library(lib_path).map(Mutex::new) })
        .map_err(Error::library)?
        .lock()
        .unwrap();

    if INSTANCE_ACTIVE.load(Ordering::SeqCst) {
        return Err(Error::multiple_instances());
    }

    let api = unsafe {
        let get_api: Symbol<GetApiFn> = lib.get(b"RENDERDOC_GetAPI\0").map_err(Error::symbol)?;

        let mut obj = ptr::null_mut();
        match get_api(version, &mut obj) {
            1 => obj as *mut RawRenderDoc,
            _ => return Err(Error::no_compatible_api()),
        }
    };

    INSTANCE_ACTIVE.store(true, Ordering::SeqCst);

    Ok(FunctionTable { api })
}

/// A unique smart pointer to the RenderDoc API entry point.
pub struct FunctionTable {
    api: *mut RawRenderDoc,
}

impl FunctionTable {
    /// Returns the underlying pointer to the API function table.
    ///
    /// # Safety
    ///
    /// Making copies of this pointer and mutating the RenderDoc API from multiple locations breaks
    /// the invariants of this loader. By calling this, you are responsible for using the raw entry
    /// point safely.
    #[inline]
    pub unsafe fn inner(&self) -> *mut RawRenderDoc {
        self.api
    }
}

impl Deref for FunctionTable {
    type Target = RawRenderDoc;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.api }
    }
}

impl DerefMut for FunctionTable {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.api }
    }
}

impl Drop for FunctionTable {
    fn drop(&mut self) {
        INSTANCE_ACTIVE.store(false, Ordering::SeqCst);
    }
}

unsafe impl Send for FunctionTable {}

#[cfg(all(windows, not(feature = "ci")))]
unsafe fn load_library(path: &str) -> Result<Library, libloading::Error> {
    libloading::os::windows::Library::open_already_loaded(path).map(Library::from)
}

#[cfg(all(unix, not(feature = "ci")))]
unsafe fn load_library(path: &str) -> Result<Library, libloading::Error> {
    // TODO: Use constant from `libloading`, once added upstream.
    const RTLD_NOLOAD: i32 = 0x4;

    let flags = libloading::os::unix::RTLD_NOW | RTLD_NOLOAD;
    libloading::os::unix::Library::open(Some(path), flags).map(Library::from)
}

#[cfg(feature = "ci")]
unsafe fn load_library(path: &str) -> Result<Library, libloading::Error> {
    Library::new(path)
}
