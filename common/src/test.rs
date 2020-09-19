#[derive(Debug, Copy, Clone)]
pub struct Context<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub requires_human_interaction: bool,
    pub should_panic: bool,
    pub timeout_ms: u32,
}

#[derive(Debug, Copy, Clone)]
pub struct Result {
    pub did_pass: bool,
}
