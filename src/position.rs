// TODO Add position
#[derive(Clone, Debug)]
pub enum Position {
    Character {
        line: usize,
        column: usize,
    },
    Range {
        start: Box<Position>,
        end: Box<Position>,
    },
    EOF,
}

impl Default for Position {
    fn default() -> Self {
        Self::EOF
    }
}