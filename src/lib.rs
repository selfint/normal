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
