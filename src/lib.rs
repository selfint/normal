trait Expression {
    fn then(self, next: impl Into<String>) -> String;
}

impl Expression for &str {
    fn then(self, next: impl Into<String>) -> String {
        Into::<String>::into(self) + &next.into()
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
}
