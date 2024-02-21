pub mod data_structures;
pub mod arrays_and_hashing;
pub mod stack;
pub mod two_pointers;
pub mod linked_lists;


pub fn sorted<T>(elements: Vec<T>) -> Vec<T>
    where T: Ord + Clone
{
    let mut elements = elements.clone();
    elements.sort();
    elements
}