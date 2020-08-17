#[derive(Clone, PartialEq, Debug, Default)]
pub struct IndentationLevel {
    stack: Vec<usize>,
}

impl IndentationLevel {
    pub fn new() -> IndentationLevel {
        IndentationLevel { stack: vec![] }
    }

    pub fn level(&self) -> usize {
        *self.stack.last().unwrap_or(&0)
    }

    pub fn update(&mut self, level: usize) {
        if level > self.level() {
            self.stack.push(level);
        } else if level < self.level() {
            self.stack.pop();
        }
    }
}
