//! Portable wrapper types around platform specific window and device handles.

use std::os::raw::c_void;

#[cfg(windows)]
use wio::com::ComPtr;

/// Raw mutable pointer to the OS-provided window handle.
pub type WindowHandle = *const c_void;

/// Raw mutable pointer to the API's root handle.
///
/// For example, this could be a pointer to an `ID3D11Device`, `HGLRC`/`GLXContext`,
/// `ID3D12Device`, etc.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct DevicePointer(pub(crate) *const c_void);

impl From<*const c_void> for DevicePointer {
    fn from(ptr: *const c_void) -> Self {
        DevicePointer(ptr)
    }
}

impl From<*mut c_void> for DevicePointer {
    fn from(ptr: *mut c_void) -> Self {
        DevicePointer(ptr)
    }
}

#[cfg(windows)]
impl From<winapi::shared::windef::HGLRC> for DevicePointer {
    fn from(ctx: winapi::shared::windef::HGLRC) -> Self {
        DevicePointer(ctx as *mut _ as *const c_void)
    }
}

#[cfg(windows)]
impl From<*mut winapi::um::d3d11::ID3D11Device> for DevicePointer {
    fn from(ctx: *mut winapi::um::d3d11::ID3D11Device) -> Self {
        DevicePointer(ctx as *mut _ as *const c_void)
    }
}

#[cfg(windows)]
impl From<wio::com::ComPtr<winapi::um::d3d11::ID3D11Device>> for DevicePointer {
    fn from(ctx: wio::com::ComPtr<winapi::um::d3d11::ID3D11Device>) -> Self {
        DevicePointer(ctx.as_raw() as *mut _ as *const c_void)
    }
}

#[cfg(windows)]
impl From<*mut winapi::um::d3d12::ID3D12Device> for DevicePointer {
    fn from(ctx: *mut winapi::um::d3d12::ID3D12Device) -> Self {
        DevicePointer(ctx as *mut _ as *const c_void)
    }
}

#[cfg(windows)]
impl From<wio::com::ComPtr<winapi::um::d3d12::ID3D12Device>> for DevicePointer {
    fn from(ctx: wio::com::ComPtr<winapi::um::d3d12::ID3D12Device>) -> Self {
        DevicePointer(ctx.as_raw() as *mut _ as *const c_void)
    }
}

#[cfg(feature = "glutin")]
impl<'a, T: glutin::context::AsRawContext> From<&'a T> for DevicePointer {
    fn from(ctx: &'a T) -> Self {
        use glutin::context::RawContext;

        #[cfg(unix)]
        match ctx.raw_context() {
            RawContext::Egl(egl) => DevicePointer::from(egl),
            RawContext::Glx(glx) => DevicePointer::from(glx),
        }

        #[cfg(windows)]
        match ctx.raw_context() {
            RawContext::Egl(egl) => DevicePointer::from(egl),
            RawContext::Wgl(wgl) => DevicePointer::from(wgl),
        }
    }
}
