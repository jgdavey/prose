extern crate prose;

#[test]
fn test_blank_string() {
    let opts = prose::FormatOpts::default();
    let data = "";
    let reformatter = prose::Reformatter::new(&opts, data);
    let result = reformatter.reformatted();
    assert_eq!(result, "");
}
