use crate::changes::Change;
use anyhow::anyhow;

/// Calculates the Levenshtein distance between text1 and text2.
/// The function takes two string slices as arguments, text1 and text2, and returns
/// a vector of Change enum variants representing the differences between the two texts.
/// # Panics
/// The function panics if the difference between the lengths of text1 and text2 is greater than usize::MAX.
///
/// # Errors
/// The function returns an error if the difference between the lengths of
/// text1 and text2 is greater than usize::MAX.
///
/// # Safety
/// The function is safe.
/// # Performance
///
/// The function has a time complexity of O(nm) and a space complexity of O(nm).
///
/// # See also
///
/// [Wikipedia](https://en.wikipedia.org/wiki/Levenshtein_distance) | [Rosetta Code](https://rosettacode.org/wiki/Levenshtein_distance#Rust) | [Levenshtein Distance](https://www.youtube.com/watch?v=MiqoA-yF-0M) | [Levenshtein Distance](https://www.youtube.com/watch?v=We3YDTzNXEk)
///
pub fn levenshtein_diff(text1: &str, text2: &str) -> anyhow::Result<Vec<Change>> {
    // The vector of changes is initialized.
    let mut changes = Vec::new();
    // The matrix is initialized with the size of the two strings plus one.
    let mut matrix = vec![vec![0; text2.len() + 1]; text1.len() + 1];

    // The first row and column of the matrix are initialized with the index of the character in the string.
    matrix
        .iter_mut()
        .take(text1.len() + 1)
        .enumerate()
        .for_each(|(i, row)| {
            row[0] = i;
        });

    matrix[0]
        .iter_mut()
        .take(text2.len() + 1)
        .enumerate()
        .for_each(|(j, col)| {
            *col = j;
        });

    // The matrix is filled with the minimum number of changes required to transform text1 into text2.
    // The algorithm is based on the following recurrence relation:
    // matrix[i][j] = min(matrix[i - 1][j] + 1, matrix[i][j - 1] + 1, matrix[i - 1][j - 1] + 1) if text1[i] != text2[j]
    // matrix[i][j] = matrix[i - 1][j - 1] if text1[i] == text2[j]

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

    // The changes are calculated by traversing the matrix from the bottom right corner to the top left corner.
    // If the current cell is equal to the cell above it plus one, then a deletion has occurred.
    // If the current cell is equal to the cell to the left of it plus one, then an insertion has occurred.
    // If the current cell is equal to the cell to the top left of it plus one, then a substitution has occurred.
    // If the current cell is equal to the cell to the top left of it, then no change has occurred.
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
