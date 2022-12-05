#[derive(Debug)]
pub struct LoxError {
    line: usize,
    message: String,
}

impl LoxError {
    fn error(line: usize, message: String) -> LoxError {
        LoxError { line, message }
    }

    pub fn report(&self, loc: String) {
        eprintln!("[line {}] Error{}: {}", self.line, loc, self.message);
    }
}