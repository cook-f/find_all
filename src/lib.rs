//! `find_all` is capable of finding all indexes of elements where a given predicate is met and therefore aims to be a simple alternative with  (nearly) identical interface to the `find` method (this difference being returning an `Option<Vec<usize>>` instead of `Option<usize>`)
//!
//!
//! ```rust
//! use find_all::FindAll;
//! let test_data = [1, 2, 3, 4, 1, 1, 1, 1];
//! let indexes = test_data.iter().find_all(|num: &&i32| **num == 9);
//! assert_eq!(indexes, None);
//!
//! let indexes = test_data.iter().find_all(|num: &&i32| **num == 1);
//! assert_eq!(indexes, Some(vec![0,4,5,6,7]));
//! ```

/// ```rust
/// use find_all::FindAll;
///let test_data = vec![1, 2, 3, 4, 1, 1, 1, 1];
///
///let indexes = test_data.iter().find_all(|num: &&i32| **num == 1);
///assert_eq!(indexes, Some(Vec::from([0, 4, 5, 6, 7])));
///
///let test_data = vec![1, 2, 3, 4, 1, 1, 1, 1];
///let indexes = test_data.iter().find_all(|num| **num == 9);
///assert_eq!(indexes, None);
///
///let test_data = vec![
///    String::from("hello"),
///    String::from("goodbye"),
///    String::from("hello"),
///];
///let indexes = test_data.iter().find_all(|string| string.contains('o'));
///assert_eq!(indexes, Some(vec![0, 1, 2]));
/// ```

pub trait FindAll: Iterator + Sized {
    fn find_all<P>(&mut self, predicate: P) -> Option<Vec<usize>>
    where
        P: FnMut(&Self::Item) -> bool;
}

impl<I> FindAll for I
where
    I: Iterator,
{
    fn find_all<P>(&mut self, mut predicate: P) -> Option<Vec<usize>>
    where
        P: FnMut(&Self::Item) -> bool,
    {
        let mut occurences = Vec::<usize>::default();
        for (index, element) in self.enumerate() {
            if predicate(&element) {
                occurences.push(index);
            }
        }

        match occurences.len() {
            0 => None,
            _ => Some(occurences),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_data = vec![1, 2, 3, 1, 1, 1];
        let occurences = test_data.iter().find_all(|element| **element == 1);

        assert_eq!(occurences, Some(vec![0, 3, 4, 5]))
    }
}
