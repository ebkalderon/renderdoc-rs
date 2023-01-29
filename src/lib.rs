#[cfg(any(target_os = "macos", target_os = "ios"))]
compile_error!("RenderDoc does not support this platform.");

use std::fmt::{self, Debug, Formatter};
use std::marker::PhantomData;

pub use self::capture_opts::{CaptureCallstacksOption, CaptureOptions, SetCaptureOptions};
pub use self::error::Error;
pub use self::input_button::{AsInputButtons, InputButton};
pub use self::version::{
    RawRenderDoc, Version, V100, V101, V102, V110, V111, V112, V120, V130, V140, V141, V142, V150,
    V160,
};

use self::version::{Below, DebugVersion, Minimum};

mod capture_opts;
mod error;
mod input_button;
mod version;

pub struct RenderDoc<V = V160> {
    api: *mut RawRenderDoc,
    _min_version: PhantomData<V>,
}

impl<V: Version> RenderDoc<V> {
    /// Initializes a new instance of the RenderDoc API.
    ///
    /// Note that RenderDoc will usually provide a higher API version than the one requested by the
    /// user, provided it is backwards compatible.
    pub fn new() -> Result<Self, Error> {
        Ok(RenderDoc {
            api: V::load()?,
            _min_version: PhantomData,
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
                _min_version: PhantomData,
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

    /// Configures how RenderDoc behaves on capture.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use renderdoc::Error;
    /// # fn main() -> Result<(), Error> {
    /// use std::time::Duration;
    /// use renderdoc::RenderDoc;
    ///
    /// let mut renderdoc: RenderDoc = RenderDoc::new()?;
    /// renderdoc
    ///     .set_capture_options()
    ///     .allow_vsync(false)
    ///     .delay_for_debugger(Duration::from_secs(3));
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_capture_options(&mut self) -> SetCaptureOptions<'_, V> {
        SetCaptureOptions {
            api: self.api,
            _min_version: PhantomData,
        }
    }

    /// Gets the current values of RenderDoc capture options.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use renderdoc::Error;
    /// # fn main() -> Result<(), Error> {
    /// use renderdoc::RenderDoc;
    ///
    /// let renderdoc: RenderDoc = RenderDoc::new()?;
    /// if renderdoc.capture_options().allow_vsync() {
    ///     // vsync is allowed
    /// } else {
    ///     // vsync is not allowed
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn capture_options(&self) -> CaptureOptions<'_, V> {
        CaptureOptions {
            api: self.api,
            _min_version: PhantomData,
        }
    }

    /// Sets which key(s) should be used to toggle focus between multiple windows.
    ///
    /// If `keys` contains no items, focus toggling will be disabled entirely.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use renderdoc::Error;
    /// # fn main() -> Result<(), Error> {
    /// use renderdoc::{InputButton, RenderDoc};
    ///
    /// let mut renderdoc: RenderDoc = RenderDoc::new()?;
    ///
    /// // Map "F" as the focus toggle key.
    /// renderdoc.set_focus_toggle_keys(InputButton::F);
    /// // Map both "F" and "T" as focus toggle keys.
    /// renderdoc.set_focus_toggle_keys([InputButton::F, InputButton::T]);
    /// // Disable focus toggling altogether.
    /// renderdoc.set_focus_toggle_keys(None);
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_focus_toggle_keys<K>(&mut self, keys: K)
    where
        K: AsInputButtons,
    {
        unsafe {
            (self.api.SetFocusToggleKeys.unwrap())(keys.as_ptr() as *mut _, keys.len());
        }
    }

    /// Sets which key(s) should be used to capture the next frame.
    ///
    /// If `keys` contains no items, capture keys will be disabled entirely.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use renderdoc::Error;
    /// # fn main() -> Result<(), Error> {
    /// use renderdoc::{InputButton, RenderDoc};
    ///
    /// let mut renderdoc: RenderDoc = RenderDoc::new()?;
    ///
    /// // Map "C" as the trigger capture key.
    /// renderdoc.set_capture_keys(InputButton::C);
    /// // Map both "C" and "T" as trigger capture keys.
    /// renderdoc.set_capture_keys([InputButton::C, InputButton::T]);
    /// // Disable capture keys altogether.
    /// renderdoc.set_capture_keys(None);
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_capture_keys<K>(&mut self, keys: K)
    where
        K: AsInputButtons,
    {
        unsafe {
            (self.api.SetCaptureKeys.unwrap())(keys.as_ptr() as *mut _, keys.len());
        }
    }
}

impl<V: Minimum<V100> + Below<V141>> RenderDoc<V> {
    /// Attempts to shut down RenderDoc.
    ///
    /// # Safety
    ///
    /// Note that this will only work correctly if done _immediately_ after the dynamic library is
    /// loaded, before any API work happens. At that point, RenderDoc will remove its injected hooks
    /// and shut down. Behavior is undefined if this is called after any API functions have been
    /// called.
    ///
    /// # Compatibility
    ///
    /// This process is only possible on Windows, and even then it is not well defined so may not be
    /// possible in all circumstances. This method is provided at your own risk.
    ///
    /// Since version 1.4.1, this method has been renamed to [`remove_hooks`].
    #[cfg(windows)]
    pub unsafe fn shutdown(self) {
        ((*self.api).__bindgen_anon_1.Shutdown.unwrap())();
    }
}

impl<V: Minimum<V141>> RenderDoc<V> {
    /// Attempts to remove RenderDoc's hooks in the application.
    ///
    /// # Safety
    ///
    /// Note that this will only work correctly if done _immediately_ after the dynamic library is
    /// loaded, before any API work happens. At that point, RenderDoc will remove its injected hooks
    /// and shut down. Behavior is undefined if this is called after any API functions have been
    /// called.
    ///
    /// # Compatibility
    ///
    /// This process is only possible on Windows, and even then it is not well defined so may not be
    /// possible in all circumstances. This method is provided at your own risk.
    ///
    /// Prior to version 1.4.1, this method was named [`shutdown`].
    #[cfg(windows)]
    pub unsafe fn remove_hooks(self) {
        ((*self.api).__bindgen_anon_1.RemoveHooks.unwrap())();
    }
}

unsafe impl<V> Send for RenderDoc<V> {}

impl<V: Version> Debug for RenderDoc<V> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct(stringify!(RenderDoc))
            .field("min_version", &DebugVersion(V::VERSION))
            .finish()
    }
}
