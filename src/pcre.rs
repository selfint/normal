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

pub fn match_nth_group(n: u32) -> String {
    "\\".to_string() + &n.to_string()
}

pub fn match_named_group(name: impl Into<String>) -> String {
    "(?P=".to_string() + &name.into() + ")"
}

pub fn match_nth_or_named_group(n_or_name: impl Into<String>) -> String {
    "\\g{".to_string() + &n_or_name.into() + "}"
}

pub fn recurse_into(expr: impl Into<String>) -> String {
    "(?".to_string() + &expr.into() + ")"
}

pub fn recurse_into_nth_group(n: u32) -> String {
    "(?".to_string() + &n.to_string() + ")"
}

pub fn recurse_into_named_group(name: impl Into<String>) -> String {
    "(?&".to_string() + &name.into() + ")"
}

pub fn recurse_into_nth_or_named_group(n_or_name: impl Into<String>) -> String {
    "\\g<".to_string() + &n_or_name.into() + ">"
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

    #[test]
    fn test_match_nth_group() {
        assert_eq!("\\1", match_nth_group(1));
    }

    #[test]
    fn test_match_named_group() {
        assert_eq!("(?P=name)", match_named_group("name"));
    }

    #[test]
    fn test_match_nth_or_named_group() {
        assert_eq!(
            "\\g{name_or_number}",
            match_nth_or_named_group("name_or_number")
        );
    }

    #[test]
    fn test_recurse_into() {
        assert_eq!("(?abc)", recurse_into("abc"));
    }

    #[test]
    fn test_recurse_into_nth_group() {
        assert_eq!("(?1)", recurse_into_nth_group(1));
    }

    #[test]
    fn test_recurse_into_named_group() {
        assert_eq!("(?&name)", recurse_into_named_group("name"))
    }

    #[test]
    fn test_recurse_nth_or_named_group() {
        assert_eq!(
            "\\g<name_or_number>",
            recurse_into_nth_or_named_group("name_or_number")
        );
    }
}
