pub trait Expression
where
    Self: ToString,
{
    fn then(&self, expr: impl Into<String>) -> String;
    fn or(&self, expr: impl Into<String>) -> String;
    fn then_repeated(&self, expr: impl Into<String>) -> String;
    fn then_at_least_once(&self, expr: impl Into<String>) -> String;
    fn then_optional(&self, expr: impl Into<String>) -> String;
    fn then_repeated_exactly(&self, amount: u32, expr: impl Into<String>) -> String;
    fn then_repeated_between(&self, min: u32, max: u32, expr: impl Into<String>) -> String;
    fn then_repeated_at_least(&self, min: u32, expr: impl Into<String>) -> String;
    fn then_group(&self, expr: impl Into<String>) -> String;
    fn then_named_group(&self, name: impl Into<String>, expr: impl Into<String>) -> String;
    fn then_non_capturing_group(&self, expr: impl Into<String>) -> String;
    fn then_atomic_group(&self, expr: impl Into<String>) -> String;
    fn then_branch_reset_group(&self, expr: impl Into<String>) -> String;
    fn then_match_nth_group(&self, n: u32) -> String;
    fn then_match_named_group(&self, name: impl Into<String>) -> String;
    fn then_match_nth_or_named_group(&self, n_or_name: impl Into<String>) -> String;
    fn then_recurse_into(&self, expr: impl Into<String>) -> String;
    fn then_recurse_into_nth_group(&self, n: u32) -> String;
    fn then_recurse_into_named_group(&self, name: impl Into<String>) -> String;
    fn then_recurse_nth_or_named_group(&self, n_or_name: impl Into<String>) -> String;
    fn then_positive_lookahead(&self, expr: impl Into<String>) -> String;
    fn then_negative_lookahead(&self, expr: impl Into<String>) -> String;
    fn then_positive_lookbehind(&self, expr: impl Into<String>) -> String;
    fn then_negative_lookbehind(&self, expr: impl Into<String>) -> String;
    fn then_conditional(
        &self,
        if_: impl Into<String>,
        then: impl Into<String>,
        else_: impl Into<String>,
    ) -> String;
}
