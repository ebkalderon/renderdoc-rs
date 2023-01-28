#[cfg(any(target_os = "macos", target_os = "ios"))]
compile_error!("RenderDoc does not support this platform.");

use std::marker::PhantomData;

pub use self::error::Error;
pub use self::version::{
    RawRenderDoc, Version, V100, V101, V102, V110, V111, V112, V120, V130, V140, V141, V142, V150,
    V160,
};

use self::version::Minimum;

mod error;
mod version;

pub struct RenderDoc<V = V160> {
    api: *mut RawRenderDoc,
    _version: PhantomData<V>,
}

impl<V: Version> RenderDoc<V> {
    /// Initializes a new instance of the RenderDoc API.
    ///
    /// Note that RenderDoc will usually provide a higher API version than the one requested by the
    /// user, provided it is backwards compatible.
    pub fn new() -> Result<Self, Error> {
        Ok(RenderDoc {
            api: V::load()?,
            _version: PhantomData,
        })
    }

    /// Asserts that the run-time API version is _at least_ `U` or newer.
    ///
    /// As the documentation for [`RenderDoc::new()`] mentions, RenderDoc will usually provide a
    /// higher API version than the one requested by the user, provided it is backwards compatible.
    ///
    /// Converts this `RenderDoc<V>` into a `RenderDoc<U>` if the actual API version is ≥ `U`.
    /// Otherwise, returns the original `RenderDoc<V>` as an error.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use renderdoc::Error;
    /// # fn main() -> Result<(), Error> {
    /// use renderdoc::{RenderDoc, V100, V120};
    ///
    /// let renderdoc: RenderDoc<V100> = RenderDoc::new()?;
    ///
    /// match renderdoc.try_upgrade::<V120>() {
    ///     Ok(_newer) => {} // We actually have 1.2.0 or newer!
    ///     Err(_orig) => {} // Version is below 1.2.0
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn try_upgrade<U>(self) -> Result<RenderDoc<U>, Self>
    where
        U: Version + Minimum<V>,
    {
        let (major, minor, patch) = self.api_version();
        if version::from_digits(major, minor, patch) >= U::VERSION {
            Ok(RenderDoc {
                api: self.api,
                _version: PhantomData,
            })
        } else {
            Err(self)
        }
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

impl<V: Minimum<V100>> RenderDoc<V> {
    /// Returns the major, minor, and patch version numbers of the RenderDoc API currently in use.
    ///
    /// Note that RenderDoc will usually provide a higher API version than the one requested by
    /// the user if it is backwards compatible.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use renderdoc::Error;
    /// # fn main() -> Result<(), Error> {
    /// use renderdoc::{RenderDoc, V120};
    ///
    /// let renderdoc: RenderDoc<V120> = RenderDoc::new()?;
    /// let (major, _minor, _patch) = renderdoc.api_version();
    /// assert_eq!(major, 1);
    /// # Ok(())
    /// # }
    /// ```
    pub fn api_version(&self) -> (u8, u8, u8) {
        let (mut major, mut minor, mut patch) = (0, 0, 0);

        unsafe {
            (self.api.GetAPIVersion.unwrap())(&mut major, &mut minor, &mut patch);
        }

        (major as u8, minor as u8, patch as u8)
    }
}

unsafe impl<V> Send for RenderDoc<V> {}
