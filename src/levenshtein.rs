//! Levenshtein algorithm
//! This function calculates the Levenshtein distance between text1 and text2, which is the minimum number of insertions, deletions, or substitutions required
//! to change one string into the other.
//!
// !The function takes two string slices as arguments, text1 and text2, and returns
//! a vector of Change enum variants representing the differences between the two texts.

use crate::changes::Change;
use anyhow::anyhow;
pub fn levenshtein_diff(text1: &str, text2: &str) -> anyhow::Result<Vec<Change>> {
    let mut changes = Vec::new();

    let mut matrix = vec![vec![0; text2.len() + 1]; text1.len() + 1];

    matrix
        .iter_mut()
        .take(text1.len() + 1)
        .enumerate()
        .for_each(|(i, row)| {
            row[0] = i;
        });

    for j in 0..=text2.len() {
        matrix[0][j] = j;
    }

    for i in 1..=text1.len() {
        for j in 1..=text2.len() {
            if text1.chars().nth(i - 1) == text2.chars().nth(j - 1) {
                matrix[i][j] = matrix[i - 1][j - 1];
            } else {
                matrix[i][j] = 1 + std::cmp::min(
                    matrix[i - 1][j],
                    std::cmp::min(matrix[i][j - 1], matrix[i - 1][j - 1]),
                );
            }
        }
    }

    let mut i = text1.len();
    let mut j = text2.len();

    while i != 0 && j != 0 {
        if text1.chars().nth(i - 1) == text2.chars().nth(j - 1) {
            i = i.checked_sub(1).ok_or(anyhow!("Underflow Error"))?;
            j = j.checked_sub(1).ok_or(anyhow!("Underflow Error"))?;
        } else if matrix[i - 1][j] < matrix[i][j - 1] {
            changes.push(Change::Deletion(
                text1.chars().nth(i - 1).ok_or(anyhow!("Underflow Error"))?,
                i - 1,
            ));
            i = i.checked_sub(1).ok_or(anyhow!("Underflow Error"))?;
        } else if matrix[i - 1][j] > matrix[i][j - 1] {
            changes.push(Change::Insertion(
                text2.chars().nth(j - 1).ok_or(anyhow!("Underflow Error"))?,
                j - 1,
            ));
            j = j.checked_sub(1).ok_or(anyhow!("Underflow Error"))?;
        } else {
            changes.push(Change::Substitution(
                text1.chars().nth(i - 1).ok_or(anyhow!("Underflow Error"))?,
                text2.chars().nth(j - 1).ok_or(anyhow!("Underflow Error"))?,
                i - 1,
            ));
            i = i.checked_sub(1).ok_or(anyhow!("Underflow Error"))?;
            j = j.checked_sub(1).ok_or(anyhow!("Underflow Error"))?;
        }
    }

    Ok(changes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_strings() {
        let changes = levenshtein_diff("", "").unwrap();
        assert_eq!(changes.len(), 0);
    }

    #[test]
    fn test_insertion() {
        let changes = levenshtein_diff("abc", "abcd").unwrap();
        assert_eq!(changes.len(), 1);
        assert_eq!(changes[0], Change::Insertion('d', 3));
    }

    #[test]
    fn test_deletion() {
        let changes = levenshtein_diff("abcd", "abc").unwrap();
        assert_eq!(changes.len(), 1);
        assert_eq!(changes[0], Change::Deletion('d', 3));
    }

    #[test]
    fn test_substitution() {
        let changes = levenshtein_diff("abcd", "abed").unwrap();
        assert_eq!(changes.len(), 1);
        assert_eq!(changes[0], Change::Substitution('c', 'e', 2));
    }

    #[test]
    fn test_multiple_changes() {
        let changes = levenshtein_diff("abcd", "abedf").unwrap();
        assert_eq!(changes.len(), 2);
        assert_eq!(changes[0], Change::Insertion('f', 4));
        assert_eq!(changes[1], Change::Substitution('c', 'e', 2));
    }

    #[test]
    fn test_same_string() {
        let changes = levenshtein_diff("abcd", "abcd").unwrap();
        assert_eq!(changes.len(), 0);
    }

    #[test]
    fn test_many_changes() {
        let changes = levenshtein_diff("abcd", "abefgh").unwrap();
        assert_eq!(changes.len(), 4);

        assert_eq!(changes[0], Change::Insertion('h', 5));
        assert_eq!(changes[1], Change::Insertion('g', 4));
        assert_eq!(changes[2], Change::Substitution('d', 'f', 3));
        assert_eq!(changes[3], Change::Substitution('c', 'e', 2));
    }

    #[test]
    fn test_medium_string() {
        let changes = levenshtein_diff(
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed non risus. Suspendisse",
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed non risus. Suspendisse",
        )
        .unwrap();
        assert_eq!(changes.len(), 0);
    }

    // Add more tests
}
