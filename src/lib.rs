//! A pure-Rust library to work with Linux memfd and seals.
//!
//! It provides support for creating `memfd` objects on Linux
//! and handling seals on them. This was first introduced in
//! Linux kernel 3.17.
//! For further details, see `memfd_create(2)` manpage.
//!
//! ```rust
//! use memfd;
//! # use memfd::errors::Result;
//!
//! fn new_sized_memfd() -> Result<memfd::Memfd> {
//!     // Create a sealable memfd.
//!     let opts = memfd::MemfdOptions::default().allow_sealing(true);
//!     let mfd = opts.create("sized-1K")?;
//!
//!     // Resize to 1024B.
//!     mfd.as_file().set_len(1024)?;
//!
//!     // Add seals to prevent further resizing.
//!     let mut seals = memfd::SealsHashSet::new();
//!     seals.insert(memfd::FileSeal::SealShrink);
//!     seals.insert(memfd::FileSeal::SealGrow);
//!     mfd.add_seals(&seals)?;
//!
//!     // Prevent further sealing changes.
//!     mfd.add_seal(memfd::FileSeal::SealSeal);
//!
//!     Ok(mfd)
//! }
//! ```

#![deny(missing_docs)]

extern crate either;
extern crate errno;
#[macro_use]
extern crate error_chain;
extern crate libc;

pub mod errors;
mod memfd;
mod nr;
mod sealing;

pub use memfd::{HugetlbSize, Memfd, MemfdOptions};
pub use sealing::{FileSeal, SealsHashSet};
