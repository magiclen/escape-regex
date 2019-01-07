/*!
# Escape Regex

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
*/

static ESCAPE_CHARS: [char; 14] = ['$', '(', ')', '*', '+', '.', '?', '[', '\\', ']', '^', '{', '|', '}'];

pub fn escape_string<S: AsRef<str>>(s: S) -> String {
    let mut chars: Vec<char> = Vec::new();

    for c in s.as_ref().chars() {
        if let Ok(_) = ESCAPE_CHARS.binary_search(&c) {
            chars.push('\\');
        }
        chars.push(c);
    }

    chars.into_iter().collect()
}