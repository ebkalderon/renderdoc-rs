//! Common library error types.

use std::fmt::{self, Display, Formatter};

use renderdoc_sys::RENDERDOC_CaptureOption;

/// Errors that can occur with the RenderDoc in-application API.
#[derive(Debug)]
pub struct Error(ErrorKind);

impl Error {
    pub(crate) fn library(cause: libloading::Error) -> Self {
        Error(ErrorKind::Library(cause))
    }

    pub(crate) fn symbol(cause: libloading::Error) -> Self {
        Error(ErrorKind::Symbol(cause))
    }

    pub(crate) fn no_compatible_api() -> Self {
        Error(ErrorKind::NoCompatibleApi)
    }

    pub(crate) fn launch_replay_ui() -> Self {
        Error(ErrorKind::LaunchReplayUi)
    }

    pub(crate) fn set_capture_options(opt: RENDERDOC_CaptureOption, val: u32) -> Self {
        Error(ErrorKind::SetCaptureOptions(opt, val))
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.0 {
            ErrorKind::Library(_) => f.write_str("Unable to load RenderDoc shared library"),
            ErrorKind::Symbol(_) => f.write_str("Unable to find `RENDERDOC_GetAPI` symbol"),
            ErrorKind::NoCompatibleApi => f.write_str("Library could not provide compatible API"),
            ErrorKind::LaunchReplayUi => f.write_str("Failed to launch replay UI"),
            ErrorKind::SetCaptureOptions(opt, val) => {
                if f.alternate() {
                    write!(
                        f,
                        "Invalid capture option {} or value {} out of range",
                        opt, val
                    )
                } else {
                    f.write_str("Invalid capture option or value, option left unchanged")
                }
            }
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self.0 {
            ErrorKind::Library(ref e) | ErrorKind::Symbol(ref e) => Some(e),
            _ => None,
        }
    }
}

#[derive(Debug)]
enum ErrorKind {
    Library(libloading::Error),
    Symbol(libloading::Error),
    NoCompatibleApi,
    LaunchReplayUi,
    SetCaptureOptions(RENDERDOC_CaptureOption, u32),
}
