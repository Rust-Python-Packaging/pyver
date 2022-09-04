use anyhow::Result;
use lazy_static::lazy_static;
use pomsky_macro::pomsky;
use regex::Captures;

/// Utility Function for Checking if a `PEP-440` Version String is valid
/// and getting it's groups
///
/// # Example Usage
/// ```
/// use pyver::validate_440_version;
///
/// match validate_440_version("v1.0") {
///     Ok(v) => println!("Is a valid Version!"),
///     Err(e) => panic!("Not Valid!"),
/// }
/// ```
pub fn validate_440_version(version: &str) -> Result<Captures> {
    lazy_static! {
        // Safe to unwrap since Regex is predefined
        // Regex as defined in PEP-0440
        static ref VERSION_VALIDATOR: regex::Regex =
            regex::Regex::new(VALIDATION_REGEX).unwrap();
    }

    // Capture each group of the regex
    // Groups are:
    // epoch, release, pre, pre_l, pre_n, post, post_l, post_n1, post_n2,
    // dev, dev_l, dev_n, local
    let version_match: Captures = match VERSION_VALIDATOR.captures(version) {
        Some(v) => v,
        None => anyhow::bail!("Failed to decode version {}", version),
    };
    Ok(version_match)
}

/// Rulex version of
/// Python's PEP-440 Regex
/// (<https://peps.python.org/pep-0440/#appendix-b-parsing-version-strings-with-regular-expressions>)
static VALIDATION_REGEX: &str = pomsky!(
    // Version String may start with v<version_number>
    // Example:
    // v1.0
    "v"?

    // Version String may include an epoch <epoch_num>!<version>
    // Example:
    // 1!1.0
    (:epoch(['0'-'9']+)'!')?

    // Version String must include major and minor version <major>.<minor>
    // Example:
    // 1.0
    :release(['0'-'9']+("."['0'-'9']+)*)

    // Version String may include Pre-Header
    // Example:
    // 1.0.preview-2
    // 1.0.rc2
    // 1.0beta2
    :pre(
        ["-" "_" "."]?

        :pre_l(
        ("preview"|"alpha"|"beta"|"pre"|"rc"|"a"|"b"|"c")
        )

        ["-" "_" "."]?

        :pre_n(['0'-'9']+)?
    )?

    // Version String may include Post-Header
    // Examples:
    // 1.0-9
    // 1.0-post2
    // 1.0.post.2
    :post(
        "-"
        :post_n1(['0'-'9']+)

        |

        ["-" "_" "."]?
        :post_l("post" | "rev" | "r")
        ["-" "_" "."]?
        :post_n2(['0'-'9']+)?
    )?

    // Version string may include Dev-header
    // Example:
    // 1.0-dev3
    // 1.0dev4
    // 1.0_dev_9
    :dev(
        ["-" "_" "."]?
        :dev_l("dev")
        ["-" "_" "."]?
        :dev_n(['0'-'9']+)?
    )?

    // Version string may include Local Version
    // Local version must start with +
    // Example:
    // 1.0+this.can.say.anything.as.long.as.its.a.letter.or.number.231241
    (
    "+"
    :local(
        ['a'-'z' '0'-'9']+
        ((["-" "_" "."] ['a'-'z' '0'-'9']+)+)?
    )
    )?
);
