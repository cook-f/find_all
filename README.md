# find_all

`find_all` is capable of finding all indexes of elements where a given predicate is met and therefore aims to be a simple alternative with  (nearly) identical interface to the `find` method (this difference being returning an `Option<Vec<usize>>` instead of `Option<usize>`)


```rust
use find_all::FindAll;
let test_data = [1, 2, 3, 4, 1, 1, 1, 1];
let indexes = test_data.iter().find_all(|num: &&i32| **num == 9);
assert_eq!(indexes, None);

let indexes = test_data.iter().find_all(|num: &&i32| **num == 1);
assert_eq!(indexes, Some(vec![0,4,5,6,7]));
```

License: GPL-3.0-only
