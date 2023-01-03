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

    #[test]
    fn test_or() {
        assert_eq!("a|b", "a".or("b"));
    }

    #[test]
    fn test_repeated() {
        assert_eq!("a*", repeated("a"));
    }

    #[test]
    fn test_at_least_once() {
        assert_eq!("a+", at_least_once("a"));
    }

    #[test]
    fn test_repeated_exactly() {
        assert_eq!("a{2}", repeated_exactly("a", 2));
    }

    #[test]
    fn test_repeated_between() {
        assert_eq!("a{2,5}", repeated_between("a", 2, 5));
    }

    #[test]
    fn test_repeated_at_least() {
        assert_eq!("a{2,}", repeated_at_least("a", 2));
    }

    #[test]
    fn test_group() {
        assert_eq!("(abc)", group("abc"));
    }

    #[test]
    fn test_named_group() {
        assert_eq!("(?P<group>abc)", named_group("abc", "group"));
    }

    #[test]
    fn test_named_non_capturing_group() {
        assert_eq!("(?:abc)", non_capturing_group("abc"));
    }

    #[test]
    fn test_atomic_group() {
        assert_eq!("(?>abc)", atomic_group("abc"));
    }
}
