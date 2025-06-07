use miette::{IntoDiagnostic, Result};
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;

pub struct ChatInput {
    editor: DefaultEditor,
}

impl ChatInput {
    pub fn new() -> Result<Self> {
        let editor = DefaultEditor::new().into_diagnostic()?;
        Ok(Self { editor })
    }

    pub fn prompt(&mut self, message: &str) -> Result<Option<String>> {
        match self.editor.readline(message) {
            Ok(input) => {
                let history_retrieve = self.editor.add_history_entry(&input);
                Ok(Some(input))
            }
            Err(ReadlineError::Interrupted) => {
                println!("👋 Goodbye!");
                Ok(None) // User pressed Ctrl+C - exit intent
            }
            Err(other_error) => Err(other_error).into_diagnostic(),
        }
    }
}
