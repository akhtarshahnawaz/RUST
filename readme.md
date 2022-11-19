
# RUST: Quick reference for programmers
## Ownership
Some Basics:
* There are mainly 3 types of memory management models: Manual, Ownership Model, Garbage Collection
* Stack is fixed size, can't grow or shrink, holds also stack frames (functions that hold local variables whose size are computed at compile time). 
* Variables live as long as stack frame lives.
* Heaps are dynamic in size, can grow and shrink, and we can control lifetime of data.
* Pushing to stack is faster than allocating to heap because system needs to find and allocate empty space where to store on heap.

There are three rules for memory management in Rust:
* Each value has a variable called its owner
* There can only be one owner at a time
* When owner goes out of scope, value is dropped
Scopes can be prescibed also using curly braces `{}`

There are few things to remember to keep track of ownership:
* Allocating a stack variable to another variable `x = y` will copy the variable
* Allocating a heap variable to another variable `x = y` will move the variable i.e. y is invalidated & moved to x
* To copy variables on residing on heap, we can use clone as ` x = y.clone()`
* Passing a heap variable to a function will move it as local variable inside function, and actual variable will be invalidated. To avoid that, you can apss a reference to the variable using `&variable`.  Reference will point to actual variable that will point to data.
* References are immutable  by default, so we cannot modify the value inside the function. To modify the value, you need to make the variable mutable, then pass a mutable reference using `&mut`
* You can only have one mutable reference to a piece of data in a praticular scope (to prevent data races).
* You cannot have mutable reference if immutable reference already exists.
* Remember that references must always be valid, and at any given time, you can either have one mutable reference or any number of immutable references.
* Scope of a variable starts when it is introduced for first time, and ends when it is used for last time. Therefore, its okay if mutable reference is used after scope of immutable reference ends.

Few things about Slices:
* Can be used on strings or arrays
* They refer to a subset of contiguous data `&s[0..5]`
* Slices don't take ownership
* Slice of a  `String` is a sting literal `&str`
##  Lifetimes

Usually Rust uses borrow checkers at compile time to check there are no dangling pointers but in complex cases we need to assist Rust by providing generic lifetime annotations, for example
* `&i32` is a reference
* `&'a i32` is a reference with lifetime a
* `&'a mut i32` mutable reference with lifetime a
* `fq funtion_name <'a>` function with lifetime a

```rust
// The lifetime of return value is min(lifetime of a, lifetime of b)
fn function_name <'a> (x:&'a str, y: &'a str)->&'a str{
...
}
```

Some things to keep in mind while dealing with lifetimes of a function output
* Each parameter that is a reference gets its own lifetime
* If there is only one input lifetime parameter, that lifetime is assigned to all output lifetime parameters
* If there are multiple input lifetime parameters, but one of them is `&self` or `&mut self`, lifetime of self is assigned to all output lifetime params

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
## ENUMs, Option ENUMs, and `.unwrap()`
 ```rust
 // ENUM is enumerated type, and can be defined as
enum IpAddrKind{
V4, V6
}
let four:IpAddrKind = IpAddrKind::V4

// ENUM can also store data like this
enum IpAddrKind{
V4(String), V6(String)
}
let localhost:IpAddrKind = IpAddrKind::V4(String::from("192.168.1.1"))
```

* **Option ENUM:** Rust doesn't have  `Null` types, so we use `Option` enum to handle values which could be empty

## Pattern Matching and Error Handling

## Generic Types
## OOPS, Traits and Impl
## Advance Traits and Types
## Trait Objects
## Smart Pointers, Box Smart Pointer (Deref and Drop Traits)
## Smart Pointers (RC, ARC, Interior Mutability, Reference Cycles)
## Closures & Iterators (Advance Functions and Closures)
## Module System
## Concurrency
## Macros (Declarative & Procedural)

