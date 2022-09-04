/*!
# Handling of `PEP-440`
This library implements Pythons Package versioning system.

Read more at <https://peps.python.org/pep-0440/>

# Usage
The `pyver` crate is available on [crates.io](https://crates.io/crates/pyver),
you can include it in your project by adding the following to your `Cargo.toml`.
```toml
[dependencies]
pyver = "0.1"
```
# Example
The following example shows how to parse a package version and 
how to compare them
```
use pyver::PackageVersion;

let a = PackageVersion::new("v1.0a2.dev456").unwrap();
let b = PackageVersion::new("v1.1a2.dev457").unwrap();

assert!(a < b);
```

If you want to verify single version strings do
```
use pyver::validate_440_version;

assert!(
    validate_440_version("1.0").is_ok()
);
```
*/
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fmt;

#[macro_use]
extern crate derivative;

mod validator;
pub use validator::validate_440_version;

/// Identifiers (i.e. the components of a version string)
pub mod ids;
use ids::{DevHead, PostHead, PostHeader, PreHeader, ReleaseHeader};

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
#[derive(Derivative, Debug, Serialize, Deserialize)]
#[derivative(PartialOrd, PartialEq)]
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

#[cfg(test)]
mod tests {
    use crate::PackageVersion;
    use anyhow::Result;

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
}
