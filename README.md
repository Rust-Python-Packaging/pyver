# PyVer (WIP)

[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE-MIT)

> **Python PEP-440 Version Parsing**

This package allows for parsing Python PEP-440 version numbers and comparisons between PEP-440 Versions

## Usage

```Toml
[dependencies]
pyver = "0.0.1"
```

```Rust
use pyver::PackageVersion;

let a = PackageVersion::new("v1.0a2.dev456").unwrap();
let b = PackageVersion::new("v1.0a2.dev457").unwrap();

assert_eq!(a < b, true);
```


## Contribution

For now Contributions will be quite loose but soon we will be switching to a rolling release model
