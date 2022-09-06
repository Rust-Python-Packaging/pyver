/*!
# Handling of `PEP-440`
This library implements Python's Package versioning system.

Read more at <https://peps.python.org/pep-0440/>

# Usage
The `pyver` crate is available on [crates.io](https://crates.io/crates/pyver),
you can include it in your project by adding the following to your `Cargo.toml`.
```toml
[dependencies]
pyver = "1.0"
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

#[macro_use]
extern crate derivative;

mod validator;
// Expose validate_440_version function
pub use validator::validate_440_version;

/// Identifiers (i.e. the components of a version string)
// Expose Ids Module
pub mod ids;

mod version;
// Expose PackageVersion Struct
pub use version::PackageVersion;
