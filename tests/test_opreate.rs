use opengit::operate::open_in_browser;

#[test]
#[ignore]
fn open_in_browser_test() {
    let input = "https://github.com/log1997/opengit";
    let expected = Some(input.to_string());
    assert_eq!(open_in_browser(input.to_string()), expected);
}
