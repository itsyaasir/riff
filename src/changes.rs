#[derive(Debug, Clone, Copy)]
/// This enum represents three types of text modifications
pub enum Change {
    /// This variant represent where a character is inserted at a specified position.
    Insertion(char, usize),

    /// This variant shows where a character is deleted from a specified position
    Deletion(char, usize),

    ///Shows where an existing character at a specified position is replaced by another one
    Substitution(char, char, usize),
}
