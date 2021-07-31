#![deny(warnings)]
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#![allow(unknown_lints)] // TODO(fxbug.dev/74605): Remove once toolchain rolls and deref_nulptr is known.
#![allow(deref_nullptr)] // TODO(fxbug.dev/74605): Remove once bindgen is fixed.

pub mod verbs;
pub use verbs::*;
