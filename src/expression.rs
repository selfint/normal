pub trait Expression<E>
where
    Self: ToString,
    E: Into<Self::Output>,
{
    type Output;

    fn then(&self, expr: E) -> Self::Output;
    fn or(&self, expr: E) -> Self::Output;
    fn then_repeated(&self, expr: E) -> Self::Output;
    fn then_at_least_once(&self, expr: E) -> Self::Output;
    fn then_optional(&self, expr: E) -> Self::Output;
    fn then_repeated_exactly(&self, amount: u32, expr: E) -> Self::Output;
    fn then_repeated_between(&self, min: u32, max: u32, expr: E) -> Self::Output;
    fn then_repeated_at_least(&self, min: u32, expr: E) -> Self::Output;
    fn then_group(&self, expr: E) -> Self::Output;
    fn then_named_group(&self, name: impl Into<String>, expr: E) -> Self::Output;
    fn then_non_capturing_group(&self, expr: E) -> Self::Output;
    fn then_atomic_group(&self, expr: E) -> Self::Output;
    fn then_branch_reset_group(&self, expr: E) -> Self::Output;
}
