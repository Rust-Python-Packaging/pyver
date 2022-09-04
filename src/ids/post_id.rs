use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

/// # `PEP-440` Post-Release identifier
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct PostHeader {
    pub post_head: Option<PostHead>,
    pub post_num: Option<u32>,
}

/// `PEP-440` Post-Release Identifier Keyword
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum PostHead {
    Post,
    Rev,
}

impl PartialOrd for PostHead {
    fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
        Some(Ordering::Equal)
    }
}

impl PartialOrd for PostHeader {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.post_num == other.post_num {
            return Some(Ordering::Equal);
        }

        if self.post_num.is_none() && other.post_num.is_some() {
            return Some(Ordering::Less);
        } else if self.post_num.is_some() && other.post_num.is_none() {
            return Some(Ordering::Greater);
        }

        if self.post_num < other.post_num {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PostHead;
    use super::PostHeader;

    #[test]
    fn test_post_ordering() {
        assert!(
            PostHeader {
                post_head: Some(PostHead::Post),
                post_num: Some(0),
            } > PostHeader {
                post_head: Some(PostHead::Post),
                post_num: None,
            }
        );
        assert!(
            PostHeader {
                post_head: Some(PostHead::Post),
                post_num: Some(1),
            } > PostHeader {
                post_head: Some(PostHead::Post),
                post_num: Some(0),
            }
        );
    }
}
