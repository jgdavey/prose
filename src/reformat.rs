// use itertools::Itertools;
use crate::analysis::{Block, Input, Token, Width};
use pathfinding::prelude::dijkstra;
use unicode_width::UnicodeWidthStr;

pub enum FormatMode {
    PlainText,
    Markdown,
    Code,
}

pub struct FormatOpts {
    pub max_length: usize,
    pub tab_width: usize,
    pub last_line: bool,
    pub reduce_jaggedness: bool,
    pub format_mode: FormatMode,
}

impl Default for FormatOpts {
    fn default() -> Self {
        FormatOpts {
            max_length: 72,
            last_line: false,
            reduce_jaggedness: false,
            tab_width: 4,
            format_mode: FormatMode::PlainText,
        }
    }
}

#[allow(dead_code)]
impl FormatOpts {
    pub fn with_max_length(max_length: usize) -> Self {
        Self {
            max_length,
            ..Default::default()
        }
    }

    pub fn new(
        max_length: usize,
        last_line: bool,
        reduce_jaggedness: bool,
        tab_width: usize,
        format_mode: FormatMode,
    ) -> Self {
        Self {
            max_length,
            tab_width,
            last_line,
            reduce_jaggedness,
            format_mode,
        }
    }
}

#[derive(Debug)]
struct Entry {
    offset: usize,
}

impl Entry {
    fn new(offset: usize) -> Self {
        Entry { offset }
    }
}

fn spaces(n: usize) -> String {
    " ".repeat(n)
}

#[derive(Default)]
pub struct Reformatter<'a> {
    blocks: Vec<Block<'a>>,
    target: usize,
    last_line: bool,
    fit: bool,
}

impl<'a> Reformatter<'a> {
    pub fn new(opts: &FormatOpts, input: &'a str) -> Reformatter<'a> {
        let input = Input::with_input(input);
        let blocks = match opts.format_mode {
            FormatMode::Code => input.analyze_code_comments(),
            _ => input.analyze_quotes(),
        }
        .unwrap_or_else(|| input.analyze_surround().unwrap());

        // eprintln!("Prefix: {}, Suffix: {}, Max: {}, Target: {}", prefix, suffix, opts.max_length, target);
        Reformatter {
            blocks,
            target: opts.max_length,
            last_line: opts.last_line,
            fit: opts.reduce_jaggedness,
        }
    }

    fn successors(
        &self,
        entries: &[Entry],
        i: usize,
        target: usize,
        allow_overage: bool,
    ) -> Vec<(usize, u64)> {
        let word1 = &entries[i];
        let count = entries.len();
        let mut results = vec![];
        for (j, word2) in entries.iter().enumerate().take(count).skip(i + 1) {
            let linew = word2.offset - word1.offset + j - i - 1;
            //width of all words + width of all spaces = total line width
            if linew > target {
                if results.is_empty() && allow_overage {
                    // ensure there's always at least a bail-out option
                    // for the next word, but very expensive
                    results.push((j, 100_000));
                }
                break;
            }
            let cost = if j > (count - 2) && !self.last_line {
                //handle last line
                0
            } else {
                let diff = (target - linew) as u64;
                diff * diff
            };
            results.push((j, cost));
        }
        results
    }

    fn solve(&self, words: &[Token<'a>], target: usize) -> (Vec<usize>, u64) {
        let count = words.len();
        let dummy = Entry::new(0);

        let mut entries = vec![dummy];
        let mut offset = 0;
        for w in words {
            let wid = w.width();
            offset += wid;
            entries.push(Entry::new(offset));
        }

        let result = dijkstra(
            &0,
            |i| self.successors(&entries, *i, target, false),
            |i| *i == count,
        );

        if let Some(tuple) = result {
            tuple
        } else {
            eprintln!("Warning: allowing some words to extend beyond target width");
            // try again, allowing overage
            dijkstra(
                &0,
                |i| self.successors(&entries, *i, target, true),
                |i| *i == count,
            )
            .expect("Unable to find optimum solution")
        }
    }

    fn reformat_section(&self, block: &Block) -> (Vec<String>, usize) {
        let words = &block.words;
        let rawtarget =
            self.target as i64 - block.prefix.width() as i64 - block.suffix.width() as i64;
        let target = std::cmp::max(rawtarget, 1) as usize;

        let min_target = if self.fit { target / 2 } else { target };
        let max_target = target;

        let mut path = vec![];
        let mut best_cost = 100_000_000;
        let mut best_target = max_target;

        for target in (min_target..=max_target).rev() {
            let (p, cost) = self.solve(words, target);
            let target_distance = max_target as u64 - target as u64;
            // higher cost the further from original target
            let cost = cost + target_distance * target_distance;
            if cost < best_cost {
                best_cost = cost;
                path = p;
                best_target = target;
            }
        }

        let mut lines: Vec<String> = vec![];

        for s in path.windows(2) {
            if let [start, end] = *s {
                let mut l = block.prefix.to_string();
                l.push_str(
                    &words[start..end]
                        .iter()
                        .map(|w| w.to_string())
                        .collect::<Vec<_>>()
                        .join(" "),
                );
                lines.push(l);
            }
        }
        if block.newline_after {
            let extra = block.prefix.trim_end().to_string();
            lines.push(extra);
        }
        (lines, best_target)
    }

    pub fn reformatted(&self) -> String {
        // get "unadorned" body
        let sections: Vec<_> = self
            .blocks
            .iter()
            .map(|s| (s, self.reformat_section(s)))
            .collect();
        let max_padding = sections
            .iter()
            .map(|s| (s.1).1)
            .max()
            .unwrap_or(self.target) as i64;

        let mut output = vec![];

        for (block, (body, _)) in &sections {
            let suffix_length = block.suffix.width();
            let prefix_length = block.prefix.width();

            for l in body {
                let mut line = l.clone();
                if suffix_length > 0 {
                    let pad_amount = max_padding - l.width() as i64 + prefix_length as i64;
                    let pad = spaces(std::cmp::max(pad_amount, 0i64) as usize);
                    line.push_str(&pad);
                    line.push_str(block.suffix);
                }
                output.push(line);
            }
        }
        output.join("\n")
    }
}

pub fn reformat(opts: &FormatOpts, input: &str) -> String {
    use FormatMode::*;
    use pulldown_cmark::{Event, Options, Parser, Tag};

    let do_reformat = match opts.format_mode {
        Markdown => {
            let mut parser = Parser::new_ext(input, Options::empty());
            let pair = (parser.next(), parser.next());
            matches!(
                pair,
                (Some(Event::Start(Tag::Paragraph)), Some(Event::Text(_)))
            )
        }
        _ => true,
    };

    if do_reformat {
        let cleaned_input = if input.find('\t').is_some() {
            let expanded = spaces(opts.tab_width);
            Token::Owned(input.replace('\t', &expanded))
        } else {
            Token::Borrowed(input)
        };

        let rfmt = Reformatter::new(opts, &cleaned_input);
        rfmt.reformatted()
    } else {
        input.to_string()
    }
}
