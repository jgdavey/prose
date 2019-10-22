use std::fs;
use std::io::{self, BufReader, BufRead};
mod reformat;
use clap::{App, Arg};
use reformat::{reformat, FormatOpts};

fn print_reformatted(opts: &FormatOpts, buf: &[String]) {
    if !buf.is_empty() {
        println!("{}", reformat(&opts, &buf.join("\n")));
    }
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
        .value_of("width")
        .unwrap()
        .parse()
        .expect("Choose a positive number for width");
    let last_line = matches.is_present("last line");
    let reduce_jaggedness = matches.is_present("better fit");
    let tab_width: usize = matches
        .value_of("tab width")
        .unwrap()
        .parse()
        .expect("Choose a positive number for tab width");

    FormatOpts {
        max_length: width,
        last_line,
        reduce_jaggedness,
        tab_width,
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
    let matches = App::new("prose")
        .version("0.1")
        .about("Reformats prose to specified width")
        .arg(Arg::with_name("width")
             .short("w")
             .long("width")
             .value_name("WIDTH")
             .default_value("72")
             .help("Sets the maximum width for a line")
             .takes_value(true))
        .arg(Arg::with_name("last line")
             .short("l")
             .long("last-line")
             .help("Treat last line of a paragraph like the rest")
             .takes_value(false))
        .arg(Arg::with_name("better fit")
             .short("f")
             .long("use-better-fit")
             .help("Be more aggressive in reducing jagged line endings, even if it means a narrower width")
             .takes_value(false))
        .arg(Arg::with_name("tab width")
             .short("t")
             .long("tab-width")
             .default_value("4")
             .help("Number of spaces to expand tab characters to")
             .takes_value(true))
        .arg(Arg::with_name("FILE")
             .help("Operate on file FILE (Use '-' for stdin)")
             .required(false)
             .index(1))
        .get_matches();

    let input = matches.value_of("FILE").unwrap_or("-");
    let opts = matches_to_format_opts(&matches);
    match get_reader(input) {
        Ok(mut rdr) => {
            if let Err(err) = process_paragraphs(&mut rdr, opts) {
                eprintln!("{}", err);
                ::std::process::exit(2);
            }
        },
        Err(e) => {
            eprintln!("Error opening {}: {}", input, e);
            ::std::process::exit(1);
        }
    }
}
