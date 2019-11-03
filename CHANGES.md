# Changes

## 0.2.2 (2019-11-02)

* Upgraded all dependencies.
* Removed "parallel" feature from `cc` dependency.

## 0.2.1 (2019-03-29)

* Minor improvements.
* Updated all dependencies.

## 0.2.0 (2019-03-25)

* Fixed a crash when reflecting optimized SPIR-V which has no name descriptors (Walter Pearce).

## 0.1.9 (2019-03-10)

* Updated all dependencies.
* Added support for NV_ray_tracing (Jasper Bekkers and Nuno Subtil).
* Support glsl uniform sampler2D type (Paweł Grabarz).
* Removed unused dependency, and load_u32_data fix (Benjamin Saunders).

## 0.1.8 (2018-12-09)

* Removed get_code_size/get_code_slice in favour of get_code (lifetime safety).
* Numerous optimizations (excessive copies).
* Fixed some FFI memory crashes.

## 0.1.7 (2018-12-08)

* Rust 2018 Edition.

## 0.1.6 (2018-11-21)

* Correct struct by-ref passing for the change_* methods.

## 0.1.5 (2018-11-21)

* Added PartialEq to all types.

## 0.1.4 (2018-11-21)

* Fixed some name mangling issues on Windows when bindings are generated on macOS.

## 0.1.3 (2018-11-21)

* Improved bindgen tooling, and also the generated bindings.rs file.

## 0.1.2 (2018-11-20)

* Added `load_u8_data` and `load_u32_data` helpers to `ShaderModule` for convenience.

## 0.1.1 (2018-11-20)

* Log all reflection data as human-readable text.
* Cleaned up some code in the demo example.

## 0.1.0 (2018-11-20)

* First release.