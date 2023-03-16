//! Levenshtein algorithm
//! This function calculates the Levenshtein distance between text1 and text2, which is the minimum number of insertions, deletions, or substitutions required
//! to change one string into the other.
//!
// !The function takes two string slices as arguments, text1 and text2, and returns
//! a vector of Change enum variants representing the differences between the two texts.

use crate::changes::Change;

pub fn levenshtein_diff(text1: &str, text2: &str) -> anyhow::Result<Vec<Change>> {
    todo!("implement the diff")
}

#[cfg(tests)]
mod tests {}
