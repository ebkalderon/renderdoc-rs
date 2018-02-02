//! Raw FFI bindings [RenderDoc], a popular graphics debugger.
//!
//! [RenderDoc]: https://renderdoc.org/

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cfg(test)]
#[macro_use]
extern crate assert_approx_eq;

#[cfg(feature = "app")]
pub mod app;

#[cfg(feature = "replay")]
pub mod replay;
