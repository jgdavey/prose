use clap::Parser;

use std::fs;
use std::io::{self, BufRead, BufReader};
mod analysis;
mod reformat;

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

fn get_reader(input: &str) -> io::Result<Box<dyn BufRead>> {
    if input == "-" {
        Ok(Box::new(BufReader::new(io::stdin())))
    } else {
        Ok(Box::new(BufReader::new(fs::File::open(input)?)))
    }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Filename or "-" for stdin
    file: Option<String>,

    /// Target width
    #[arg(short, long, default_value_t = 72)]
    width: usize,

    /// Tab width
    #[arg(short, long, default_value_t = 4)]
    tab_width: usize,

    /// Treat last line of a paragraph like the rest
    #[arg(short, long)]
    last_line: bool,

    /// Be more aggressive in reducing jagged line endings, even if it means a narrower width
    #[arg(short, long = "use-better-fit")]
    fit: bool,

    /// Treat text like markdown (format paragraphs only)
    #[arg(short, long)]
    markdown: bool,

    /// Try to handle code comments
    #[arg(short, long)]
    code_comments: bool,
}

fn main() {
    let cli = Cli::parse();

    let input = cli.file.unwrap_or_else(|| String::from("-"));

    let format_mode = if cli.markdown {
        FormatMode::Markdown
    } else if cli.code_comments {
        FormatMode::Code
    } else {
        FormatMode::PlainText
    };

    let opts = FormatOpts {
        max_length: cli.width,
        last_line: cli.last_line,
        reduce_jaggedness: cli.fit,
        tab_width: cli.tab_width,
        format_mode,
    };

    match get_reader(&input) {
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
