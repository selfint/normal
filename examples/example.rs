use lazy_static::lazy_static;

use normal::prelude::*;

lazy_static! {
    static ref STATIC_RE: String = "this is an example"
        .then(NEWLINE)
        .then("of a static regex")
        .then(NEWLINE);
}

fn main() {
    println!("{}", *STATIC_RE);

    println!(
        "{}",
        "this is an example".then_named_group(
            "name",
            "of a group named name"
                .then_non_capturing_group("with a non capturing group inside it"),
        )
    );

    println!(
        "{}",
        "this is an example"
            .then_repeated_between(
                2,
                5,
                named_group(
                    "name",
                    "of a group named name"
                        .then_non_capturing_group("with a non capturing group inside it")
                        .then("that is repeated between 2 to 5 times")
                ),
            )
            .then(NEWLINE)
    );

    println!(
        "{}",
        "this is an exam"
            .then_conditional(positive_lookahead("ple"), "ple exam\\.", "\\.")
            .then(" that matches either example exam, or just exam")
            .then(" followed by a dot")
    )
}
