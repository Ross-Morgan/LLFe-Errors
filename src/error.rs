use std::fmt::Display;

#[derive(Clone, Debug, Default)]
pub struct LLFeError {
    pub description: String,
    pub caused_by: Option<Box<Self>>,
    // pub position: Position,
}


impl LLFeError {
    fn display(&self) -> &String {
        &self.description
    }
}


impl Display for LLFeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // <Self as ErrorDisplay>::display(&self, n);
        writeln!(f, "Error:")?;

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
        }

        Ok(())
    }
}


fn format_counter(n: usize) -> String {
    format!("{:0>4}", format!("{:X}", n))
}
