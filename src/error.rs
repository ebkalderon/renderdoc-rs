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

    pub(crate) fn multiple_instances() -> Self {
        Error(ErrorKind::MultipleInstances)
    }

    pub(crate) fn non_utf8_path() -> Self {
        Error(ErrorKind::NonUtf8Path)
    }

    pub(crate) fn launch_replay_ui() -> Self {
        Error(ErrorKind::LaunchReplayUi)
    }

    pub(crate) fn set_capture_options(opt: RENDERDOC_CaptureOption, val: u32) -> Self {
        Error(ErrorKind::SetCaptureOptions(opt, val))
    }

    pub(crate) fn get_capture_options(opt: RENDERDOC_CaptureOption) -> Self {
        Error(ErrorKind::GetCaptureOptions(opt))
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.0 {
            ErrorKind::Library(_) => f.write_str("Unable to load RenderDoc shared library"),
            ErrorKind::Symbol(_) => f.write_str("Unable to find `RENDERDOC_GetAPI` symbol"),
            ErrorKind::NoCompatibleApi => f.write_str("Library could not provide compatible API"),
            ErrorKind::MultipleInstances => f.write_str("Multiple API instances are not permitted"),
            ErrorKind::NonUtf8Path => f.write_str("RenderDoc only accepts UTF-8 paths"),
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
            ErrorKind::GetCaptureOptions(opt) => {
                if f.alternate() {
                    write!(f, "Invalid capture option {}", opt)
                } else {
                    f.write_str("Invalid capture option")
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
    MultipleInstances,
    LaunchReplayUi,
    SetCaptureOptions(RENDERDOC_CaptureOption, u32),
    GetCaptureOptions(RENDERDOC_CaptureOption),
    NonUtf8Path,
}
