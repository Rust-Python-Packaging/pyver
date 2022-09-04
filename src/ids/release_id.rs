use serde::{Deserialize, Serialize};

/// `PEP-440` Release numbers
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, PartialOrd)]
pub struct ReleaseHeader {
    /// Major release such as 1.0 or breaking changes
    pub major: u32,
    /// Minor release Such as new functionality
    pub minor: u32,
}

#[cfg(test)]
mod test {
    use super::ReleaseHeader;

    #[test]
    fn test_release_ordering() {
        assert!(
            ReleaseHeader { major: 1, minor: 0 } > ReleaseHeader { major: 0, minor: 0 }
        );
        assert!(
            ReleaseHeader { major: 1, minor: 1 } > ReleaseHeader { major: 1, minor: 0 }
        );
        assert!(
            ReleaseHeader { major: 2, minor: 1 }
                > ReleaseHeader {
                    major: 1,
                    minor: 52,
                }
        );
    }
}
