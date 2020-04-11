//! This crate shares common functionality across the `heim` sub-crates.
//!
//! Do **NOT** use it directly.

#![doc(html_root_url = "https://docs.rs/heim-common/0.1.0-alpha.1")]
#![deny(
    unused,
    unused_imports,
    unused_features,
    bare_trait_objects,
    future_incompatible,
    missing_debug_implementations,
    missing_docs,
    nonstandard_style,
    dead_code,
    deprecated,
    intra_doc_link_resolution_failure
)]
#![warn(
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_results
)]
#![allow(clippy::missing_safety_doc)]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod errors;
#[doc(hidden)]
pub mod fs;
mod macros;
#[doc(hidden)]
pub mod sys;
pub mod units;
#[doc(hidden)]
pub mod utils;

pub use self::errors::{Context, Error, Result};

/// Process identifier type.
#[cfg(unix)]
pub type Pid = libc::pid_t;

/// Process identifier type.
// TODO: Is it a correct type for pid?
#[cfg(target_os = "windows")]
pub type Pid = winapi::shared::minwindef::DWORD;

/// Prelude intended to be used across `heim-*` crates.
///
/// Consider not to use it in your code, because it is kinda internal
/// and might change at any time.
pub mod prelude {
    pub use super::errors::{Error, Result};
    pub use super::fs as fs_ext;
    pub use super::wrap;
}
