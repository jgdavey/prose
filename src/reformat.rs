use unicode_width::UnicodeWidthStr;
use pathfinding::prelude::dijkstra;

pub struct FormatOpts {
    pub max_length: usize,
    pub tab_width: usize,
    pub last_line: bool,
    pub reduce_jaggedness: bool
}

impl Default for FormatOpts {
    fn default() -> Self {
        FormatOpts { max_length: 72, last_line: false, reduce_jaggedness: false, tab_width: 4 }
    }
}

#[allow(dead_code)]
impl FormatOpts {
    pub fn with_max_length(max_length: usize) -> Self {
        Self { max_length, ..Default::default() }
    }

    pub fn new(max_length: usize, last_line: bool, reduce_jaggedness: bool, tab_width: usize) -> Self {
        Self { max_length, last_line, reduce_jaggedness, tab_width }
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

struct Block {
    prefix: String,
    suffix: String,
    words: Vec<String>,
    newline_after: bool
}

enum Dir {
    Forward,
    Reverse
}

fn longest_common_affix(char_slices: &[Vec<char>], dir: Dir) -> Vec<char> {
    let get = |s: &[char], i| {
        match dir {
            Dir::Forward => s[i],
            Dir::Reverse => s[s.len() - i -1]
        }
    };

    if char_slices.is_empty() {
        return vec![];
    }
    let mut ret = Vec::new();
    let mut i = 0;
    loop {
        let mut c = None;
        for s in char_slices {
            if i == s.len() {
                if let Dir::Reverse = dir {
                    ret.reverse();
                }
                return ret;
            }
            match c {
                None => {
                    c = Some(get(s,i));
                }
                Some(letter) if letter != get(s,i) => {
                    if let Dir::Reverse = dir {
                        ret.reverse();
                    }
                    return ret
                },
                _ => continue,
            }
        }
        if let Some(letter) = c {
            ret.push(letter);
        }
        i += 1;
    }
}

fn analyze(lines: &[&str]) -> Vec<Block> {
    let charlines = lines.iter()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut prefix;
    let mut suffix;

    prefix = longest_common_affix(&charlines[..], Dir::Forward);
    suffix = longest_common_affix(&charlines[..], Dir::Reverse);

    if prefix == suffix && prefix.len() > 0 {
        prefix = vec![];
        suffix = vec![];
    }

    let prefixstr: String = prefix.iter().collect();
    let suffixstr: String = suffix.iter().collect();

    let mut blocks: Vec<Block> = vec![];
    let mut words: Vec<String> = vec![];
    let mut indentation = 0;
    for (i, line) in charlines.iter().enumerate() {
        let len = line.len();
        let trimmed = &line[prefix.len()..(len - suffix.len())];
        let mut word = String::new();
        let mut found_non_blank = false;
        for c in trimmed {
            if c.is_whitespace() {
                if !word.is_empty() {
                    words.push(word);
                    word = String::new();
                }
                if words.is_empty() && i == 0 {
                    indentation += 1;
                }
            } else {
                found_non_blank = true;
                word.push(*c);
            }
        }
        if !word.is_empty() {
            words.push(word);
        } else if !found_non_blank {
            if !words.is_empty() {
                let mut s = spaces(indentation);
                s.push_str(&words[0]);
                words[0] = s;
            }
            blocks.push(Block {
                prefix: prefixstr.clone(),
                suffix: suffixstr.clone(),
                words: words,
                newline_after: true
            });
            words = vec![];
            indentation = 0;
        }
    }
    if !words.is_empty() {
        let mut s = spaces(indentation);
        s.push_str(&words[0]);
        words[0] = s;

        blocks.push(Block {
            prefix: prefixstr,
            suffix: suffixstr,
            words: words,
            newline_after: false
        });
    }
    blocks
}

fn spaces(n: usize) -> String {
    std::iter::repeat(" ").take(n).collect::<String>()
}

#[derive(Debug)]
struct Entry {
    len: usize,
    offset: usize
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
        Reformatter {blocks,
                     target: opts.max_length,
                     last_line: opts.last_line,
                     fit: opts.reduce_jaggedness }
    }

    fn successors(&self, entries: &[Entry], i: usize, target: usize, allow_overage: bool) -> Vec<(usize, u64)> {
        let word1 = &entries[i];
        let count = entries.len();
        let mut results = vec![];
        for (j, word2) in entries.iter().enumerate().take(count).skip(i+1) {
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
            if j > (count - 2) && !self.last_line { //handle last line
                cost = 0;
            } else {
                let diff = (target - linew) as u64;
                cost = diff * diff;
            };
            results.push((j,cost));
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

        let result = dijkstra(&0,
                              |i| self.successors(&entries, *i, target, false),
                              |i| *i == count );

        if let Some(tuple) = result {
            tuple
        } else {
            eprintln!("Warning: allowing some words to extend beyond target width");
            // try again, allowing overage
            dijkstra(&0,
                     |i| self.successors(&entries, *i, target, true),
                     |i| *i == count ).expect("Unable to find optimum solution")
        }
    }

    fn reformat_section(&self, block: &Block) -> (Vec<String>, usize) {
        let words = &block.words;
        let rawtarget = self.target as i64
            - block.prefix.width() as i64
            - block.suffix.width() as i64;
        let target = std::cmp::max(rawtarget, 1) as usize;

        let min_target = if self.fit {
            target / 2
        } else {
            target
        };
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
                l.push_str(&words[start..end].iter().map(|w| w.to_string()).collect::<Vec<_>>().join(" "));
                lines.push(l);
            }
        }
        if block.newline_after {
            lines.push(block.prefix.clone());
        }
        (lines, best_target)
    }

    pub fn reformatted(&self) -> String {
        // get "unadorned" body
        let sections: Vec<_> = self.blocks.iter()
            .map(|s| (s, self.reformat_section(s)))
            .collect();
        let max_padding = sections.iter().map(|s| (s.1).1).max().unwrap_or(self.target) as i64;

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
