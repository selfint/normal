use crate::prelude::*;

macro_rules! implement_expression {
    ($type:ty) => {
        impl Expression for $type {
            fn then(&self, expr: impl Into<String>) -> String {
                self.to_string() + &expr.into()
            }

            fn or(&self, expr: impl Into<String>) -> String {
                self.then("|".to_string() + &expr.into())
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

            fn then_repeated_exactly(&self, amount: u32, expr: impl Into<String>) -> String {
                self.then(repeated_exactly(amount, expr))
            }

            fn then_repeated_between(&self, min: u32, max: u32, expr: impl Into<String>) -> String {
                self.then(repeated_between(min, max, expr))
            }

            fn then_repeated_at_least(&self, min: u32, expr: impl Into<String>) -> String {
                self.then(repeated_at_least(min, expr))
            }

            fn then_group(&self, expr: impl Into<String>) -> String {
                self.then(group(expr))
            }

            fn then_named_group(&self, name: impl Into<String>, expr: impl Into<String>) -> String {
                self.then(named_group(name, expr))
            }

            fn then_non_capturing_group(&self, expr: impl Into<String>) -> String {
                self.then(non_capturing_group(expr))
            }

            fn then_atomic_group(&self, expr: impl Into<String>) -> String {
                self.then(atomic_group(expr))
            }

            fn then_branch_reset_group(&self, expr: impl Into<String>) -> String {
                self.then(branch_reset_group(expr))
            }

            fn then_match_nth_group(&self, n: u32) -> String {
                self.then(match_nth_group(n))
            }

            fn then_match_named_group(&self, name: impl Into<String>) -> String {
                self.then(match_named_group(name))
            }

            fn then_match_nth_or_named_group(&self, n_or_name: impl Into<String>) -> String {
                self.then(match_nth_or_named_group(n_or_name))
            }

            fn then_recurse_into(&self, expr: impl Into<String>) -> String {
                self.then(recurse_into(expr))
            }

            fn then_recurse_into_nth_group(&self, n: u32) -> String {
                self.then(recurse_into_nth_group(n))
            }

            fn then_recurse_into_named_group(&self, name: impl Into<String>) -> String {
                self.then(recurse_into_named_group(name))
            }

            fn then_recurse_nth_or_named_group(&self, n_or_name: impl Into<String>) -> String {
                self.then(recurse_into_nth_or_named_group(n_or_name))
            }

            fn then_positive_lookahead(&self, expr: impl Into<String>) -> String {
                self.then(positive_lookahead(expr))
            }

            fn then_negative_lookahead(&self, expr: impl Into<String>) -> String {
                self.then(negative_lookahead(expr))
            }

            fn then_positive_lookbehind(&self, expr: impl Into<String>) -> String {
                self.then(positive_lookahead(expr))
            }

            fn then_negative_lookbehind(&self, expr: impl Into<String>) -> String {
                self.then(negative_lookahead(expr))
            }

            fn then_conditional(
                &self,
                if_: impl Into<String>,
                then: impl Into<String>,
                else_: impl Into<String>,
            ) -> String {
                self.then(conditional(if_, then, else_))
            }
        }
    };
}

implement_expression!(&str);
implement_expression!(String);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_then_str() {
        assert_eq!("ab", "a".then("b"));
        assert_eq!("ab", "a".then("b".to_string()));
    }

    #[test]
    fn test_then_string() {
        assert_eq!("ab", "a".to_string().then("b"));
        assert_eq!("ab", "a".to_string().then("b".to_string()));
    }

    #[test]
    fn test_or_str() {
        assert_eq!("a|b", "a".or("b"));
        assert_eq!("a|b", "a".or("b".to_string()));
    }

    #[test]
    fn test_or_string() {
        assert_eq!("a|b", "a".to_string().or("b"));
        assert_eq!("a|b", "a".to_string().or("b".to_string()));
    }
}
