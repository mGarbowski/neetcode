use std::collections::HashMap;
use std::hash::Hash;

pub mod valid_sudoku;
pub mod two_sum;
pub mod is_anagram;
pub mod product_except_self;
pub mod group_anagrams;
pub mod top_k_frequent;
pub mod contains_duplicate;

fn count_elements<T>(elements: Vec<T>) -> HashMap<T, usize>
    where T: Eq + Hash
{
    let mut counts = HashMap::new();
    for element in elements {
        *counts.entry(element).or_default() += 1
    }
    counts
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::arrays_and_hashing::count_elements;

    #[test]
    fn count_elements_multiple() {
        assert_eq!(HashMap::from([(1, 3), (2, 2), (3, 1)]), count_elements(vec![1, 1, 1, 2, 2, 3]))
    }
}