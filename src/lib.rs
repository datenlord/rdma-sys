#![deny(warnings)]
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#![allow(deref_nullptr)] // TODO(fxbug.dev/74605): Remove once bindgen is fixed.
#![allow(
    clippy::missing_safety_doc,
    clippy::needless_return,
    clippy::too_many_arguments,
    clippy::cmp_null
)]

mod bindings {
    use crate::*;
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

mod types;
mod verbs;

use std::os::raw::{c_int, c_uint, c_void};

pub use self::bindings::*;
pub use self::types::*;
pub use self::verbs::*;
