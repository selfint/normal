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