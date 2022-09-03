//! # `PEP-440` Developmental release identifier

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, PartialOrd)]
pub struct DevHead {
    pub dev_num: Option<u32>,
}
