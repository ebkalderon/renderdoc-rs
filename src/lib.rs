//! Bindings to RenderDoc in-application API.

#![deny(missing_docs)]

#[cfg(any(target_os = "macos", target_os = "ios"))]
compile_error!("RenderDoc does not support this platform.");

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate lazy_static;
extern crate shared_library;

#[cfg(target_os = "windows")]
extern crate winapi;
#[cfg(feature = "winit")]
extern crate winit;

pub mod app;
