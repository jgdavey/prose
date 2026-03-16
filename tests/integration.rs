extern crate prose;

mod diff;

use prose::{FormatMode, FormatOpts};
use std::io::Cursor;

fn process_to_string(input: &str, opts: FormatOpts) -> String {
    let mut out = Vec::<u8>::new();
    prose::process_paragraphs(&mut Cursor::new(input), &mut out, opts).unwrap();
    String::from_utf8(out).unwrap()
}

#[test]
fn test_blank_string() {
    let opts = FormatOpts::default();
    let data = "";
    let result = prose::reformat(&opts, data);
    assert_eq!(result, "");
}

#[test]
fn test_one_line() {
    let opts = FormatOpts::with_max_length(25);
    let data = "Lot's of string data... to be reformatted";
    let actual = prose::reformat(&opts, data);
    assert_eq!(actual, "Lot's of string data...\nto be reformatted");
}

#[test]
fn test_widths() {
    let opts = FormatOpts::with_max_length(40);
    let data = include_str!("data/inputs/comments.txt");
    let mut actual = prose::reformat(&opts, data);
    actual.push('\n'); // usually by virtue of println
    let expected = include_str!("data/outputs/comments_40.txt");
    assert_diff!(expected, &actual);
}

#[test]
fn test_comments_regression() {
    let opts = FormatOpts::with_max_length(40);
    let data = include_str!("data/inputs/comments_regress.txt");
    let mut actual = prose::reformat(&opts, data);
    actual.push('\n'); // usually by virtue of println
    let expected = include_str!("data/outputs/comments_40.txt");
    assert_diff!(expected, &actual);
}

#[test]
fn test_aggressive_fit() {
    let opts = FormatOpts {
        max_length: 50,
        reduce_jaggedness: true,
        ..Default::default()
    };
    let data = include_str!("data/inputs/plain_indented.txt");
    let mut actual = prose::reformat(&opts, data);
    actual.push('\n'); // usually by virtue of println
    let expected = include_str!("data/outputs/plain_indented_50_f.txt");
    assert_diff!(expected, &actual);
}

#[test]
fn test_email_quoting() {
    let opts = FormatOpts {
        max_length: 40,
        ..Default::default()
    };
    let data = include_str!("data/inputs/email.txt");
    let mut actual = prose::reformat(&opts, data);
    actual.push('\n'); // usually by virtue of println
    let expected = include_str!("data/outputs/email_40.txt");
    assert_diff!(expected, &actual);
}

#[test]
fn test_tab_expansion() {
    let opts = FormatOpts {
        max_length: 40,
        ..Default::default()
    };
    let data = include_str!("data/inputs/tabs.txt");
    let mut actual = prose::reformat::reformat(&opts, data);
    actual.push('\n'); // usually by virtue of println
    let expected = include_str!("data/outputs/tabs_40.txt");
    assert_diff!(expected, &actual);
}

#[test]
fn test_utf8_with_prefixes() {
    let opts = FormatOpts {
        max_length: 40,
        ..Default::default()
    };
    let data = include_str!("data/inputs/greek.txt");
    let mut actual = prose::reformat::reformat(&opts, data);
    actual.push('\n'); // usually by virtue of println
    let expected = include_str!("data/outputs/greek_40.txt");
    assert_diff!(expected, &actual);
}

#[test]
fn test_diacritics() {
    let opts = FormatOpts {
        max_length: 40,
        ..Default::default()
    };
    let data = include_str!("data/inputs/diacritics.txt");
    let mut actual = prose::reformat(&opts, data);
    actual.push('\n'); // usually by virtue of println
    let expected = include_str!("data/outputs/diacritics_40.txt");
    assert_diff!(expected, &actual);
}

#[test]
fn test_emoji() {
    let opts = FormatOpts {
        max_length: 40,
        ..Default::default()
    };
    let data = include_str!("data/inputs/emoji.txt");
    let mut actual = prose::reformat(&opts, data);
    actual.push('\n'); // usually by virtue of println
    let expected = include_str!("data/outputs/emoji_40.txt");
    assert_diff!(expected, &actual);
}

#[test]
fn test_markdown() {
    let opts = FormatOpts {
        max_length: 53,
        format_mode: FormatMode::Markdown,
        ..Default::default()
    };
    let data = include_str!("data/inputs/markdown.md");
    let mut actual: String = data
        .split("\n\n")
        .map(|s| prose::reformat::reformat(&opts, s))
        .collect::<Vec<_>>()
        .join("\n\n");
    actual.push('\n'); // usually by virtue of println
    let expected = include_str!("data/outputs/markdown_53.md");
    assert_diff!(expected, &actual);
}

#[test]
fn test_rust_comments() {
    let opts = FormatOpts {
        max_length: 53,
        format_mode: FormatMode::Code,
        ..Default::default()
    };
    let data = include_str!("data/inputs/rust_comments.txt");
    let mut actual: String = data
        .split("\n\n")
        .map(|s| prose::reformat::reformat(&opts, s))
        .collect::<Vec<_>>()
        .join("\n\n");
    actual.push('\n'); // usually by virtue of println
    let expected = include_str!("data/outputs/rust_comments_56.txt");
    assert_diff!(expected, &actual);
}

#[test]
fn process_test_widths() {
    let actual = process_to_string(
        include_str!("data/inputs/comments.txt"),
        FormatOpts::with_max_length(40),
    );
    assert_diff!(include_str!("data/outputs/comments_40.txt"), &actual);
}

#[test]
fn process_test_comments_regression() {
    let actual = process_to_string(
        include_str!("data/inputs/comments_regress.txt"),
        FormatOpts::with_max_length(40),
    );
    assert_diff!(include_str!("data/outputs/comments_40.txt"), &actual);
}

#[test]
fn process_test_plain_last_line() {
    let actual = process_to_string(
        include_str!("data/inputs/plain.txt"),
        FormatOpts {
            max_length: 56,
            last_line: true,
            ..Default::default()
        },
    );
    assert_diff!(include_str!("data/outputs/plain_56_l.txt"), &actual);
}

#[test]
fn process_test_aggressive_fit() {
    let actual = process_to_string(
        include_str!("data/inputs/plain_indented.txt"),
        FormatOpts {
            max_length: 50,
            reduce_jaggedness: true,
            ..Default::default()
        },
    );
    assert_diff!(
        include_str!("data/outputs/plain_indented_50_f.txt"),
        &actual
    );
}

#[test]
fn process_test_tab_expansion() {
    let actual = process_to_string(
        include_str!("data/inputs/tabs.txt"),
        FormatOpts::with_max_length(40),
    );
    assert_diff!(include_str!("data/outputs/tabs_40.txt"), &actual);
}

#[test]
fn process_test_utf8_with_prefixes() {
    let actual = process_to_string(
        include_str!("data/inputs/greek.txt"),
        FormatOpts::with_max_length(40),
    );
    assert_diff!(include_str!("data/outputs/greek_40.txt"), &actual);
}

#[test]
fn process_test_diacritics() {
    let actual = process_to_string(
        include_str!("data/inputs/diacritics.txt"),
        FormatOpts::with_max_length(40),
    );
    assert_diff!(include_str!("data/outputs/diacritics_40.txt"), &actual);
}

#[test]
fn process_test_emoji() {
    let actual = process_to_string(
        include_str!("data/inputs/emoji.txt"),
        FormatOpts::with_max_length(40),
    );
    assert_diff!(include_str!("data/outputs/emoji_40.txt"), &actual);
}

#[test]
fn process_test_markdown() {
    let actual = process_to_string(
        include_str!("data/inputs/markdown.md"),
        FormatOpts {
            max_length: 53,
            format_mode: FormatMode::Markdown,
            ..Default::default()
        },
    );
    assert_diff!(include_str!("data/outputs/markdown_53.md"), &actual);
}

#[test]
fn process_test_rust_comments() {
    let actual = process_to_string(
        include_str!("data/inputs/rust_comments.txt"),
        FormatOpts {
            max_length: 53,
            format_mode: FormatMode::Code,
            ..Default::default()
        },
    );
    assert_diff!(include_str!("data/outputs/rust_comments_56.txt"), &actual);
}

#[test]
fn process_test_yaml_comments() {
    let actual = process_to_string(
        include_str!("data/inputs/comments.yml"),
        FormatOpts {
            max_length: 53,
            format_mode: FormatMode::Code,
            ..Default::default()
        },
    );
    assert_diff!(include_str!("data/outputs/comments_53.yml"), &actual);
}
