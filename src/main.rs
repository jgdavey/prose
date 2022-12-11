#[macro_use]
extern crate clap;

use std::fs;
use std::io::{self, BufRead, BufReader};
mod reformat;
use clap::{Arg, ArgAction, Command};
use reformat::{reformat, FormatMode, FormatOpts};

fn print_reformatted(opts: &FormatOpts, buf: &[String]) {
    println!("{}", reformat(opts, &buf.join("\n")));
}

fn process_paragraphs<R: BufRead + ?Sized>(io: &mut R, opts: FormatOpts) -> io::Result<()> {
    let mut buf = vec![];
    for line in io.lines() {
        let l = line?;
        if l.trim().is_empty() {
            print_reformatted(&opts, &buf);
            println!();
            buf = vec![];
        } else {
            buf.push(l);
        }
    }
    print_reformatted(&opts, &buf);
    Ok(())
}

fn matches_to_format_opts(matches: &clap::ArgMatches) -> FormatOpts {
    let width: usize = matches
        .get_one::<usize>("width")
        .cloned()
        .expect("Choose a positive number for width");
    let last_line = matches.get_flag("last line");
    let reduce_jaggedness = matches.get_flag("better fit");
    let tab_width: usize = matches
        .get_one::<usize>("tab width")
        .cloned()
        .expect("Choose a positive number for tab width");
    let format_mode = if matches.get_flag("markdown") {
        FormatMode::Markdown
    } else if matches.get_flag("code comments") {
        FormatMode::Code
    } else {
        FormatMode::PlainText
    };

    FormatOpts {
        max_length: width,
        last_line,
        reduce_jaggedness,
        tab_width,
        format_mode,
    }
}

fn get_reader(input: &str) -> io::Result<Box<dyn BufRead>> {
    if input == "-" {
        Ok(Box::new(BufReader::new(io::stdin())))
    } else {
        Ok(Box::new(BufReader::new(fs::File::open(input)?)))
    }
}

fn main() {
    let matches = Command::new("prose")
        .version(crate_version!())
        .about("Reformats prose to specified width")
        .arg(Arg::new("width")
             .short('w')
             .long("width")
             .value_name("WIDTH")
             .value_parser(clap::value_parser!(usize))
             .default_value("72")
             .help("Sets the maximum width for a line"))
        .arg(Arg::new("last line")
             .short('l')
             .long("last-line")
             .help("Treat last line of a paragraph like the rest")
             .action(ArgAction::SetTrue))
        .arg(Arg::new("better fit")
             .short('f')
             .long("use-better-fit")
             .help("Be more aggressive in reducing jagged line endings, even if it means a narrower width")
             .action(ArgAction::SetTrue))
        .arg(Arg::new("tab width")
             .short('t')
             .long("tab-width")
             .value_parser(clap::value_parser!(usize))
             .default_value("4")
             .help("Number of spaces to expand tab characters to"))
        .arg(Arg::new("markdown")
             .short('m')
             .long("markdown")
             .conflicts_with("code comments")
             .help("Parse as markdown rather than plain text")
             .action(ArgAction::SetTrue))
        .arg(Arg::new("code comments")
             .short('c')
             .long("code-comments")
             .help("Handle common code-comment prefixes")
             .action(ArgAction::SetTrue))
        .arg(Arg::new("FILE")
             .help("Operate on file FILE (Use '-' for stdin)")
             .required(false)
             .index(1))
        .get_matches();

    let input = matches.get_one::<&str>("FILE").unwrap_or(&"-");
    let opts = matches_to_format_opts(&matches);
    match get_reader(input) {
        Ok(mut rdr) => {
            if let Err(err) = process_paragraphs(&mut rdr, opts) {
                eprintln!("{}", err);
                ::std::process::exit(2);
            }
        }
        Err(e) => {
            eprintln!("Error opening {}: {}", input, e);
            ::std::process::exit(1);
        }
    }
}
