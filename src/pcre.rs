pub fn repeated(expr: impl Into<String>) -> String {
    expr.into() + "*"
}

pub fn at_least_once(expr: impl Into<String>) -> String {
    expr.into() + "+"
}

pub fn optional(expr: impl Into<String>) -> String {
    expr.into() + "?"
}

pub fn repeated_exactly(amount: u32, expr: impl Into<String>) -> String {
    expr.into() + "{" + &amount.to_string() + "}"
}

pub fn repeated_between(min: u32, max: u32, expr: impl Into<String>) -> String {
    expr.into() + "{" + &min.to_string() + "," + &max.to_string() + "}"
}

pub fn repeated_at_least(min: u32, expr: impl Into<String>) -> String {
    expr.into() + "{" + &min.to_string() + ",}"
}

pub fn group(expr: impl Into<String>) -> String {
    "(".to_string() + &expr.into() + ")"
}

pub fn named_group(name: impl Into<String>, expr: impl Into<String>) -> String {
    "(?P<".to_string() + &name.into() + ">" + &expr.into() + ")"
}

pub fn non_capturing_group(expr: impl Into<String>) -> String {
    "(?:".to_string() + &expr.into() + ")"
}

pub fn atomic_group(expr: impl Into<String>) -> String {
    "(?>".to_string() + &expr.into() + ")"
}

pub fn branch_reset_group(expr: impl Into<String>) -> String {
    "(?|".to_string() + &expr.into() + ")"
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!("a{2}", repeated_exactly(2, "a"));
    }

    #[test]
    fn test_repeated_between() {
        assert_eq!("a{2,5}", repeated_between(2, 5, "a",));
    }

    #[test]
    fn test_repeated_at_least() {
        assert_eq!("a{2,}", repeated_at_least(2, "a"));
    }

    #[test]
    fn test_group() {
        assert_eq!("(abc)", group("abc"));
    }

    #[test]
    fn test_named_group() {
        assert_eq!("(?P<group>abc)", named_group("group", "abc"));
    }

    #[test]
    fn test_named_non_capturing_group() {
        assert_eq!("(?:abc)", non_capturing_group("abc"));
    }

    #[test]
    fn test_atomic_group() {
        assert_eq!("(?>abc)", atomic_group("abc"));
    }

    #[test]
    fn test_branch_reset_group() {
        assert_eq!("(?|abc)", branch_reset_group("abc"));
    }
}
