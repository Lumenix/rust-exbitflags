[![Cargo]][url: Cargo] [![License]][url: License] [![Semver]][url: Semver] [![Gittip]][url: Gittip]

# rust-exbitflags

> Macro simplifying bitflag declaration

## Install

```toml
[dependencies]
exbitflags = "0.1.0"
```

exbitflags uses semver, so only major versions break backward compatibility. While Rust hasn't reached 1.0, compatibility breaks through language evolution are ignored and counted as bugfixes; the compatibility is for this API only.

## Usage

```rust
// Loads exbitflags.
#[phase(plugin)] extern crate exbitflags;

// Creates bitflags named Operations with type u32
// with flags ADD = 0x1, DELETE = 0x2, MODIFY = 0x4.
ebf!( Operations, u32, ADD, DELETE, MODIFY );
```

## Platforms

- All platforms supported by rust.

## Bugs / Issues / Feature requests / Contribution

Want to contribute? Great stuff! Please use the issue system that github provides to report bugs/issues or request an enhancement. Pull requests are also more than welcome.

## Author

**Mazdak Farrokhzad / Centril [&lt;twingoow@gmail.com&gt;]**

+ [twitter]
+ [github]

## Copyright & License

Licensed under the **[ASL2.0 License]**.
Copyright 2015 Mazdak Farrokhzad.

## Acknowledgements

Thanks to [Daniel Keep](https://github.com/DanielKeep) for his [quick intro to macros]

## Changelog

### 0.1.0
+ Initial version.

<!-- references -->

[Cargo]: http://img.shields.io/badge/cargo-0.1.0-orange.svg?style=flat
[url: Cargo]: @TODO
[License]: http://img.shields.io/badge/license-ASL2.0-blue.svg?style=flat
[url: License]: LICENSE
[Semver]: http://img.shields.io/badge/semver-2.0.0-blue.svg?style=flat
[url: Semver]: http://semver.org/spec/v2.0.0.html
[Gittip]: http://img.shields.io/gittip/Centril.svg?style=flat
[url: Gittip]: https://www.gittip.com/Centril/

[Daniel Keep]: https://github.com/DanielKeep
[quick intro to macros]: https://danielkeep.github.io/quick-intro-to-macros.html

[twitter]: http://twitter.com/CenoRIX
[github]: http://github.com/centril
[&lt;twingoow@gmail.com&gt;]: mailto:twingoow@gmail.com

[ASL2.0 License]: LICENSE

<!-- references -->