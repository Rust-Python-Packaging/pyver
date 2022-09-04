//! # `PEP-440` Pre-Release identifier

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, PartialOrd)]
pub enum PreHeader {
    /// Present in versions like 1.1beta1 or 1.0b1 both are represented the same way
    /// ```
    ///# use pyver::ids::PreHeader;
    ///
    /// PreHeader::Beta(Some(1));
    /// ```
    Beta(Option<u32>),
    /// Present in versions like 1.0alpha2 or 1.0a2 both are represented the same way
    /// ```
    ///# use pyver::ids::PreHeader;
    ///
    /// PreHeader::Alpha(Some(2));
    /// ```
    Alpha(Option<u32>),
    /// Present in versions like 1.1pre3
    /// ```
    ///# use pyver::ids::PreHeader;
    ///
    /// PreHeader::Preview(Some(3));
    /// ```
    Preview(Option<u32>),
    /// Present in versions like 1.1-rc-4 or 1.1c-4
    /// ```
    ///# use pyver::ids::PreHeader;
    ///
    /// PreHeader::ReleaseCandidate(Some(4));
    /// ```
    ReleaseCandidate(Option<u32>),
}

#[cfg(test)]
mod tests {
    use super::PreHeader;

    #[test]
    fn test_pre_ordering() {
        assert!(PreHeader::ReleaseCandidate(None) > PreHeader::Preview(None));
        assert!(PreHeader::Preview(None) > PreHeader::Alpha(None));
        assert!(PreHeader::Alpha(None) > PreHeader::Beta(None));

        assert!(
            PreHeader::ReleaseCandidate(Some(2)) > PreHeader::ReleaseCandidate(Some(1))
        );
        assert!(PreHeader::Preview(Some(50)) > PreHeader::Preview(Some(3)));
        assert!(PreHeader::Alpha(Some(504)) > PreHeader::Alpha(Some(0)));
        assert!(PreHeader::Beta(Some(1234)) > PreHeader::Beta(Some(1)));

        assert!(PreHeader::ReleaseCandidate(Some(1)) > PreHeader::Beta(Some(45067885)));
    }
}
