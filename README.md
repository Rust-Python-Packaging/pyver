# PyVer

[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE-MIT)

> **Python PEP-440 Version Parser**

This package allows for parsing Python [PEP-440](https://peps.python.org/pep-0440/) version numbers and for comparisons between [PEP-440](https://peps.python.org/pep-0440/) version numbers.

## Usage

```Toml
[dependencies]
pyver = "0"
```

```Rust
use pyver::PackageVersion;

let a = PackageVersion::new("v1.0a2.dev456").unwrap();
let b = PackageVersion::new("v1.0a2.dev457").unwrap();

assert_eq!(a < b, true);
```

## Contribution

For now Contributions will be quite loose.
