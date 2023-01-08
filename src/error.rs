use std::fmt::Display;

use colored::{Colorize, ColoredString};
use crossterm_cursor::cursor;

#[derive(Clone, Debug, Default)]
pub struct LLFeError {
    pub description: String,
    pub caused_by: Option<Box<Self>>,
    // pub position: Position,
}


impl LLFeError {
    fn display(&self) -> ColoredString {
        (&self.description).red()
    }
}


impl Display for LLFeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut c = cursor();

        writeln!(f, "Compiling program produced")?;


        let mut inner_error = match self.caused_by.as_ref() {
            Some(o) => o,
            None => return Ok(()),
        };

        let mut counter = 0;

        loop {
            writeln!(f, "{} - {}", format_counter(counter), (*inner_error).display())?;

            // If next error is some, continue loop and update error
            // If next error is none, break out of loop
            inner_error = match (**inner_error).caused_by.as_ref() {
                Some(inner_err) => inner_err,
                None => break,
            };

            // Increment error counter
            counter += 1;
        };

        let _ = c.save_position();
        let _ = c.move_up(counter as u16 + 2);
        let _ = c.move_right("Compiling program produced".len() as u16 + 1);

        print!("{counter} errors");

        let _ = c.restore_position();

        Ok(())
    }
}


fn format_counter(n: usize) -> ColoredString {
    format!("{:0>4}", format!("{:X}", n)).cyan()
}
