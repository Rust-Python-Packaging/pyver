use serde::{Deserialize, Serialize};

/// # `PEP-440` Developmental release identifier
/// This identifier is used to mark a developmental release
///
/// Examples of versions that use this struct:
/// - `1.0.dev456`
/// - `1.0rc1.dev1`
///
/// ## Example Usage
/// ```
/// use pyver::ids::DevHead;
///
/// assert!(
///     DevHead { dev_num: Some(0) }
///     >
///     DevHead { dev_num: None }
/// );
/// ```
#[derive(
    Hash, Ord, Clone, Debug, Serialize, Deserialize, Eq, PartialEq, PartialOrd,
)]
pub struct DevHead {
    pub dev_num: Option<u32>,
}

#[cfg(test)]
mod tests {
    use super::DevHead;

    #[test]
    fn test_dev_ordering() {
        assert!(DevHead { dev_num: Some(0) } > DevHead { dev_num: None });
        assert!(DevHead { dev_num: Some(1) } > DevHead { dev_num: Some(0) });
    }
}
