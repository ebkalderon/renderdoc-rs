use std::convert::TryFrom;
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::ptr;
use std::time::{self, Duration, SystemTime};

use crate::{Error, FunctionTable, Minimum, V100, V120};

/// A frame capture file saved to disk.
///
/// This struct is created by [`Captures::get`]. See its documentation for more.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Capture {
    /// Absolute path to where the capture file can be found.
    pub path: PathBuf,
    /// Local time when the capture was taken.
    pub timestamp: SystemTime,
}

/// A list of frame captures created by RenderDoc.
///
/// This struct is created by the [`captures`](crate::RenderDoc::captures) method on
/// [`RenderDoc<V>`](crate::RenderDoc).
pub struct Captures<'api, V> {
    api: &'api mut FunctionTable,
    _min_version: PhantomData<V>,
}

impl<'api, V> Captures<'api, V> {
    pub(super) fn new(api: &'api mut FunctionTable) -> Self {
        Captures {
            api,
            _min_version: PhantomData,
        }
    }
}

impl<'api, V: Minimum<V100>> Captures<'api, V> {
    /// Returns the total number of captures that have been made.
    pub fn len(&self) -> usize {
        unsafe { (self.api.GetNumCaptures.unwrap())() as usize }
    }

    /// Returns the details of the _N_-th capture, by index.
    ///
    /// Like most indexing operations, the first capture starts with index `0` and increments from
    /// there. If a frame capture with the given index exists, a [`Capture`] is returned
    /// containing:
    ///
    /// 1. The absolute path to the capture file on disk
    /// 2. The time when the capture was taken
    ///
    /// # Notes
    ///
    /// The path to the capture file is _not_ guaranteed to exist. When captures are deleted in the
    /// UI, they will still appear in this list.
    pub fn get(&self, index: usize) -> Option<Capture> {
        let index = u32::try_from(index).ok()?;
        let path_len = self.check_exists(index)?;

        let mut buf = Vec::with_capacity(path_len);
        let mut time = 0u64;

        let capture_exists = unsafe {
            let null = ptr::null_mut();
            (self.api.GetCapture.unwrap())(index, buf.as_mut_ptr() as *mut _, null, &mut time) == 1
        };

        debug_assert!(capture_exists, "capture must exist, as checked above");

        let path = CString::from_vec_with_nul(buf)
            .expect("RenderDoc should have written null terminator")
            .into_string()
            .expect("RenderDoc should have written valid UTF-8");

        Some(Capture {
            path: path.into(),
            timestamp: time::UNIX_EPOCH + Duration::from_secs(time),
        })
    }

    /// Checks whether a capture file exists at the given `index`.
    ///
    /// Returns the length of its file path (in bytes) if it exists, or `None` otherwise.
    fn check_exists(&self, index: u32) -> Option<usize> {
        let mut path_len = 0u32;
        let null = ptr::null_mut();

        unsafe {
            if (self.api.GetCapture.unwrap())(index, null, &mut path_len, null as *mut _) == 1 {
                Some(path_len as usize)
            } else {
                None
            }
        }
    }

    /// Returns an iterator over all frame captures.
    pub fn iter(&self) -> CapturesIter<'_, V> {
        CapturesIter {
            caps: MaybeBorrowed::Borrowed(self),
            index: 0,
        }
    }

    /// Returns the path template where new captures will be stored.
    ///
    /// The template can either be a relative or absolute path, which determines where captures
    /// will be saved and how they will be named. Relative paths will be saved relative to the
    /// process' current working directory.
    ///
    /// By default, this will be in a folder controlled by the UI - initially the system temporary
    /// directory, and the filename is the executable's filename.
    ///
    /// # Examples
    ///
    /// ```
    /// # use renderdoc::Error;
    /// # fn main() -> Result<(), Error> {
    /// use renderdoc::{RenderDoc, V100};
    ///
    /// let mut renderdoc: RenderDoc<V100> = RenderDoc::new()?;
    /// println!("{:?}", renderdoc.captures().path_template()); // e.g. `my_captures/example`
    /// # Ok(())
    /// # }
    /// ```
    pub fn path_template(&self) -> &Path {
        unsafe {
            let ptr = (self.api.__bindgen_anon_3.GetLogFilePathTemplate.unwrap())();
            CStr::from_ptr(ptr)
                .to_str()
                .map(Path::new)
                .expect("RenderDoc should have written valid UTF-8")
        }
    }

    /// Sets the path template where new capture files should be stored.
    ///
    /// The template can either be a relative or absolute path, and it determines where captures
    /// will be saved and how they will be named. Relative paths will be saved relative to the
    /// process' current working directory. Any extensions at the end of the path will be stripped.
    /// If the save directory does not exist it will be created, including any parent directories.
    ///
    /// The default template is in a folder controlled by the UI - initially the system temporary
    /// directory, and the filename is the executable's filename.
    ///
    /// Returns an error if `path_template` is not valid UTF-8.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use renderdoc::Error;
    /// # fn main() -> Result<(), Error> {
    /// use renderdoc::{RenderDoc, V100};
    ///
    /// let mut renderdoc: RenderDoc<V100> = RenderDoc::new()?;
    ///
    /// renderdoc.captures().set_path_template("my_captures/example").unwrap();
    ///
    /// renderdoc.trigger_capture(); // Saved as `my_captures/example_frame123.rdc`
    /// renderdoc.trigger_capture(); // Saved as `my_captures/example_frame456.rdc`
    ///
    /// assert_eq!(renderdoc.captures().path_template(), "my_captures/example");
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_path_template<P>(&mut self, path_template: P) -> Result<(), Error>
    where
        P: Into<PathBuf>,
    {
        let path = path_template
            .into()
            .into_os_string()
            .into_string()
            .map_err(|_| Error::non_utf8_path())
            .map(|utf8| CString::new(utf8).expect("Null in UTF-8 string is impossible"))?;

        unsafe {
            (self.api.__bindgen_anon_2.SetLogFilePathTemplate.unwrap())(path.as_ptr());
        }

        Ok(())
    }

    /// Adds or sets comments associated with a capture file, which will be displayed in the UI
    /// when opened.
    ///
    /// If the `capture_file` path value is empty, the most recent capture file is used.
    ///
    /// Returns an error if `capture_file` is not valid UTF-8.
    ///
    /// # Compatibility
    ///
    /// This method is only compatible with RenderDoc version 1.2.0 and newer.
    pub fn set_comments<P, C>(&mut self, capture_file: P, comments: C) -> Result<(), Error>
    where
        P: Into<PathBuf>,
        C: Into<String>,
        V: Minimum<V120>,
    {
        let path = capture_file
            .into()
            .into_os_string()
            .into_string()
            .map_err(|_| Error::non_utf8_path())
            .map(|s| CString::new(s).expect("Null in UTF-8 string is impossible"))?;

        let comments = CString::new(comments.into()).expect("Null in UTF-8 string is impossible");

        unsafe {
            (self.api.SetCaptureFileComments.unwrap())(path.as_ptr(), comments.as_ptr());
        }

        Ok(())
    }
}

impl<'api, V> IntoIterator for Captures<'api, V>
where
    V: Minimum<V100> + 'api,
{
    type Item = Capture;
    type IntoIter = CapturesIter<'api, V>;

    fn into_iter(self) -> Self::IntoIter {
        CapturesIter {
            caps: MaybeBorrowed::Owned(self),
            index: 0,
        }
    }
}

/// An iterator of RenderDoc frame captures.
///
/// This struct is created by [`Captures::iter()`]. See its documentation for more.
pub struct CapturesIter<'api, V> {
    caps: MaybeBorrowed<'api, Captures<'api, V>>,
    index: usize,
}

impl<V> Iterator for CapturesIter<'_, V>
where
    V: Minimum<V100>,
{
    type Item = Capture;

    fn next(&mut self) -> Option<Self::Item> {
        let capture = self.caps.get(self.index)?;
        self.index += 1;
        Some(capture)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (remaining_len(self.caps.len(), self.index), None)
    }

    fn count(self) -> usize {
        remaining_len(self.caps.len(), self.index)
    }

    fn last(mut self) -> Option<Self::Item> {
        match self.caps.len() {
            0 => None,
            count => {
                self.index = count;
                self.caps.get(self.index - 1)
            }
        }
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.index += n;
        self.next()
    }
}

enum MaybeBorrowed<'a, T> {
    Borrowed(&'a T),
    Owned(T),
}

impl<'a, T> Deref for MaybeBorrowed<'a, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        match self {
            MaybeBorrowed::Borrowed(v) => v,
            MaybeBorrowed::Owned(ref v) => v,
        }
    }
}

#[inline]
const fn remaining_len(num_captures: usize, next_index: usize) -> usize {
    num_captures.saturating_sub(next_index)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_remaining_len_correctly() {
        assert_eq!(remaining_len(0, 0), 0);
        assert_eq!(remaining_len(5, 1), 4);
    }

    #[test]
    fn remaining_len_does_not_panic() {
        assert_eq!(remaining_len(0, 100), 0);
    }
}
