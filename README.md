# vec-string
Very short crate to display vectors Vec&lt;T> where T: Display
Normally you can do `format!("{:?}", vec);` when the elements implement `Debug`.
This crate is to do the same but for when the elements implement `Display`.
```rust
assert_eq!("[1, 2, 3]", vec![1, 2, 3].vec_string());
```
