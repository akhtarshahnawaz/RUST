
# RUST: Quick reference for programmers

## Common Collections

##### Strings in Rust comes in two varities: `String` and `&str`.
* A `String` are dynamic, growable, and allocated on heap. They are stored as a vector of bytes `(Vec<u8>)`, but guaranteed to always be a valid UTF-8 sequence.
* String literals `&str` are stored directly in binary, fixed in size, and stored on stack. `&str` is a slice `(&[u8])` that always points to a valid UTF-8 sequence, and can be used to view into a String.

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
```rust
use std::collections::HashMap;

let mut contacts = HashMap::new();
contacts.insert("Daniel", "798-1364");
contacts.insert("Ashley", "645-7689");
```

## Structs
* Structs are just like dictionaries
* **Tuple Structs** don't have keys, and are used to group variables as logic units. Elements of tuple structs are accessed using `tuple_struct.index`  syntax.
* Implementing `Display` trait on struct can help to print structs directly. Otherwise you would need to use `{:?}` format string and derive the debug macro using `#[derive(Debug)]`
* Structs can have **associated functions** which unlike methods aren't tied to specific instance of the struct. To define associated function, you skip passing the `self` argument.
 ```rust
 // A tuple struct
struct Pair(i32, f32);

// Instantiate a tuple struct
let pair = Pair(1, 0.1);


// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Instantiate a `Point`
let point: Point = Point { x: 10.3, y: 0.4 };
```
## Ownership
