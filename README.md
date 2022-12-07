# PyVer

<!-- markdownlint-disable MD013 -->
![Crates.io](https://img.shields.io/crates/l/pyver) ![Crates.io](https://img.shields.io/crates/v/pyver) ![docs.rs](https://img.shields.io/docsrs/pyver) [![🧪 Tests](https://github.com/Allstreamer/pyver/actions/workflows/tests.yml/badge.svg)](https://github.com/Allstreamer/pyver/actions/workflows/tests.yml) [![🖋  Check linting](https://github.com/Allstreamer/pyver/actions/workflows/lint.yml/badge.svg)](https://github.com/Allstreamer/pyver/actions/workflows/lint.yml) [![🔨 Build](https://github.com/Allstreamer/pyver/actions/workflows/build.yml/badge.svg)](https://github.com/Allstreamer/pyver/actions/workflows/build.yml) [![📦 Package](https://github.com/Allstreamer/pyver/actions/workflows/package.yml/badge.svg)](https://github.com/Allstreamer/pyver/actions/workflows/package.yml) [![📄 Build docs](https://github.com/Allstreamer/pyver/actions/workflows/docs.yml/badge.svg)](https://github.com/Allstreamer/pyver/actions/workflows/docs.yml) [![👔 Check formatting](https://github.com/Allstreamer/pyver/actions/workflows/format.yml/badge.svg)](https://github.com/Allstreamer/pyver/actions/workflows/format.yml)
<!-- markdownlint-enable MD013 -->

> **Python PEP-440 Version Parser**

This package allows for parsing Python [PEP-440](https://peps.python.org/pep-0440/)
version numbers and for comparisons between
[PEP-440](https://peps.python.org/pep-0440/) version numbers.

## Usage

```toml
[dependencies]
pyver = "1"
```

The following is an example for initilizing and comparing two version strings

```rust
use pyver::PackageVersion;

let a = PackageVersion::new("v1.0a2.dev456").unwrap();
let b = PackageVersion::new("v1.0a2.dev457").unwrap();

assert_eq!(a < b, true);
```

Comparing single version components

```rust
use pyver::PackageVersion;

let a = PackageVersion::new("1!1.323.dev2").unwrap();
let b = PackageVersion::new("v3.2.dev2").unwrap();

// Check that both have the same dev version
assert_eq!(a.dev, b.dev);
```

Seperation of version identifiers

```rust
use pyver::PackageVersion;

let version = PackageVersion::new("v1.23.dev2").unwrap();

println!("{:?}", version.release.major);
// > 1

println!("{:?}", version.release.minor);
// > 2

println!("{:?}", version.dev);
// > Some(DevHead { dev_num: Some(2) })
```

See more examples at the [docs](https://docs.rs/pyver/latest/pyver/)

## Contribution

For now Contributions will be quite loose.
