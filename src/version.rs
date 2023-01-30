use std::ffi::c_uint;
use std::fmt::{self, Debug, Formatter};

use renderdoc_sys::RENDERDOC_Version;

pub trait Version {
    const VERSION: RENDERDOC_Version;
}

pub trait Minimum<V: Version> {}
pub trait Below<V: Version> {}

pub enum V100 {}
pub enum V101 {}
pub enum V102 {}
pub enum V110 {}
pub enum V111 {}
pub enum V112 {}
pub enum V120 {}
pub enum V130 {}
pub enum V140 {}
pub enum V141 {}
pub enum V142 {}
pub enum V150 {}
pub enum V160 {}

macro_rules! define_versions {
    // Entry point into the macro.
    ($($name:ident => $version_code:ident),+) => {
        $(define_versions!(@version $name $version_code);)+

        define_versions!(@minimum $($name)+);
        define_versions!(@below $($name)+);
    };

    // Implements the `Version` trait for `$name`.
    (@version $name:ident $version_code:ident) => {
        impl Version for $name {
            const VERSION: RENDERDOC_Version = renderdoc_sys::$version_code;
        }
    };

    // Implements the `Minimum<V>` trait for all version enums.
    (@minimum $first:ident $($rest:ident)*) => {
        impl<V: Version> Minimum<$first> for V {}

        define_versions!(@minimum_rec $($rest)*);
    };

    (@minimum_rec $first:ident $second:ident $($rest:ident)*) => {
        impl Minimum<$first> for $first {}

        impl<V: Minimum<$second>> Minimum<$first> for V {}

        define_versions!(@minimum_rec $second $($rest)*);
    };

    (@minimum_rec $single:ident) => {
        impl Minimum<$single> for $single {}
    };

    // Implements the `Below<V>` trait for all version enums.
    (@below $($name:ident)*) => {
        define_versions!(@below_rev [$($name)*]);
    };

    (@below_rev [$first:ident $($rest:ident)*] $($reversed:ident)*) => {
        define_versions!(@below_rev [$($rest)*] $first $($reversed)*);
    };

    (@below_rev [] $($reversed:ident)*) => {
        define_versions!(@below_rec $($reversed)*);
    };

    (@below_rec $first:ident $second:ident $($rest:ident)+) => {
        impl Below<$first> for $second {}

        impl<V: Below<$second>> Below<$first> for V {}

        define_versions!(@below_rec $second $($rest)*);
    };

    (@below_rec $first:ident $second:ident) => {
        impl Below<$first> for $second {}
    };
}

define_versions! {
    V100 => eRENDERDOC_API_Version_1_0_0,
    V101 => eRENDERDOC_API_Version_1_0_1,
    V102 => eRENDERDOC_API_Version_1_0_2,
    V110 => eRENDERDOC_API_Version_1_1_0,
    V111 => eRENDERDOC_API_Version_1_1_1,
    V112 => eRENDERDOC_API_Version_1_1_2,
    V120 => eRENDERDOC_API_Version_1_2_0,
    V130 => eRENDERDOC_API_Version_1_3_0,
    V140 => eRENDERDOC_API_Version_1_4_0,
    V141 => eRENDERDOC_API_Version_1_4_1,
    V142 => eRENDERDOC_API_Version_1_4_2,
    V150 => eRENDERDOC_API_Version_1_5_0,
    V160 => eRENDERDOC_API_Version_1_6_0
}

/// Encodes the given RenderDoc version into a valid `RENDERDOC_Version`.
#[inline]
pub const fn from_digits(major: u8, minor: u8, patch: u8) -> RENDERDOC_Version {
    let mut version = major as c_uint * 10_000;
    version += minor as c_uint * 100;
    version += patch as c_uint;
    version
}

/// Decodes the given `RENDERDOC_Version` into major, minor, and patch digits.
#[inline]
const fn into_digits(ver: RENDERDOC_Version) -> (u8, u8, u8) {
    let patch = ver % 100;
    let minor = ver % 10_000 / 100;
    let major = ver / 10_000;
    (major as u8, minor as u8, patch as u8)
}

/// Newtype used to pretty-print a RenderDoc version.
pub struct DebugVersion(pub RENDERDOC_Version);

impl Debug for DebugVersion {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let (major, minor, patch) = into_digits(self.0);
        write!(f, "{}.{}.{}", major, minor, patch)
    }
}

#[cfg(test)]
mod tests {
    use renderdoc_sys::*;

    use super::*;

    fn versions() -> impl IntoIterator<Item = ((u8, u8, u8), RENDERDOC_Version)> {
        [
            ((1, 0, 0), eRENDERDOC_API_Version_1_0_0),
            ((1, 0, 1), eRENDERDOC_API_Version_1_0_1),
            ((1, 0, 2), eRENDERDOC_API_Version_1_0_2),
            ((1, 1, 0), eRENDERDOC_API_Version_1_1_0),
            ((1, 1, 1), eRENDERDOC_API_Version_1_1_1),
            ((1, 1, 2), eRENDERDOC_API_Version_1_1_2),
            ((1, 2, 0), eRENDERDOC_API_Version_1_2_0),
            ((1, 3, 0), eRENDERDOC_API_Version_1_3_0),
            ((1, 4, 0), eRENDERDOC_API_Version_1_4_0),
            ((1, 4, 1), eRENDERDOC_API_Version_1_4_1),
            ((1, 4, 2), eRENDERDOC_API_Version_1_4_2),
            ((1, 5, 0), eRENDERDOC_API_Version_1_5_0),
            ((1, 6, 0), eRENDERDOC_API_Version_1_6_0),
        ]
    }

    #[test]
    fn encodes_version_from_digits() {
        for (input_digits, expected_version) in versions() {
            let (major, minor, patch) = input_digits;
            let computed_version = from_digits(major, minor, patch);
            assert_eq!(computed_version, expected_version);
        }
    }

    #[test]
    fn decodes_digits_from_version() {
        for (expected_digits, input_version) in versions() {
            let computed_digits = into_digits(input_version);
            assert_eq!(computed_digits, expected_digits);
        }
    }

    #[test]
    fn round_trip_from_into_digits() {
        let computed_versions = versions()
            .into_iter()
            .map(|((major, minor, patch), _)| from_digits(major, minor, patch));

        for (computed_version, (expected_digits, _)) in computed_versions.zip(versions()) {
            let computed_digits = into_digits(computed_version);
            assert_eq!(computed_digits, expected_digits);
        }
    }

    #[test]
    fn produces_reasonable_debug_output() {
        assert_eq!(
            format!("{:?}", DebugVersion(eRENDERDOC_API_Version_1_4_2)),
            "1.4.2"
        );
    }
}
