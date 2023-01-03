use normal::prelude::*;

fn main() {
    println!(
        "{}",
        "this is an example"
            .then(NEWLINE)
            .then("of then")
            .then(NEWLINE)
    );

    println!(
        "{}",
        "this is an example".then_named_group(
            "of a group named name"
                .then_non_capturing_group("with a non capturing group inside it"),
            "name"
        )
    );

    println!(
        "{}",
        "this is an example".then_repeated_between(
            named_group(
                "of a group named name"
                    .then_non_capturing_group("with a non capturing group inside it")
                    .then("that is repeated between 2 to 5 times"),
                "name"
            ),
            2,
            5
        )
    );
}
