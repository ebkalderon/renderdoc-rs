use std::cmp::Ordering;
use std::fmt::{self, Binary, Debug, Formatter};
use std::marker::PhantomData;
use std::ops::{BitAnd, BitOr, Not};

use renderdoc_sys::RENDERDOC_OverlayBits;

use crate::{Minimum, V100};

/// Bit flags for configuring the RenderDoc overlay.
///
/// These flags may be passed as arguments to
/// [`set_overlay_bits()`](crate::RenderDoc::set_overlay_bits) and
/// [`clear_overlay_bits`](crate::RenderDoc::clear_overlay_bits) to configure the overlay, or
/// returned from [`overlay_bits()`](crate::RenderDoc::overlay_bits) to inspect the current overlay
/// configuration.
///
/// The overlay state is set to [`OverlayBits::DEFAULT`] on startup.
pub struct OverlayBits<V = V100> {
    pub(super) bits: RENDERDOC_OverlayBits,
    _min_version: PhantomData<V>,
}

impl<V: Minimum<V100>> OverlayBits<V> {
    /// Enable the debug overlay globally.
    pub const ENABLED: Self = Self::from_bits(renderdoc_sys::eRENDERDOC_Overlay_Enabled);

    /// Show the frame rate counter, along with average, minimum, maximum frame times (measured in
    /// milliseconds).
    pub const FRAME_RATE: Self = Self::from_bits(renderdoc_sys::eRENDERDOC_Overlay_FrameRate);

    /// Show the current frame number.
    pub const FRAME_NUMBER: Self = Self::from_bits(renderdoc_sys::eRENDERDOC_Overlay_FrameNumber);

    /// Show a list of recent frame captures, out of the total number of captures made.
    pub const CAPTURE_LIST: Self = Self::from_bits(renderdoc_sys::eRENDERDOC_Overlay_CaptureList);

    /// The default configuration of the overlay at startup.
    ///
    /// At present, this includes [`ENABLED`](Self::ENABLED), [`FRAME_RATE`](Self::FRAME_RATE),
    /// [`FRAME_NUMBER`](Self::FRAME_NUMBER), and [`CAPTURE_LIST`](Self::CAPTURE_LIST).
    pub const DEFAULT: Self = Self::from_bits(renderdoc_sys::eRENDERDOC_Overlay_Default);
}

impl<V> OverlayBits<V> {
    /// Convert from underlying bit representation, preserving all bits (even those not
    /// corresponding to a defined flag).
    const fn from_bits(bits: RENDERDOC_OverlayBits) -> Self {
        OverlayBits {
            bits,
            _min_version: PhantomData,
        }
    }

    /// Convert from underlying bit representation, dropping any bits that do not correspond to flags.
    pub(super) const fn from_bits_truncate(bits: RENDERDOC_OverlayBits) -> Self {
        use renderdoc_sys::*;

        OverlayBits {
            bits: (bits & eRENDERDOC_Overlay_Enabled)
                | (bits & eRENDERDOC_Overlay_FrameRate)
                | (bits & eRENDERDOC_Overlay_FrameNumber)
                | (bits & eRENDERDOC_Overlay_CaptureList)
                | (bits & eRENDERDOC_Overlay_Default),
            _min_version: PhantomData,
        }
    }

    /// Equivalent to having all bits enabled.
    #[inline]
    pub const fn all() -> Self {
        OverlayBits::from_bits(RENDERDOC_OverlayBits::MAX)
    }

    /// Equivalent to having all bits disabled.
    #[inline]
    pub const fn empty() -> Self {
        OverlayBits::from_bits(RENDERDOC_OverlayBits::MIN)
    }

    /// Returns `true` all of the flags in `other` are contained within `self`.
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        self.bits & other.bits == other.bits
    }

    /// Returns `true` if this corresponds to the default overlay configuration.
    #[inline]
    pub const fn is_default(self) -> bool {
        use renderdoc_sys::eRENDERDOC_Overlay_Default;
        self.bits & eRENDERDOC_Overlay_Default == eRENDERDOC_Overlay_Default
    }

    /// Returns `true` if no flags are stored.
    #[inline]
    pub const fn is_empty(self) -> bool {
        self.bits == RENDERDOC_OverlayBits::MIN
    }

    /// Returns `true` if all flags are set.
    #[inline]
    pub const fn is_all(self) -> bool {
        self.bits == RENDERDOC_OverlayBits::MAX
    }

    /// Iterate over all enabled flag values.
    pub fn iter(self) -> impl Iterator<Item = OverlayBits<V>> {
        [
            renderdoc_sys::eRENDERDOC_Overlay_Enabled,
            renderdoc_sys::eRENDERDOC_Overlay_FrameRate,
            renderdoc_sys::eRENDERDOC_Overlay_FrameNumber,
            renderdoc_sys::eRENDERDOC_Overlay_CaptureList,
        ]
        .iter()
        .copied()
        .map(OverlayBits::from_bits)
        .filter(move |&flag| self.contains(flag))
    }
}

impl<V> Binary for OverlayBits<V> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:b}", self.bits)
    }
}

/// Returns the intersection between the two sets of flags.
impl<V> BitAnd for OverlayBits<V> {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        OverlayBits::from_bits(self.bits & rhs.bits)
    }
}

/// Returns the union of the two sets of flags.
impl<V> BitOr for OverlayBits<V> {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        OverlayBits::from_bits(self.bits | rhs.bits)
    }
}

impl<V> Clone for OverlayBits<V> {
    #[inline]
    fn clone(&self) -> Self {
        OverlayBits::from_bits(self.bits)
    }
}

impl<V> Copy for OverlayBits<V> {}

impl<V> Debug for OverlayBits<V> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use renderdoc_sys::*;

        let flags: Vec<_> = [
            (eRENDERDOC_Overlay_Enabled, "ENABLED"),
            (eRENDERDOC_Overlay_FrameRate, "FRAME_RATE"),
            (eRENDERDOC_Overlay_FrameNumber, "FRAME_NUMBER"),
            (eRENDERDOC_Overlay_CaptureList, "CAPTURE_LIST"),
        ]
        .iter()
        .filter(|&(flag, _)| self.bits & flag == *flag)
        .map(|&(_, s)| s)
        .collect();

        if flags.is_empty() {
            f.write_str("(empty)")
        } else {
            f.write_str(&flags.join(" | "))
        }
    }
}

/// Returns the complement of this set of flags.
impl<V> Not for OverlayBits<V> {
    type Output = Self;

    #[inline]
    fn not(self) -> Self::Output {
        OverlayBits::from_bits(!self.bits)
    }
}

impl<V> PartialOrd for OverlayBits<V> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.bits.partial_cmp(&other.bits)
    }
}

impl<V> Ord for OverlayBits<V> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.bits.cmp(&other.bits)
    }
}

impl<V1, V2> PartialEq<OverlayBits<V2>> for OverlayBits<V1> {
    #[inline]
    fn eq(&self, rhs: &OverlayBits<V2>) -> bool {
        self.bits == rhs.bits
    }
}

impl<V> Eq for OverlayBits<V> {}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use renderdoc_sys::*;

    use super::*;
    use crate::V101;

    #[test]
    fn from_bits_truncate_works() {
        let invalid_flag = 0b10000;
        let bits = eRENDERDOC_Overlay_CaptureList | invalid_flag;
        let truncated = OverlayBits::<V100>::from_bits_truncate(bits);
        assert_eq!(truncated.bits & invalid_flag, 0);
        assert_eq!(
            truncated.bits & eRENDERDOC_Overlay_CaptureList,
            eRENDERDOC_Overlay_CaptureList
        );
    }

    #[test]
    fn iter_works() {
        let flags: BTreeSet<OverlayBits<V100>> = (OverlayBits::FRAME_RATE | OverlayBits::ENABLED)
            .iter()
            .collect();

        assert_eq!(
            flags,
            BTreeSet::from([OverlayBits::<V100>::ENABLED, OverlayBits::FRAME_RATE])
        );

        let all: BTreeSet<OverlayBits<V100>> = OverlayBits::all().iter().collect();

        assert_eq!(
            all,
            BTreeSet::from([
                OverlayBits::<V100>::ENABLED,
                OverlayBits::FRAME_RATE,
                OverlayBits::FRAME_NUMBER,
                OverlayBits::CAPTURE_LIST
            ])
        );
    }

    #[test]
    fn bitand_works() {
        let flags: OverlayBits<V100> = OverlayBits::FRAME_RATE & OverlayBits::FRAME_NUMBER;
        assert_eq!(
            flags.bits,
            eRENDERDOC_Overlay_FrameRate & eRENDERDOC_Overlay_FrameNumber
        );
    }

    #[test]
    fn bitor_works() {
        let flags: OverlayBits<V100> = OverlayBits::ENABLED | OverlayBits::FRAME_RATE;
        assert_eq!(
            flags.bits,
            eRENDERDOC_Overlay_Enabled | eRENDERDOC_Overlay_FrameRate
        );
    }

    #[test]
    fn bitnot_works() {
        let flags: OverlayBits<V100> = !OverlayBits::CAPTURE_LIST;
        assert_eq!(flags.bits, !eRENDERDOC_Overlay_CaptureList);
    }

    #[test]
    fn fmt_binary_works() {
        let flags: OverlayBits<V100> = OverlayBits::ENABLED | OverlayBits::CAPTURE_LIST;
        assert_eq!(format!("{:b}", flags), format!("{:b}", flags.bits));
    }

    #[test]
    fn fmt_debug_works() {
        let flags: OverlayBits<V100> = OverlayBits::FRAME_NUMBER | OverlayBits::CAPTURE_LIST;
        assert_eq!(format!("{:?}", flags), "FRAME_NUMBER | CAPTURE_LIST");

        let empty: OverlayBits<V100> = OverlayBits::empty();
        assert_eq!(format!("{:?}", empty), "(empty)");

        let all: OverlayBits<V100> = OverlayBits::all();
        assert_eq!(
            format!("{:?}", all),
            "ENABLED | FRAME_RATE | FRAME_NUMBER | CAPTURE_LIST"
        );
    }

    #[test]
    fn partial_eq_with_same_version() {
        let first: OverlayBits<V100> = OverlayBits::ENABLED;
        let second: OverlayBits<V100> = OverlayBits::ENABLED;
        assert_eq!(first, second);
        assert_eq!(second, first);
    }

    #[test]
    fn partial_eq_with_different_versions() {
        let first: OverlayBits<V100> = OverlayBits::ENABLED;
        let second: OverlayBits<V101> = OverlayBits::ENABLED;
        assert_eq!(first, second);
        assert_eq!(second, first);
    }
}
