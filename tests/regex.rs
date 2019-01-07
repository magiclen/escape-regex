extern crate escape_regex;
extern crate regex;

use regex::Regex;

#[test]
fn basic() {
    assert_eq!(r"Hello\. 1 \+ 1 = 2\. \* Does \[this\] work\?", escape_regex::escape_string("Hello. 1 + 1 = 2. * Does [this] work?"));
}

#[test]
fn real_cases() {
    let sentence = "Someone: /Hello. 1 + 1 = 2. * Does [this] work?/g I hope so.";

    let pattern = "/Hello. 1 + 1 = 2. * Does [this] work?/g";
    let escaped_pattern = escape_regex::escape_string(pattern);

    let regex_1 = Regex::new(pattern).unwrap();
    let regex_2 = Regex::new(&escaped_pattern).unwrap();

    assert_eq!(false, regex_1.is_match(sentence));
    assert_eq!(true, regex_2.is_match(sentence));
}