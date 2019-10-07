extern crate prose;

mod diff;

use prose::{FormatOpts, Reformatter};

#[test]
fn test_blank_string() {
    let opts = FormatOpts::default();
    let data = "";
    let reformatter = Reformatter::new(&opts, data);
    let result = reformatter.reformatted();
    assert_eq!(result, "");
}

#[test]
fn test_widths() {
    let opts = FormatOpts::with_max_length(40);
    let data = include_str!("data/inputs/comments.txt");
    let reformatter = Reformatter::new(&opts, data);
    let mut actual = reformatter.reformatted();
    actual.push_str("\n"); // usually by virtue of println
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
    let reformatter = Reformatter::new(&opts, data);
    let mut actual = reformatter.reformatted();
    actual.push_str("\n"); // usually by virtue of println
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
    let reformatter = Reformatter::new(&opts, data);
    let mut actual = reformatter.reformatted();
    actual.push_str("\n"); // usually by virtue of println
    let expected = include_str!("data/outputs/email_40.txt");
    assert_diff!(expected, &actual);
}
