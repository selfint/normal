# Normal

A simple library for writing readable regular expressions.

Writing regular expressions isn't (usually) an often occurrence. The goal of this library
is to make reading/writing regular expressions easier, now and in the future, and not needing to remember/re-learn the PCRE syntax each time.

In the future, maybe adding full documentation to all the methods/constants can make this
library a way to also learn PCRE syntax.

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
- [x] Groups:
  - [x] Un-named - `(...)`
  - [x] Named - `(?P<name>...)`
  - [x] Non capturing - `(?...)`
  - [x] Atomic - `(?>...)`
  - [x] Branch reset groups - `(?|...)`
  - [x] Match capture group by number - `\Y`
  - [x] Match capture group by name - `(?P=name)`
  - [x] Match capture group by name or number - `\g{Y}`
  - [x] Recurse into pattern - `(?R)`
  - [x] Recurse into group by number - `(?Y)`
  - [x] Recurse into group by name - `(?&name)`
  - [x] Recurse into group by name or number - `\g<Y>`
  - [ ] Comment - `(?#...)` (do we need this?)
- [x] Constants
- [ ] Assertions:
  - [ ] Positive lookahead - `(?=...)`
  - [ ] Negative lookahead - `(?!...)`
  - [ ] Positive lookbehind - `(?<=...)`
  - [ ] Negative lookbehind - `(?<!...)`
  - [ ] Conditional - `(?()|)`
