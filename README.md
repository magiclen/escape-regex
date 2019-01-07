Escape Regex (Deprecated)
====================

[![Build Status](https://travis-ci.org/magiclen/escape-regex.svg?branch=master)](https://travis-ci.org/magiclen/escape-regex)
[![Build status](https://ci.appveyor.com/api/projects/status/3pg0fwrp5altj7y5/branch/master?svg=true)](https://ci.appveyor.com/project/magiclen/escape-regex/branch/master)

Escape regular expression special characters in order to make it able to be concatenated safely by users.

## Example

Use the `escape` function of the `regex` crate, instead of this crate.

```rust
extern crate regex;

let pattern = "123*456";
let escaped_pattern = regex::escape(pattern);

let reg = regex::Regex::new(&escaped_pattern).unwrap();

assert_eq!(true, reg.is_match("0123*4567"));
```

## License

[MIT](LICENSE)