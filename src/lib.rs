//! # Basic usage
//!
//! ```
//! extern crate prose;
//!
//! use prose::{Reformatter,FormatOpts};
//!
//! let data = "Lot's of string data... to be reformatted";
//! let opts = FormatOpts::default();
//! let reformatter = Reformatter::new(&opts, data);
//! let new_data = reformatter.reformatted();
//! ```

pub mod reformat;

pub use reformat::{FormatOpts, Reformatter};

