use itertools::Itertools;
use std::borrow::Cow;
use unicode_width::UnicodeWidthStr;

pub type Token<'a> = Cow<'a, str>;

pub trait Width {
    fn width(&self) -> usize;
}

impl<'a> Width for Token<'a> {
    fn width(&self) -> usize {
        match self {
            Token::Borrowed(s) => UnicodeWidthStr::width(*s),
            Token::Owned(s) => UnicodeWidthStr::width(s.as_str()),
        }
    }
}

#[derive(Debug)]
pub struct Block<'a> {
    pub prefix: &'a str,
    pub suffix: &'a str,
    pub words: Vec<Token<'a>>,
    pub newline_after: bool,
}

enum Dir {
    Forward,
    Reverse,
}

fn trim_off<'a>(s: &'a str, prefix: &str, suffix: &str) -> &'a str {
    if s.len() < (prefix.len() + suffix.len()) {
        ""
    } else {
        &s[prefix.len()..(s.len() - suffix.len())]
    }
}

fn get_quotes(line: &str) -> (usize, &str) {
    let quote_chars = line
        .split(|c: char| !(c.is_whitespace() || c == '>'))
        .next()
        .unwrap();
    if quote_chars.is_empty() {
        (0, "")
    } else {
        let l = quote_chars.chars().filter(|&c| c == '>').count();
        (l, quote_chars)
    }
}

fn collect_blocks<'a>(lines: &[&'a str], prefix: &'a str, suffix: &'a str) -> Vec<Block<'a>> {
    let mut blocks: Vec<Block> = vec![];
    let groups = lines
        .iter()
        .map(|s| trim_off(s, prefix, suffix))
        .group_by(|l| l.trim().is_empty());
    for (_, line_group) in &groups {
        let mut words = vec![];
        let mut newline_after = false;
        for (i, line) in line_group.enumerate() {
            if line.trim().is_empty() {
                newline_after = true;
                continue;
            }
            let mut w: Vec<_> = line.split_whitespace().collect();
            if i == 0 {
                let indentation = line.chars().take_while(|&c| c.is_whitespace()).count();
                let len = w[0].len();
                w[0] = &line[0..(len + indentation)];
            }
            words.extend(w);
        }
        blocks.push(Block {
            prefix,
            suffix,
            words: words.iter().map(|w| Token::Borrowed(*w)).collect(),
            newline_after,
        });
    }
    blocks
}

pub struct Input<'a> {
    lines: Vec<&'a str>,
}

impl<'a> Input<'a> {
    fn longest_common_affix(&self, dir: Dir) -> &'a str {
        if self.lines.is_empty() {
            return "";
        }
        let mut ret = None;
        let mut i = 0;
        'outer: loop {
            let mut range = None;
            for s in self.lines.iter() {
                if i >= s.len() {
                    break 'outer;
                }
                let (start, finish) = match dir {
                    Dir::Forward => (0, i + 1),
                    Dir::Reverse => ((s.len() - i - 1), s.len()),
                };
                if !s.is_char_boundary(start) || !s.is_char_boundary(finish) {
                    i += 1;
                    continue 'outer;
                }
                match range {
                    None => {
                        range = Some(&s[start..finish]);
                    }
                    Some(prev) if prev != &s[start..finish] => {
                        break 'outer;
                    }
                    _ => continue,
                }
            }
            ret = range;
            i += 1;
        }
        ret.unwrap_or("")
    }

    pub fn analyze_quotes(&self) -> Option<Vec<Block<'a>>> {
        let quotes: Vec<_> = self.lines.iter().map(|line| get_quotes(line)).collect();
        if quotes.iter().any(|&(l, _)| l > 0) {
            let mut blocks = vec![];
            let mut quote = &(0, "");
            let mut i = 0;
            let mut idx = 0;
            for this_quote in quotes.iter() {
                if quotes[i].0 != quote.0 {
                    if idx < i {
                        blocks.extend(collect_blocks(&self.lines[idx..i], quote.1, ""));
                    }
                    quote = this_quote;
                    idx = i;
                }
                i += 1;
            }
            if idx < i {
                blocks.extend(collect_blocks(&self.lines[idx..i], quote.1, ""));
            }
            Some(blocks)
        } else {
            None
        }
    }

    pub fn analyze_code_comments(&self) -> Option<Vec<Block<'a>>> {
        if self.lines.is_empty() {
            return None;
        }
        let comment_styles = ["///", "//", "#", ";;", ";", "--"];
        let first = self.lines[0];
        let start = first.find(|c: char| !c.is_ascii_whitespace())?;
        let comment_style = comment_styles
            .iter()
            .find(|&pat| (first[start..]).starts_with(pat))?;
        let pat = &first[0..=(start + comment_style.len())];
        if self.lines.iter().all(|line| line.starts_with(pat)) {
            let collected = collect_blocks(&self.lines, pat, "");
            Some(collected)
        } else {
            None
        }
    }

    pub fn analyze_surround(&self) -> Option<Vec<Block<'a>>> {
        let mut prefix = self.longest_common_affix(Dir::Forward);
        let mut suffix = self.longest_common_affix(Dir::Reverse);

        if prefix == suffix && !prefix.is_empty() {
            prefix = "";
            suffix = "";
        }

        let collected = collect_blocks(&self.lines, prefix, suffix);

        Some(collected)
    }

    pub fn with_input(input: &'a str) -> Self {
        Self {
            lines: input.lines().collect(),
        }
    }
}
