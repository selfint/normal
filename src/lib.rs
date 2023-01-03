pub const DIGIT: &str = "\\d";
pub const NON_DIGIT: &str = "\\D";
pub const WHITESPACE: &str = "\\s";
pub const NON_WHITESPACE: &str = "\\S";
pub const WORD: &str = "\\w";
pub const NON_WORD: &str = "\\W";
pub const WORD_BOUNDARY: &str = "\\b";
pub const NON_WORD_BOUNDARY: &str = "\\B";
pub const NEWLINE: &str = "\\n";
pub const NON_NEWLINE: &str = ".";

pub const START: &str = "^";
pub const START_FORCED: &str = "\\A";
pub const END: &str = "$";
pub const END_FORCED: &str = "\\Z";

pub const START_OF_MATCH: &str = "\\G";

pub const CARRIAGE_RETURN: &str = "\\r";
pub const TAB: &str = "\\r";
pub const NULL: &str = "\\0";
pub const BACKSPACE: &str = "[\\b]";
pub const ESCAPE: &str = "\\e";

pub trait Expression {
    fn then(self, next: impl Into<String>) -> String;
    fn or(self, other: impl Into<String>) -> String;
    fn then_repeated(self, next: impl Into<String>) -> String;
    fn then_at_least_once(self, next: impl Into<String>) -> String;
    fn then_optional(self, next: impl Into<String>) -> String;
    fn then_repeated_exactly(self, next: impl Into<String>, amount: u32) -> String;
    fn then_repeated_between(self, next: impl Into<String>, min: u32, max: u32) -> String;
    fn then_repeated_at_least(self, next: impl Into<String>, min: u32) -> String;
    fn then_group(self, group: impl Into<String>) -> String;
}

impl Expression for &str {
    fn then(self, next: impl Into<String>) -> String {
        self.to_string() + &next.into()
    }

    fn or(self, other: impl Into<String>) -> String {
        self.to_string() + "|" + &other.into()
    }

    fn then_repeated(self, next: impl Into<String>) -> String {
        self.to_string() + &next.into() + "*"
    }

    fn then_at_least_once(self, next: impl Into<String>) -> String {
        self.to_string() + &next.into() + "+"
    }

    fn then_optional(self, next: impl Into<String>) -> String {
        self.to_string() + &next.into() + "?"
    }

    fn then_repeated_exactly(self, next: impl Into<String>, amount: u32) -> String {
        self.to_string() + &next.into() + "{" + &amount.to_string() + "}"
    }

    fn then_repeated_between(self, next: impl Into<String>, min: u32, max: u32) -> String {
        self.to_string() + &next.into() + "{" + &min.to_string() + "," + &max.to_string() + "}"
    }

    fn then_repeated_at_least(self, next: impl Into<String>, min: u32) -> String {
        self.to_string() + &next.into() + "{" + &min.to_string() + ",}"
    }

    fn then_group(self, group: impl Into<String>) -> String {
        self.to_string() + "(" + &group.into() + ")"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_then() {
        let expected = "ab";

        let actual_str = "a".then("b");
        let actual_string_1 = "a".to_string().then("b");
        let actual_string_2 = "a".then("b".to_string());
        let actual_string_3 = "a".to_string().then("b".to_string());

        assert_eq!(expected, actual_str);
        assert_eq!(expected, actual_string_1);
        assert_eq!(expected, actual_string_2);
        assert_eq!(expected, actual_string_3);
    }

    #[test]
    fn test_or() {
        let expected = "a|b";

        let actual = "a".or("b");

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_then_repeated() {
        let expected = "a*";

        let actual = "".then_repeated("a");

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_then_at_least_once() {
        let expected = "a+";

        let actual = "".then_at_least_once("a");

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_then_repeated_exactly() {
        let expected = "a{2}";

        let actual = "".then_repeated_exactly("a", 2);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_then_repeated_between() {
        let expected = "a{2,5}";

        let actual = "".then_repeated_between("a", 2, 5);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_then_repeated_at_least() {
        let expected = "a{2,}";

        let actual = "".then_repeated_at_least("a", 2);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_then_group() {
        let expected = "(abc)";

        let actual = "".then_group("abc");

        assert_eq!(expected, actual);
    }
}
