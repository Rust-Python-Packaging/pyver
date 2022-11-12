use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

/// # `PEP-440` Post-Release identifier
/// This identifier is used to mark a Post Release/Revision Version
///
/// Examples of versions that use this struct:
/// - `1.0.post456`
/// - `1.0rev`
///
/// ## Example Usage
/// ```
/// use pyver::ids::PostHeader;
/// use pyver::ids::PostHead;
///
/// assert!(
///     PostHeader {
///         post_head: Some(PostHead::Post),
///         post_num: Some(0),
///     } > PostHeader {
///         post_head: Some(PostHead::Post),
///         post_num: None,
///     }
/// );
/// ```
#[derive(Hash, Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct PostHeader {
    pub post_head: Option<PostHead>,
    pub post_num: Option<u32>,
}

/// `PEP-440` Post-Release Identifier Keyword
/// This is a helper enum to tack whether it's a Revision or
/// a Post-Release
///
/// Examples of versions that use this enum:
/// - `1.0.post456`
/// - `1.0rev`
#[derive(Hash, Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum PostHead {
    /// ```
    /// use pyver::ids::PostHead;
    ///
    /// PostHead::Post;
    /// ```
    Post,
    /// ```
    /// use pyver::ids::PostHead;
    ///
    /// PostHead::Rev;
    /// ```
    Rev,
}

impl Ord for PostHead {
    fn cmp(&self, _other: &Self) -> Ordering {
        Ordering::Equal
    }
}

impl PartialOrd for PostHead {
    fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
        Some(Ordering::Equal)
    }
}

impl PartialOrd<Self> for PostHeader {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PostHeader {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.post_num == other.post_num {
            return Ordering::Equal;
        }

        if self.post_num.is_none() && other.post_num.is_some() {
            return Ordering::Less;
        } else if self.post_num.is_some() && other.post_num.is_none() {
            return Ordering::Greater;
        }

        if self.post_num < other.post_num {
            Ordering::Less
        } else {
            Ordering::Greater
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
