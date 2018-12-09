# Changes

## 0.1.8 (2018-12-09)

* Removed get_code_size/get_code_slice in favour of get_code (lifetime safety)
* Numerous optimizations (excessive copies)
* Fixed some FFI memory crashes

## 0.1.7 (2018-12-08)

* Rust 2018 Edition

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