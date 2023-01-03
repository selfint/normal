//! # Normal
//! A simple library for writing readable regular expressions.
/// ```
/// # use normal::prelude::*;
/// let expression = "example"
///         .then_repeated_between(
///             2,
///             5,
///             named_group(
///                 "name",
///                 "named group"
///                     .then_non_capturing_group("non capturing group")
///                     .then("repeated 2 to 5 times")
///             ),
///         )
///         .then(NEWLINE);
///
/// let raw = "example(?P<name>named group(?:non capturing group)repeated 2 to 5 times){2,5}\\n";
///
/// assert_eq!(expression, raw);
/// ```
pub mod prelude;

use prelude::*;

impl Expression for &str {
    fn then(&self, expr: impl Into<String>) -> String {
        self.to_string() + &expr.into()
    }
}

impl Expression for String {
    fn then(&self, expr: impl Into<String>) -> String {
        self.to_string() + &expr.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_then_str() {
        let expected = "ab";

        let actual_str = "a".then("b");
        let actual_string = "a".then("b".to_string());

        assert_eq!(expected, actual_str);
        assert_eq!(expected, actual_string);
    }

    #[test]
    fn test_then_string() {
        let expected = "ab";

        let actual_str = "a".to_string().then("b");
        let actual_string = "a".to_string().then("b".to_string());

        assert_eq!(expected, actual_str);
        assert_eq!(expected, actual_string);
    }
}
