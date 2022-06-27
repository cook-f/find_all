//! `find_all` is capable of finding all indexes of elements where a predicate is met and therefore aims to be a simple alternative with  (nearly) identical interface to the `find` method (this difference being returning an `Option<Vec<usize>>` instead of `Option<usize>`)
//!
//!
//! ```rust
//! let test_data = [1, 2, 3, 4, 1, 1, 1, 1];
//! let indexes = find_all::find_all(test_data.iter(), |num: &i32| *num == 9);
//! assert_eq!(indexes, None);
//!
//! let indexes = find_all::find_all(test_data.iter(), |num: &i32| *num == 1);
//! assert_eq!(indexes, Some(vec![0,4,5,6,7]));
//! ```

/// ```rust
/// let test_data = vec![1, 2, 3, 4, 1, 1, 1, 1];
/// let indexes = find_all::find_all(test_data.iter(), |num: &i32| *num == 1);
/// assert_eq!(indexes, Some(Vec::from([0, 4, 5, 6, 7])));
///
/// let test_data = vec![1, 2, 3, 4, 1, 1, 1, 1];
/// let indexes = find_all::find_all(test_data.iter(), |num: &i32| *num == 9);
/// assert_eq!(indexes, None);
///
/// let test_data = vec![
///     String::from("hello"),
///     String::from("goodbye"),
///     String::from("hello"),
/// ];
/// let indexes = find_all::find_all(test_data.iter(), |string| string.contains('o'));
/// assert_eq!(indexes, Some(vec![0, 1, 2]));
/// ```
pub fn find_all<I, P>(iter: I, mut predicate: P) -> Option<Vec<usize>>
where
    I: Iterator,
    P: FnMut(I::Item) -> bool,
{
    let mut occurences = Vec::<usize>::default();
    for (index, element) in iter.enumerate() {
        if predicate(element) {
            occurences.push(index);
        }
    }

    match occurences.len() {
        0 => None,
        _ => Some(occurences),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
