#[cfg(any(target_os = "macos", target_os = "ios"))]
compile_error!("RenderDoc does not support this platform.");

use std::marker::PhantomData;

pub use self::error::Error;
pub use self::version::{
    Minimum, RawRenderDoc, Version, V100, V101, V102, V110, V111, V112, V120, V130, V140, V141,
    V142, V150, V160,
};

mod error;
mod version;

pub struct RenderDoc<V = V160> {
    api: *mut RawRenderDoc,
    _version: PhantomData<V>,
}

impl<V: Version> RenderDoc<V> {
    /// Initializes a new instance of the RenderDoc API.
    ///
    /// Note that RenderDoc will usually provide a higher API version than the one requested by
    /// the user if it is backwards compatible.
    pub fn new() -> Result<Self, Error> {
        Ok(RenderDoc {
            api: V::load()?,
            _version: PhantomData,
        })
    }

    /// Returns the underlying API entry point struct.
    ///
    /// # Safety
    ///
    /// Directly accessing this function table discards any and all safety features of this library.
    pub unsafe fn raw_api(&self) -> *mut RawRenderDoc {
        self.api
    }
}

impl<V: Minimum<V100>> RenderDoc<V> {}

unsafe impl<V> Send for RenderDoc<V> {}
