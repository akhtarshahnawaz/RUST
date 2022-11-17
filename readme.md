
# RUST: Quick reference for programmers

## Common Collections

##### Strings in Rust comes in two varities: `String` and `&str`.
* A `String` is stored as a vector of bytes `(Vec<u8>)`, but guaranteed to always be a valid UTF-8 sequence. They is heap allocated, growable and not null terminated.
* `&str` is a slice `(&[u8])` that always points to a valid UTF-8 sequence, and can be used to view into a String, just like `&[T]` is a view into `Vec<T>`.
* 
```rust
// A reference to a string allocated in read only memory
let pangram: &'static str = "the quick brown fox jumps over the lazy dog";

// Heap allocate a string
let alice = String::from("I like dogs");
```


##### Arrays are assigned on stack, and are immutable

```rust
let a:[i32;_] = [1,2,3];
```


##### Vectors are assigned on heap, and are mutable. There are 2 methods to create a vector, normal and using macros:

```rust
// Normal Syntax
let v: vec<i32> = vec.new();
v.push(1);
v.push(2);

// Syntax using Macro
let v: vec<i32> = vec![1,2]
```
Elements of a vectors can be accessed using `&v[index]`. However using `v.get(index)` is safer and prevents conditions like out of bound error. There are 4 commonly used methods on vectors, `.get(index)`, `.remove(index)`, `.push(value)`, `.contains(&value)`
We can iterate and modify our vector like 
```rust
// Normal Syntax
for i in &mut v{
  *i += 50;
}
```
`&mut` here means that it is a mutable reference, and `*i` is a dereference operator.

##### Hashmaps are key:value store, and need to be imported from `std::collections::Hashmap`. Values in a hashmap can be changed similar to vectors by first getting a mutable reference using `&mut` and then changing the value by dereference operator.

