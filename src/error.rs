//! Common library error types.

use std::fmt::{self, Display, Formatter};

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
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.0 {
            ErrorKind::Library(_) => write!(f, "Unable to load RenderDoc shared library"),
            ErrorKind::Symbol(_) => write!(f, "Unable to find `RENDERDOC_GetAPI` symbol"),
            ErrorKind::NoCompatibleApi => write!(f, "Library could not provide compatible API"),
            ErrorKind::LaunchReplayUi => write!(f, "Failed to launch replay UI"),
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
}
