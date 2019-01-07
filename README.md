Escape Regex
====================

[![Build Status](https://travis-ci.org/magiclen/escape-regex.svg?branch=master)](https://travis-ci.org/magiclen/escape-regex)

Escape regular expression special characters in order to make it able to be concatenated safely by users.

## Example

```rust
extern crate escape_regex;

extern crate regex;

use regex::Regex;

let pattern = "123*456";
let escaped_pattern = escape_regex::escape_string(pattern);

let reg = Regex::new(&escaped_pattern).unwrap();

assert_eq!(true, reg.is_match("0123*4567"));
```

## Crates.io

https://crates.io/crates/escape-regex

## Documentation

https://docs.rs/escape-regex

## License

[MIT](LICENSE)