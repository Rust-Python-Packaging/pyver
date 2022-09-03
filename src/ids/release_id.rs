use serde::{Deserialize, Serialize};

/// `PEP-440` Release numbers
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, PartialOrd)]
pub struct ReleaseHeader {
    /// Major release such as 1.0 or breaking changes
    pub major: u32,
    /// Minor release Such as new functionality
    pub minor: u32,
}