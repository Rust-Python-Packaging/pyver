//! # `PEP-440` Developmental release identifier

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, PartialOrd)]
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
