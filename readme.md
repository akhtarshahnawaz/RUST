
# RUST: Quick reference for programmers

## Common Collections
Arrays are assigned on stack, and are immutable

```rust
let a:[i32;_] = [1,2,3];
```


Vectors are assigned on heap, and are mutable. There are 2 methods to create a vector, normal and using macros:

```rust
\\ Normal Syntax
let v: vec<i32> = vec.new();
v.push(1);
v.push(2);
v.push(3);

\\ Syntax using Macro
let v: vec<i32> = vec![1,2,3]
```
Elements of a vectors can be accessed using `&v[index]`. However using `v.get(index)` is safer and prevents conditions like out of bound error.
