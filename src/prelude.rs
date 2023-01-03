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
    fn then(&self, expr: impl Into<String>) -> String;

    fn or(&self, other: impl Into<String>) -> String {
        self.then("|".to_string() + &other.into())
    }

    fn then_repeated(&self, expr: impl Into<String>) -> String {
        self.then(repeated(expr))
    }

    fn then_at_least_once(&self, expr: impl Into<String>) -> String {
        self.then(at_least_once(expr))
    }

    fn then_optional(&self, expr: impl Into<String>) -> String {
        self.then(optional(expr))
    }

    fn then_repeated_exactly(&self, expr: impl Into<String>, amount: u32) -> String {
        self.then(repeated_exactly(expr, amount))
    }

    fn then_repeated_between(&self, expr: impl Into<String>, min: u32, max: u32) -> String {
        self.then(repeated_between(expr, min, max))
    }

    fn then_repeated_at_least(&self, expr: impl Into<String>, min: u32) -> String {
        self.then(repeated_at_least(expr, min))
    }

    fn then_group(&self, expr: impl Into<String>) -> String {
        self.then(group(expr))
    }

    fn then_named_group(&self, expr: impl Into<String>, name: impl Into<String>) -> String {
        self.then(named_group(expr, name))
    }

    fn then_non_capturing_group(&self, expr: impl Into<String>) -> String {
        self.then(non_capturing_group(expr))
    }

    fn then_atomic_group(&self, expr: impl Into<String>) -> String {
        self.then(atomic_group(expr))
    }
}

pub fn repeated(expr: impl Into<String>) -> String {
    expr.into() + "*"
}

pub fn at_least_once(expr: impl Into<String>) -> String {
    expr.into() + "+"
}

pub fn optional(expr: impl Into<String>) -> String {
    expr.into() + "?"
}

pub fn repeated_exactly(expr: impl Into<String>, amount: u32) -> String {
    expr.into() + "{" + &amount.to_string() + "}"
}

pub fn repeated_between(expr: impl Into<String>, min: u32, max: u32) -> String {
    expr.into() + "{" + &min.to_string() + "," + &max.to_string() + "}"
}

pub fn repeated_at_least(expr: impl Into<String>, min: u32) -> String {
    expr.into() + "{" + &min.to_string() + ",}"
}

pub fn group(expr: impl Into<String>) -> String {
    "(".to_string() + &expr.into() + ")"
}

pub fn named_group(group: impl Into<String>, name: impl Into<String>) -> String {
    "(?P<".to_string() + &name.into() + ">" + &group.into() + ")"
}

pub fn non_capturing_group(group: impl Into<String>) -> String {
    "(?:".to_string() + &group.into() + ")"
}

pub fn atomic_group(group: impl Into<String>) -> String {
    "(?>".to_string() + &group.into() + ")"
}

#[cfg(test)]
mod tests {
    use super::*;

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
