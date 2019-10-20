use pathfinding::prelude::dijkstra;
use unicode_width::UnicodeWidthStr;
use itertools::Itertools;

pub struct FormatOpts {
    pub max_length: usize,
    pub tab_width: usize,
    pub last_line: bool,
    pub reduce_jaggedness: bool,
}

impl Default for FormatOpts {
    fn default() -> Self {
        FormatOpts {
            max_length: 72,
            last_line: false,
            reduce_jaggedness: false,
            tab_width: 4,
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
    ) -> Self {
        Self {
            max_length,
            last_line,
            reduce_jaggedness,
            tab_width,
        }
    }
}

trait Width {
    fn width(&self) -> usize;
}

impl Width for String {
    fn width(&self) -> usize {
        UnicodeWidthStr::width(self.as_str())
    }
}

impl Width for &str {
    fn width(&self) -> usize {
        UnicodeWidthStr::width(*self)
    }
}

#[derive(Debug)]
struct Block {
    prefix: String,
    suffix: String,
    words: Vec<String>,
    newline_after: bool,
}

enum Dir {
    Forward,
    Reverse,
}

fn longest_common_affix(char_slices: &[Vec<char>], dir: Dir) -> String {
    if char_slices.is_empty() {
        return String::from("");
    }
    let mut ret = Vec::new();
    let mut i = 0;
    'outer: loop {
        let mut c = None;
        for s in char_slices {
            if i == s.len() {
                break 'outer;
            }
            let j = match dir {
                Dir::Forward => i,
                Dir::Reverse => s.len() - i - 1,
            };
            match c {
                None => {
                    c = Some(s[j]);
                }
                Some(letter) if letter != s[j] => {
                    break 'outer;
                }
                _ => continue,
            }
        }
        if let Some(letter) = c {
            ret.push(letter);
        }
        i += 1;
    }
    if let Dir::Reverse = dir {
        ret.reverse();
    }
    ret.iter().collect()
}

fn spaces(n: usize) -> String {
    std::iter::repeat(" ").take(n).collect::<String>()
}

fn trim_off<'a>(s: &'a str, prefix: &str, suffix: &str) -> &'a str {
    if s.len() < prefix.len() {
        ""
    } else {
        s.trim_start_matches(prefix).trim_end_matches(suffix)
    }
}

fn collect_blocks(lines: &[&str], prefix: &str, suffix: &str) -> Vec<Block> {
    let mut blocks: Vec<Block> = vec![];
    let groups = lines.iter()
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
            let mut w: Vec<_> = line.split_whitespace().map(|w| w.to_string()).collect();
            if i == 0 {
                let indentation = line.chars().take_while(|&c| c.is_whitespace()).count();
                let mut s = spaces(indentation);
                s.push_str(&w[0]);
                w[0] = s;
            }
            words.extend(w);
        }
        blocks.push(Block {
            prefix: prefix.to_string(),
            suffix: suffix.to_string(),
            words,
            newline_after,
        });
    }
    blocks
}

fn get_quotes(line: &str) -> (usize, &str) {
    let quote_chars = line.splitn(2, |c: char| !(c.is_whitespace() || c == '>')).next().unwrap();
    if quote_chars.is_empty() {
        (0, "")
    } else {
        let l = quote_chars.chars().filter(|&c| c == '>').count();
        (l, quote_chars)
    }
}

fn analyze_quotes(lines: &[&str]) -> Option<Vec<Block>> {
    let quotes: Vec<_> = lines
        .iter()
        .map(|line| get_quotes(line))
        .collect();
    if quotes.iter().any(|&(l, _)| l > 0) {
        let mut blocks = vec![];
        let mut quote = &(0, "");
        let mut i = 0;
        let mut idx = 0;
        for this_quote in quotes.iter() {
            if quotes[i].0 != quote.0 {
                if idx < i {
                    blocks.extend(collect_blocks(
                        &lines[idx..i],
                        &quote.1,
                        "",
                    ));
                }
                quote = this_quote;
                idx = i;
            }
            i += 1;
        }
        if idx < i {
            blocks.extend(collect_blocks(
                &lines[idx..i],
                quote.1.clone(),
                "",
            ));
        }
        Some(blocks)
    } else {
        None
    }
}

fn analyze_surround(lines: &[&str]) -> Option<Vec<Block>> {
    let char_slices = lines
        .iter()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut prefix = longest_common_affix(&char_slices[..], Dir::Forward);
    let mut suffix = longest_common_affix(&char_slices[..], Dir::Reverse);

    if prefix == suffix && !prefix.is_empty() {
        prefix = String::from("");
        suffix = String::from("");
    }

    Some(collect_blocks(lines, &prefix, &suffix))
}

fn analyze(lines: &[&str]) -> Vec<Block> {
    let blocks = analyze_quotes(lines).or_else(|| analyze_surround(lines));
    blocks.unwrap()
}

#[derive(Debug)]
struct Entry {
    len: usize,
    offset: usize,
}

impl Entry {
    fn new(len: usize, offset: usize) -> Self {
        Entry { len, offset }
    }
}

#[derive(Default)]
pub struct Reformatter {
    blocks: Vec<Block>,
    target: usize,
    last_line: bool,
    fit: bool,
}

impl Reformatter {
    pub fn new(opts: &FormatOpts, data: &str) -> Self {
        let expanded = spaces(opts.tab_width);
        let data = data.replace("\t", &expanded);
        let input_lines: Vec<_> = data.lines().collect();
        let blocks = analyze(&input_lines);
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
            let cost;
            if j > (count - 2) && !self.last_line {
                //handle last line
                cost = 0;
            } else {
                let diff = (target - linew) as u64;
                cost = diff * diff;
            };
            results.push((j, cost));
        }
        results
    }

    fn solve(&self, words: &[String], target: usize) -> (Vec<usize>, u64) {
        let count = words.len();
        let dummy = Entry::new(0, 0);

        let mut entries = vec![dummy];
        let mut offset = 0;
        for w in words {
            let wid = w.width();
            offset += wid;
            entries.push(Entry::new(wid, offset));
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
            // higher cost the further from original target
            let cost = cost + max_target as u64 - target as u64;
            if cost < best_cost {
                best_cost = cost;
                path = p;
                best_target = target;
            }
        }

        let mut lines: Vec<String> = vec![];

        for s in path.windows(2) {
            if let [start, end] = *s {
                let mut l = block.prefix.clone();
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
                    line.push_str(&block.suffix);
                }
                output.push(line);
            }
        }
        output.join("\n")
    }
}

pub fn reformat(opts: &FormatOpts, data: &str) -> String {
    let rfmt = Reformatter::new(opts, data);
    rfmt.reformatted()
}
