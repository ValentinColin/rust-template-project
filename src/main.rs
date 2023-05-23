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

use crate::lib::greet;
use crate::prelude::*;

mod error;
mod lib;
mod prelude;

/// This function returns the greeting: `Hello, world!`
fn main() -> Result<()> {
    greet();
    Ok(())
}
