use unicode_width::UnicodeWidthStr;
use pathfinding::prelude::dijkstra;

pub struct FormatOpts {
    pub max_length: usize,
    pub last_line: bool,
    pub reduce_jaggedness: bool
}

impl Default for FormatOpts {
    fn default() -> Self {
        FormatOpts { max_length: 72, last_line: false, reduce_jaggedness: false }
    }
}

// impl FormatOpts {
//     pub fn new() -> Self {
//         Self::default()
//     }
// }

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

fn determine_indentation(lines: Vec<&str>) -> (usize, usize) {
    let first_indent = indentation(&lines[0]);
    let indent = if lines.len() > 1 {
        indentation(&lines[1])
    } else {
        first_indent
    };

    (first_indent, indent)
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
    indent: usize,
    target: usize,
    last_line: bool,
    fit: bool,
    words: Vec<String>
}

impl Reformatter {
    pub fn new(opts: &FormatOpts, data: &str) -> Self {
        let input_lines: Vec<_> = data.lines().collect();
        let (first_indent, indent) = determine_indentation(input_lines);
        let target = *[opts.max_length - indent, 1].iter().max().unwrap();

        let mut words: Vec<_> = data.split_whitespace().map(String::from).collect();
        if first_indent > indent {
            let mut indented = spaces(first_indent - indent);
            indented.push_str(&words[0]);
            words[0] = indented;
        }

        Reformatter {indent, target, words,
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

    fn solve(&self, target: usize) -> (Vec<usize>, u64) {
        let count = self.words.len();
        let dummy = Entry::new(0, 0);

        let mut entries = vec![dummy];
        let mut offset = 0;
        for w in &self.words {
            let len = UnicodeWidthStr::width(&w[..]);
            offset += len;
            entries.push(Entry::new(len, offset));
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

    pub fn reformatted(&self) -> String {
        let min_target = if self.fit {
            self.target / 2
        } else {
            self.target
        };
        let max_target = self.target;

        let mut path = vec![];
        let mut best_cost = 100_000_000;

        for target in (min_target..=max_target).rev() {
            let (p, cost) = self.solve(target);
            if cost < best_cost {
                best_cost = cost;
                path = p;
            }
        }

        let mut lines: Vec<String> = vec![];

        for s in path.windows(2) {
            if let [start, end] = *s {
                let mut line = spaces(self.indent);
                line.push_str(&self.words[start..end].iter().map(|w| w.to_string()).collect::<Vec<_>>().join(" "));
                lines.push(line);
            }
        }

        lines.join("\n")
    }
}

pub fn reformat(opts: &FormatOpts, data: &str) -> String {
    let rfmt = Reformatter::new(opts, data);
    rfmt.reformatted()
}
