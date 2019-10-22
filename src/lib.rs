//! # Basic usage
//!
//! ```
//! extern crate prose;
//!
//! use prose::{self, FormatOpts};
//!
//! let data = "Lot's of string data... to be reformatted";
//! let opts = FormatOpts::with_max_length(25);
//! let new_data = prose::reformat(&opts, data);
//!
//! assert_eq!(new_data, "Lot's of string data...\nto be reformatted");
//! ```

pub mod reformat;

pub use reformat::{FormatOpts, Reformatter, reformat};
