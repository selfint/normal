trait Expression {
    fn then(self, next: impl Into<String>) -> String;
    fn or(self, other: impl Into<String>) -> String;
    fn then_repeated(self, next: impl Into<String>) -> String;
    fn then_at_least_once(self, next: impl Into<String>) -> String;
    fn then_optional(self, next: impl Into<String>) -> String;
    fn then_repeated_exactly(self, next: impl Into<String>, amount: u32) -> String;
    fn then_repeated_between(self, next: impl Into<String>, min: u32, max: u32) -> String;
    fn then_repeated_at_least(self, next: impl Into<String>, min: u32) -> String;
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
}
