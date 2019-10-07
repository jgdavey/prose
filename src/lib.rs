//! # Basic usage
//!
//! ```
//! extern crate prose;
//!
//! use prose::{Reformatter,FormatOpts};
//!
//! let data = "Lot's of string data... to be reformatted";
//! let opts = FormatOpts::with_max_length(25);
//! let reformatter = Reformatter::new(&opts, data);
//! let new_data = reformatter.reformatted();
//!
//! assert_eq!(new_data, "Lot's of string data...\nto be reformatted");
//! ```

pub mod reformat;

pub use reformat::{FormatOpts, Reformatter};
