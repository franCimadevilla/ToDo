use rustyline::Editor;
use rustyline::history::DefaultHistory;

/// Trait for abstracting the read/edit of lines with initial values in the UI
pub trait LineEditor {
    fn readline_with_initial(
        &mut self,
        prompt: &str,
        initial: (&str, &str),
    ) -> Result<String, String>;
}

impl LineEditor for Editor<(), DefaultHistory> {
    fn readline_with_initial(
        &mut self,
        prompt: &str,
        initial: (&str, &str),
    ) -> Result<String, String> {
        Editor::readline_with_initial(self, prompt, initial)
            .map_err(|e| format!("Failed in the line editor: {}", e))
    }
}

pub struct MockLineEditor {
    inputs: Vec<String>,
    index: usize,
}

impl MockLineEditor {
    pub fn new(inputs: Vec<String>) -> Self {
        MockLineEditor { inputs, index: 0 }
    }
}

impl LineEditor for MockLineEditor {
    fn readline_with_initial(
        &mut self,
        _prompt: &str,
        _initial: (&str, &str),
    ) -> Result<String, String> {
        if self.index < self.inputs.len() {
            let input = self.inputs[self.index].clone();
            self.index += 1;
            Ok(input)
        } else {
            Err("No more inputs available".to_string())
        }
    }
}
