use super::ids::{DevHead, PostHead, PostHeader, PreHeader, ReleaseHeader};
use super::validate_440_version;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::hash::{Hash, Hasher};

/// `PEP-440` Compliant versioning system
///
//# This struct is sorted so that PartialOrd
//# correctly interprets priority
//# Lower == More important
///
/// # Example Usage
/// ```
///# use pyver::PackageVersion;
/// let _ = PackageVersion::new("v1.0");
/// ```
#[derive(Ord, Clone, Derivative, Debug, Serialize, Deserialize)]
#[derivative(PartialOrd)]
pub struct PackageVersion {
    /// ## Original String
    /// Just holds the original string passed in when creating
    /// the `PackageVersion` as some formating data is lost
    /// when parsing the string
    #[derivative(PartialOrd = "ignore", PartialEq = "ignore")]
    pub original: String,

    /// ## `PEP-440` Local version identifier
    /// Local version sorting will have to be it's own issue
    /// since there are no limits to what a local version can be
    ///
    /// For those who can read regex here it is for the local version:
    /// `[a-z0-9]+(?:(?:[\-_.][a-z0-9]+)+)?`
    ///
    /// Here in Rulex:
    /// ```toml
    ///  ['a'-'z' '0'-'9']+
    ///  ((["-" "_" "."] ['a'-'z' '0'-'9']+)+)?
    /// ```
    #[derivative(PartialOrd = "ignore", PartialEq = "ignore")]
    pub local: Option<String>,

    /// ## `PEP-440` Developmental release identifier
    pub dev: Option<DevHead>,

    /// ## `PEP-440` Post-Release identifier
    pub post: Option<PostHeader>,

    /// ## `PEP-440` Pre-Release identifier
    pub pre: Option<PreHeader>,

    /// ## `PEP-440` Release number
    pub release: ReleaseHeader,

    /// ## `PEP-440` Version-Epoch
    pub epoch: Option<u32>,
}

impl PackageVersion {
    pub fn new(version: &str) -> Result<Self> {
        let version_match = validate_440_version(version)?;

        let epoch: Option<u32> = match version_match.name("epoch") {
            // Convert Epoch String to Epoch Number
            Some(v) => Some(v.as_str().parse::<u32>()?),
            None => None,
        };

        let release: ReleaseHeader = match version_match.name("release") {
            Some(v) => {
                // Does Release String contain minor version
                if v.as_str().contains('.') {
                    let split: Vec<&str> = v.as_str().split('.').into_iter().collect();
                    ReleaseHeader {
                        major: split[0].parse::<u32>()?,
                        minor: split[1].parse::<u32>()?,
                    }
                } else {
                    ReleaseHeader {
                        major: v.as_str().parse::<u32>()?,
                        minor: 0,
                    }
                }
            }
            // There always has to be at least a major version
            None => anyhow::bail!("Failed to decode version {}", version),
        };

        let pre: Option<PreHeader> = match version_match.name("pre") {
            Some(_) => {
                let pre_n = match version_match.name("pre_n") {
                    Some(v) => Some(v.as_str().parse::<u32>()?),
                    None => None,
                };

                // Should be safe to unwrap since we already checked if pre has a value
                // since pre_n has to exist
                match version_match.name("pre_l").unwrap().as_str() {
                    "alpha" => Some(PreHeader::Alpha(pre_n)),
                    "a" => Some(PreHeader::Alpha(pre_n)),
                    "beta" => Some(PreHeader::Beta(pre_n)),
                    "b" => Some(PreHeader::Beta(pre_n)),
                    "rc" => Some(PreHeader::ReleaseCandidate(pre_n)),
                    "c" => Some(PreHeader::ReleaseCandidate(pre_n)),
                    "preview" => Some(PreHeader::Preview(pre_n)),
                    "pre" => Some(PreHeader::Preview(pre_n)),
                    _ => None,
                }
            }
            None => None,
        };

        let post: Option<PostHeader> = match version_match.name("post") {
            Some(_) => {
                let post_num: Option<u32> = match version_match.name("post_n1") {
                    Some(v) => Some(v.as_str().parse::<u32>()?),
                    None => match version_match.name("post_n2") {
                        Some(v) => Some(v.as_str().parse::<u32>()?),
                        _ => None,
                    },
                };

                let post_head: Option<PostHead> = match version_match.name("post_l") {
                    Some(v) => {
                        match v.as_str() {
                            "post" => Some(PostHead::Post),
                            "rev" => Some(PostHead::Rev),
                            "r" => Some(PostHead::Rev),
                            // This branch Should be impossible (see regex-group post_l)
                            _ => None,
                        }
                    }
                    None => None,
                };

                Some(PostHeader {
                    post_head,
                    post_num,
                })
            }
            None => None,
        };

        let dev: Option<DevHead> = match version_match.name("dev") {
            Some(_) => {
                let dev_num = match version_match.name("dev_n") {
                    Some(v) => Some(v.as_str().parse::<u32>()?),
                    None => None,
                };
                Some(DevHead { dev_num })
            }
            None => None,
        };

        let local: Option<String> =
            version_match.name("local").map(|v| v.as_str().to_string());

        Ok(Self {
            original: version.to_string(),
            epoch,
            release,
            pre,
            post,
            dev,
            local,
        })
    }
}

impl fmt::Display for PackageVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.original)
    }
}

impl PartialEq<Self> for PackageVersion {
    fn eq(&self, other: &Self) -> bool {
        self.release == other.release
            && self.local == other.local
            && self.dev == other.dev
            && self.epoch == other.epoch
            && self.post == other.post
            && self.pre == other.pre
    }
}

impl Eq for PackageVersion {}

/// The hash of the `PackageVersion` is calculated only from the `original` field.
impl Hash for PackageVersion {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.release.hash(state);
        self.local.hash(state);
        self.dev.hash(state);
        self.epoch.hash(state);
        self.post.hash(state);
        self.pre.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use crate::PackageVersion;
    use anyhow::Result;
    use std::collections::hash_map::DefaultHasher;
    use std::collections::HashMap;
    use std::hash::{Hash, Hasher};

    fn default_hash<T: Hash>(value: &T) -> u64 {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        hasher.finish()
    }

    #[test]
    fn test_pep440_ordering() -> Result<()> {
        assert!(
            PackageVersion::new(
                "v1!1.0-preview-921.post-516.dev-241+yeah.this.is.the.problem.with.local.versions",
            )?
                >
                PackageVersion::new("1.0")?
        );
        Ok(())
    }

    #[test]
    fn test_pep440_equality() -> Result<()> {
        assert_eq!(
            PackageVersion::new("1.0a1")?,
            PackageVersion::new("1.0alpha1")?
        );
        assert_eq!(
            PackageVersion::new("1.0b")?,
            PackageVersion::new("1.0beta")?
        );
        assert_eq!(PackageVersion::new("1.0r")?, PackageVersion::new("1.0rev")?);
        assert_eq!(PackageVersion::new("1.0c")?, PackageVersion::new("1.0rc")?);
        assert_eq!(PackageVersion::new("v1.0")?, PackageVersion::new("1.0")?);
        Ok(())
    }

    #[test]
    fn test_pep440() {
        // list of every example mentioned in pep-440
        let versions = vec![
            "1.0",
            "v1.1",
            "2.0",
            "2013.10",
            "2014.04",
            "1!1.0",
            "1!1.1",
            "1!2.0",
            "2!1.0.pre0",
            "1.0.dev456",
            "1.0a1",
            "1.0a2.dev456",
            "1.0a12.dev456",
            "1.0a12",
            "1.0b1.dev456",
            "1.0b2",
            "1.0b2.post345.dev456",
            "1.0b2.post345",
            "1.0rc1.dev456",
            "1.0rc1",
            "1.0",
            "1.0+abc.5",
            "1.0+abc.7",
            "1.0+5",
            "1.0.post456.dev34",
            "1.0.post456",
            "1.0.15",
            "1.1.dev1",
        ];

        for version in versions {
            match PackageVersion::new(version) {
                Ok(_v) => continue,
                Err(e) => panic!("Oh no {}", e),
            }
        }
    }

    #[test]
    fn test_pep440_negative() {
        let versions = vec!["not a version"];

        for version in versions {
            match PackageVersion::new(version) {
                Ok(v) => panic!("Oh no {}", v),
                Err(_e) => continue,
            }
        }
    }

    #[test]
    fn test_use_package_version_as_hash_key() -> Result<()> {
        let versions = vec![
            "1.0",
            "v1.1",
            "2.0",
            "2013.10",
            "2014.04",
            "1!1.0",
            "1!1.1",
            "1!2.0",
            "2!1.0.pre0",
            "1.0.dev456",
            "1.0a1",
            "1.0a2.dev456",
            "1.0a12.dev456",
            "1.0a12",
            "1.0b1.dev456",
            "1.0b2",
            "1.0b2.post345.dev456",
            "1.0b2.post345",
            "1.0rc1.dev456",
            "1.0rc1",
            "1.0",
            "1.0+abc.5",
            "1.0+abc.7",
            "1.0+5",
            "1.0.post456.dev34",
            "1.0.post456",
            "1.0.15",
            "1.1.dev1",
        ];

        let mut some_hash = HashMap::new();

        for i in 0..versions.len() {
            some_hash.insert(versions[i], i);
        }
        Ok(())
    }

    #[test]
    fn test_hashing_and_eq() -> Result<()> {
        assert_eq!(
            default_hash(&PackageVersion::new("1.0a1")?),
            default_hash(&PackageVersion::new("1.0alpha1")?)
        );
        assert_eq!(
            default_hash(&PackageVersion::new("1.0b")?),
            default_hash(&PackageVersion::new("1.0beta")?)
        );
        assert_eq!(
            default_hash(&PackageVersion::new("1.0r")?),
            default_hash(&PackageVersion::new("1.0rev")?)
        );
        assert_eq!(
            default_hash(&PackageVersion::new("1.0c")?),
            default_hash(&PackageVersion::new("1.0rc")?)
        );
        assert_eq!(
            default_hash(&PackageVersion::new("v1.0")?),
            default_hash(&PackageVersion::new("1.0")?)
        );

        Ok(())
    }
}
