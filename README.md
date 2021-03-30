cartesian-rs
============

A macro for combining iterators to the cartesian product.
Using the macro it is possible to write code compacter, and use less indention space.

Writing one for-loop and iterating through the cartesian product of multiple iterators
is similar to nesting for loops, in each loop iterating through one of the iterators.
However, the semantics of `break` is changed.

## Example

```rust
let (m, n, p) = (3, 3, 1);

let mut id = vec![vec![0; n]; m];
for (i, j) in cartesian!(0..m, 0..n) {
    id[i][j] = (i == j) as u32;
}

let col_vec = vec![vec![1], vec![2], vec![4]];

let mut res = vec![vec![0; p]; m];

for (i, j, k) in cartesian!(0..m, 0..n, 0..p) {
    res[i][k] += id[i][j] * col_vec[j][k];
}

assert_eq!(col_vec, res);
```

## License

This package is licensed under the [MIT license](LICENSE).
