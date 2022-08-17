//! Description of this crate

// Careful to the syntax:
//
// | Documentation | Inner         | Outer         |
// |---------------|:-------------:|:-------------:|
// | Line          | //! blabla    | /// blabla    |
// | Block         | /*! blabla */ | /** blabla */ |
//
// - Inner attribute: #![attr]
// - Outer attribute: #[attr]

// Good practice: use these attributes
#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]

mod lib;
use crate::lib::lib_hello;

/// This function returns the greeting: `Hello, world!`
fn main() {
    lib_hello();
}
