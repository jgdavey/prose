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

mod analysis;
pub mod reformat;

pub use reformat::{FormatMode, FormatOpts, Reformatter, reformat};

use std::io::{self, BufRead, Write};

fn print_reformatted<W: Write>(out: &mut W, opts: &FormatOpts, buf: &[String]) -> io::Result<()> {
    if !buf.is_empty() {
        writeln!(out, "{}", reformat(opts, &buf.join("\n")))?;
    }
    Ok(())
}

pub fn process_paragraphs<R: BufRead + ?Sized, W: Write>(
    reader: &mut R,
    out: &mut W,
    opts: FormatOpts,
) -> io::Result<()> {
    let mut buf = vec![];
    for line in reader.lines() {
        let l = line?;
        if l.trim().is_empty() {
            print_reformatted(out, &opts, &buf)?;
            writeln!(out)?;
            buf = vec![];
        } else {
            buf.push(l);
        }
    }
    print_reformatted(out, &opts, &buf)?;
    Ok(())
}
