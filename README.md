spirv-reflect
========

[![spirv-reflect on travis-ci.com](https://travis-ci.com/gwihlidal/spirv-reflect-rs.svg?branch=master)](https://travis-ci.com/gwihlidal/spirv-reflect-rs)
[![Latest version](https://img.shields.io/crates/v/spirv-reflect.svg)](https://crates.io/crates/spirv-reflect)
[![Documentation](https://docs.rs/spirv-reflect/badge.svg)](https://docs.rs/spirv-reflect)
[![](https://tokei.rs/b1/github/gwihlidal/spirv-reflect-rs)](https://github.com/gwihlidal/spirv-reflect-rs)
![MIT](https://img.shields.io/badge/license-MIT-blue.svg)
![APACHE2](https://img.shields.io/badge/license-APACHE2-blue.svg)

Reflection API in rust for SPIR-V shader byte code, intended for Vulkan applications.

This crate provides an FFI layer and idiomatic rust wrappers for the excellent [SPIRV-Reflect](https://github.com/chaoticbob/SPIRV-Reflect) C/C++ library.

- [Documentation](https://docs.rs/spirv-reflect)
- [Release Notes](https://github.com/gwihlidal/spirv-reflect-rs/releases)

## Features

* Extract descriptor bindings from SPIR-V bytecode, to assist in the generation of Vulkan descriptor set and pipeline layouts.
* Extract push constant block size from SPIR-V bytecode to assist in the generation of pipeline layouts.
* Extract full layout data for uniform buffer and push constant blocks from SPIR-V bytecode, to assist in application updates of these structures.
* Extract input/output variables from SPIR-V bytecode (including semantic decorations for HLSL shaders), to assist in validation of pipeline input/output settings.
* Easily map Vulkan types to DirectX 12 resource types
* Remap descriptor bindings, and update the source SPIR-V bytecode accordingly.
* Log all reflection data as human-readable text.

## Planned Features

* Extensive unit tests and examples.
* Pure rust version.
* Command line tool for reflection and manipulation.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
spirv-reflect = "0.2.2"
```

and add this to your crate root:

```rust
extern crate spirv_reflect;
```

## Example

Currently there is only a single monolithic `demo` example, which shows some usage. A CLI tool is planned that will be useful on its own, and as a clean example of usage patterns.

```shell
cargo run --release --example demo
```

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Credits and Special Thanks

- [Hai Nguyen](https://github.com/chaoticbob) (Author of C/C++ library)
- [Cort Stratton](https://github.com/cdwfs) (Author of C/C++ library)
- [Daniel Collin](https://github.com/emoon) (Code Review)
- [Alexandru Ene](https://github.com/AlexEne) (Contribution)
- [Jasper Bekkers](https://github.com/Jasper-Bekkers) (Contribution)
- [Benjamin Saunders](https://github.com/Ralith) (Contribution)
- [Nuno Subtil](https://github.com/nsubtil) (Contribution)
- [Paweł Grabarz](https://github.com/Frizi) (Contribution)
- [Walter Pearce](https://github.com/jaynus) (Contribution)
- [Bastian Kauschke](https://github.com/lcnr) (Contribution)
- [Lukas Wirth](https://github.com/Veykril) (Contribution)

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

Contributions are always welcome; please look at the [issue tracker](https://github.com/gwihlidal/spirv-reflect-rs/issues) to see what
known improvements are documented.

## Code of Conduct

Contribution to the spirv-reflect crate is organized under the terms of the
Contributor Covenant, the maintainer of spirv-reflect, @gwihlidal, promises to
intervene to uphold that code of conduct.