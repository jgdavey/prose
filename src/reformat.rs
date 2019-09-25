use unicode_width::UnicodeWidthStr;
use pathfinding::prelude::dijkstra;
use itertools::Itertools;

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

#[derive(Debug,PartialEq)]
enum Box {
    Word(String),
    BlankLine
}

impl Box {
    fn is_blank(&self) -> bool {
        match self {
            Self::BlankLine => true,
            _ => false
        }
    }
}

impl ToString for Box {
    fn to_string(&self) -> String {
        match self {
            Self::Word(w) => w.to_string(),
            Self::BlankLine => "".to_string()
        }
    }
}

trait Width {
    fn width(&self) -> usize;
}

impl Width for String {
    fn width(&self) -> usize {
        UnicodeWidthStr::width(&self[..])
    }
}

impl Width for &str {
    fn width(&self) -> usize {
        UnicodeWidthStr::width(*self)
    }
}

fn indentation(s: &str) -> usize {
    let mut i = 0;
    for c in s.chars() {
        if c.is_whitespace() {
            i += 1;
        } else {
            return i;
        }
    }
    0
}

struct Analysis {
    prefix: String,
    suffix: String,
    words: Vec<Box>
}

// returns prefix, suffix, words separated by spaces
fn extract_from_lines(lines: &[&str]) -> (Vec<char>, Vec<char>, Vec<Box>) {
    let charlines = lines.iter()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut i = 0;
    let mut prefix = None;
    let mut prefixvec = vec![];
    let mut suffix = None;
    let mut suffixvec = vec![];
    if charlines.len() > 1 {
        loop {
            let mut p = None;
            let mut s = None;
            if prefix.is_some() && suffix.is_some() {
                break;
            }
            for line in &charlines {
                let len = line.len();
                if i == len {
                    p = None;
                    s = None;
                    prefix = Some(i);
                    suffix = Some(i);
                    break;
                }
                if prefix.is_none() {
                    match p {
                        None => {
                            p = Some(line[i]);
                        }
                        Some(letter) if letter != line[i] => {
                            p = None;
                            prefix = Some(i);
                        },
                        _ => ()
                    }
                }
                if suffix.is_none() {
                    match s {
                        None => {
                            s = Some(line[len - i - 1]);
                        },
                        Some(letter) if letter != line[len - i - 1] => {
                            s = None;
                            suffix = Some(i);
                        },
                        _ => ()
                    }

                }
            }
            if let Some(letter) = p {
                prefixvec.push(letter);
            }
            if let Some(letter) = s {
                suffixvec.push(letter);
            }
            i += 1;
        }
    }
    suffixvec.reverse();
    let mut words: Vec<Box> = vec![];
    for line in &charlines {
        let len = line.len();
        let trimmed = &line[prefixvec.len()..(len - suffixvec.len())];
        let mut word = String::new();
        let mut found_non_blank = false;
        for c in trimmed {
            if c.is_whitespace() {
                if !word.is_empty() {
                    words.push(Box::Word(word));
                    word = String::new();
                }
            } else {
                found_non_blank = true;
                word.push(*c);
            }
        }
        if !word.is_empty() {
            words.push(Box::Word(word));
        } else if !found_non_blank {
            words.push(Box::BlankLine);
        }
    }

    (prefixvec, suffixvec, words)
}

fn analyze(lines: &[&str]) -> Analysis {
    let (prefixvec, suffixvec, words) = extract_from_lines(&lines);
    let first_line = lines[0];
    let fl_without_prefix: String = first_line.chars().skip(prefixvec.len()).collect();
    let first_indent = indentation(&fl_without_prefix);
    let prefix: String = prefixvec.iter().collect();
    let suffix: String = suffixvec.iter().collect();
    let mut words = words;

    if first_indent > 0 {
        if let Box::Word(w) = &words[0] {
            let mut indented = spaces(first_indent).to_string();
            indented.push_str(w);
            words[0] = Box::Word(indented);
        }
    }

    Analysis { words, prefix, suffix }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn boxify(v: Vec<&str>) -> Vec<Box> {
        v.iter().map(|i| {
            if *i == "" {
                Box::BlankLine
            } else {
                Box::Word(i.to_string())
            }
        }).collect()
    }

    #[test]
    fn analyze_simple_prefix_test() {
        let input = vec!["> Line one!",
                         "> line two?"];
        let analysis = analyze(&input);
        assert_eq!(analysis.words, boxify(vec!["Line", "one!", "line", "two?"]));
        assert_eq!(analysis.prefix, "> ".to_string());
        assert_eq!(analysis.suffix, "".to_string());
    }
    #[test]
    fn analyze_first_line_indent_test() {
        let input = vec!["  First line,",
                         "and second one."];
        let analysis = analyze(&input);
        assert_eq!(analysis.words, boxify(vec!["  First", "line,", "and", "second", "one."]));
        assert_eq!(analysis.prefix, "".to_string());
        assert_eq!(analysis.suffix, "".to_string());
    }
    #[test]
    fn analyze_single_line_test() {
        let input = vec!["First line, and actually the only line."];
        let analysis = analyze(&input);
        assert_eq!(analysis.words, boxify(vec!["First", "line,", "and", "actually", "the", "only", "line."]));
        assert_eq!(analysis.prefix, "".to_string());
        assert_eq!(analysis.suffix, "".to_string());
    }
    #[test]
    fn analyze_blank_lines_test() {
        let input = vec!["Not blank", " ", "not blank again"];
        let analysis = analyze(&input);
        assert_eq!(analysis.words, boxify(vec!["Not", "blank", "", "not", "blank", "again"]));
        assert_eq!(analysis.prefix, "".to_string());
        assert_eq!(analysis.suffix, "".to_string());
    }

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
    prefix: String,
    suffix: String,
    suffix_length: usize,
    target: usize,
    last_line: bool,
    fit: bool,
    words: Vec<Box>
}

impl Reformatter {
    pub fn new(opts: &FormatOpts, data: &str) -> Self {
        let expanded = spaces(opts.tab_width);
        let data = data.replace("\t", &expanded);
        let input_lines: Vec<_> = data.lines().collect();
        let Analysis {prefix, suffix, words} = analyze(&input_lines);
        let prefix_length = prefix.width();
        let suffix_length = suffix.width();
        let rawtarget = opts.max_length as i64 - prefix_length as i64 - suffix_length as i64;
        let target = std::cmp::max(rawtarget, 1) as usize;
        // eprintln!("Prefix: {}, Suffix: {}, Max: {}, Target: {}", prefix, suffix, opts.max_length, target);
        Reformatter {prefix, suffix, target, words, suffix_length,
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

    fn solve(&self, words: &[Box], target: usize) -> (Vec<usize>, u64) {
        let count = words.len();
        let dummy = Entry::new(0, 0);

        let mut entries = vec![dummy];
        let mut offset = 0;
        for w in words {
            let wid = match w {
                Box::Word(w) => w.width(),
                Box::BlankLine => target
            };
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

    fn reformat_section(&self, words: &[Box]) -> (Vec<String>, usize) {
        let min_target = if self.fit {
            self.target / 2
        } else {
            self.target
        };
        let max_target = self.target;

        let mut path = vec![];
        let mut best_cost = 100_000_000;
        let mut best_target = max_target;

        for target in (min_target..=max_target).rev() {
            let (p, cost) = self.solve(words, target);
            if cost < best_cost {
                best_cost = cost;
                path = p;
                best_target = target;
            }
        }

        let mut lines: Vec<String> = vec![];

        for s in path.windows(2) {
            if let [start, end] = *s {
                lines.push(words[start..end].iter().map(|w| w.to_string()).collect::<Vec<_>>().join(" "));
            }
        }
        (lines, best_target)
    }

    pub fn reformatted(&self) -> String {
        // get "unadorned" body
        let sections: Vec<_> = self.words
            .split(|w| w.is_blank())
            .filter(|s| !s.is_empty())
            .map(|s| self.reformat_section(s))
            .collect();
        let max_padding = sections.iter().map(|s| s.1).max().unwrap_or(self.target) as i64;

        let mut output = vec![];

        for body in sections.iter().map(|s| &s.0).intersperse(&vec!["".to_string()]) {
            for l in body {
                let mut line = self.prefix.clone();
                line.push_str(&l);
                if self.suffix_length > 0 {
                    let pad_amount = max_padding - l.width() as i64;
                    let pad = spaces(std::cmp::max(pad_amount, 0i64) as usize);
                    line.push_str(&pad);
                    line.push_str(&self.suffix);
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
