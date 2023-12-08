pub trait Problem {
    fn solve_a(&self, input: &str) -> Option<String>;
    fn solve_b(&self, input: &str) -> Option<String>;
}
