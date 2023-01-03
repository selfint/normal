# Normal

A simple library for writing readable regular expressions.

## Example

```rs
use normal::prelude::*;

let expression = "example"
        .then_repeated_between(
            2,
            5,
            named_group(
                "name",
                "named group"
                    .then_non_capturing_group("non capturing group")
                    .then("repeated 2 to 5 times")
            ),
        )
        .then(NEWLINE);

let raw = "example(?P<name>named group(?:non capturing group)repeated 2 to 5 times){2,5}\\n";

assert_eq!(expression, raw);
```

## Supported PCRE syntax

Syntax cheat sheet taken from [debuggex.com](https://www.debuggex.com/cheatsheet/regex/pcre).

- [x] Basics:
  - [x] then
  - [x] |
- [x] Quantifiers:
  - [x] *
  - [x] +
  - [x] ?
  - [x] {amount}
  - [x] {min,max}
  - [x] {min,}
- [ ] Groups:
  - [x] Un-named - `(...)`
  - [x] Named - `(?P<name>...)`
  - [x] Non capturing - `(?...)`
  - [x] Atomic - `(?>...)`
  - [ ] Duplicate numbers - `(?|...)`
  - [x] Match capture group by number - `\Y`
  - [x] Match capture group by name - `(?P=name)`
  - [ ] Match capture group by name or number - `\g{Y}`
  - [ ] Recurse into pattern - `(?R)`
  - [ ] Recurse into group by number - `(?Y)`
  - [ ] Recurse into group by name - `(?&name)`
  - [ ] Recurse into group by name or number - `\g<Y>`
  - [ ] Comment - `(?#...)` (do we need this?)
- [x] Constants
- [ ] Assertions:
  - [ ] Positive lookahead - `(?=...)`
  - [ ] Negative lookahead - `(?!...)`
  - [ ] Positive lookbehind - `(?<=...)`
  - [ ] Negative lookbehind - `(?<!...)`
  - [ ] Conditional - `(?()|)`
