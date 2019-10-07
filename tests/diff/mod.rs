extern crate difference;

use ansi_term;
use ansi_term::Style;
use ansi_term::Colour::{Red, Green};
use difference::{Difference, Changeset};
use std::env;

pub fn print_diff(cs: &Changeset) -> std::io::Result<()> {
    let Changeset {diffs, ..} = cs;

    let red_fg;
    let green_fg;

    match env::var("TERM").as_ref().map(String::as_str) {
        Ok("dumb") | Err(_) => {
            red_fg = Style::new();
            green_fg = Style::new();
        },
        Ok(_) => {
            red_fg = Style::new().fg(Red);
            green_fg = Style::new().fg(Green);
        }
    }

    for i in 0..diffs.len() {
        match diffs[i] {
            Difference::Same(ref x) => {
                for line in x.lines() {
                    println!(" {}", line);
                }
            }
            Difference::Add(ref x) => {
                for line in x.lines() {
                    println!("{}{}", green_fg.paint("+"), green_fg.paint(line));
                }
            }
            Difference::Rem(ref x) => {
                for line in x.lines() {
                    println!("{}{}", red_fg.paint("-"), red_fg.paint(line));
                }
            }
        }
    }
    Ok(())
}

#[macro_export]
macro_rules! assert_diff {
    ($orig:expr, $edit:expr) => ({
        assert_diff!($orig, $edit, "\n", 0)
    });

    ($orig:expr, $edit:expr, $split:expr, $expected:expr) => ({
        let orig = $orig;
        let edit = $edit;

        let changeset = difference::Changeset::new(orig, edit, &($split));
        if changeset.distance != $expected {
            if let Err(e) = $crate::diff::print_diff(&changeset) {
                eprintln!("{}", e);
            }
            panic!("assertion failed: edit distance was {}, not {}, see diff above",
                   changeset.distance,
                   &($expected))
        }
    })
}
