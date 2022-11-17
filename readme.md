
# RUST: Quick reference for programmers

## Common Collections
Arrays are assigned on stack, and are immutable

```rust
let a:[i32;_] = [1,2,3];
```


Vectors are assigned on heap, and are mutable

```rust
let v: vec<i32> = vec.new();
```
Elements of a vectors can be accessed using `&v[index]$`{.rust}. However using `v.get()$`{.rust} is safer and prevents conditions like out of bound error.
