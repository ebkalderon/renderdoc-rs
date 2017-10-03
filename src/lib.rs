//! Bindings to RenderDoc in-application API.

#![deny(missing_docs)]

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate lazy_static;
extern crate libc;
extern crate shared_library;

#[cfg(target_os = "windows")]
extern crate winapi;

pub mod app;
