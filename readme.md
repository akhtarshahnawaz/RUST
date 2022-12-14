
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
 ```rust
 // Behind the scene, `Option` ENUM is defined like this
 enum Option <T>{
 Some(T),
 None
 }
 
 // We can use `Option` ENUM like this
 let some_number = Some(5); // Type will be inferred automatically
 let some_string = Some("a string"); // Type will be inferred automatically
 let absent_number: Option<i32> = None; // Type need to be declared in case on None
 
 // If you have an `Option` ENUM, you can call `.unwrap_or()` method with default value to handle `None` case
 // Following statement will return 0 incase the value is `None`
 some_number.unwrap_or(default: 0); 
```
## Pattern Matching and Pattern Matching with Option ENUM
Matching in rust is very similar to `switch-case` statement in other language, with many extra exciting features

```rust
// For a simple 
let number = 13;
match number {
    1 => println!("One!"),
    2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
    13..=19 => println!("A teen"), // Match an inclusive range
    _ => println!("Ain't special") // Default Case
}

// We can use match on ENUMs too
// Running `match` on `ENUM` must be exhaustive, i.e. we must match all cases or use `_` for default cases
let value = match coin{
 coin:Penny =>1,
 coin:Dime=>10,
 _=>0
}

// Match is very frequently used with `Option` ENUM to handle empty case
fn plus_one(x: Option<i32>)->Option<i32>{
 match x{
  None => None,
  Some(i:i32)=> Some(i+1)
 }
}
```

## Error Handling, `Result` ENUM and `?` syntax for unwrap
* `panic!("Crash & Burn")` immediately panics and crashes with the given message
* **Result** ENUM is similar to `Option` ENUM. Just like Option enum returns Some & None, Result enum returns OK or Err
```rust
// Behind the scene, result enum is defined like this
enum Result<T,E>{
 OK(t),Err(E)
}
```
You can either do `match` on returned results, or you can call `.unwrap()`. Unwrapping will return things from inside `OK` or it will panic in case of `Err`
* Adding `?` at the end is a shortcut for `unwrap`
* There is also a `.expect()` method which is a shortcut for `.unwrap_or_panic_with_this_message()` in Rust.

## Generic Types
Generic type allows you to create functions that can use any datatypes as argument.
```rust
// Here we define a function that accept a Vector with element of type T, and return value of type T
// The type T must implement PartialOrd & Copy trait
fn get_largest <T: PartialOrd + Copy>(number_list:Vec<T>)-> T {
 ...
}

// Here we define a struct with two items x & y, such that x is of type T and y is of type U
Struct Point<T,U>{
 x:T,
 y:U
}

// Here we define a enum with two items OK & Err, such that OK is of type T and Err is of type E
enum Result<T,E>{
 x:T,
 y:E
}

// Here we implement a function x for a Struct Point(which contains items with two generic types of U&V) and returns a value of type U
impl <U,V> Point<U,V>{
 fn x(&self) -> &U{
  &self.x
 }
}

```
## OOPS, Traits and Impl
* Traits are very similar to interfaces in other languages. They prescribe a set of functions that must be implemented for some collection data type.
```rust
// Here we define a trait with functions that must be implemented for any Data Type that wants to implement this trait
// Note: We can either have only function signatures or we can also have default function definitions
public trait Summary{
fn Summarize(&self)->String;
}

// Here we implement Summary trait for NewsArticle  
impl Summary for NewsArticle{
 fn Summarize(&self)->String{
  ..
 }
}
```
* Trait implementation and use can get very complicated very fast. Here are some examples:
```rust
// Implement for Pair Struct (which takes a generic type T), where type T already implements Display and PartialOrder traits
impl<T: Display + PartialOrder> Pair<T>{
}

// All the three functions defined below takes two arguments where first argument 't' takes reference to a value of type that implement Display & Clone, and second argument 'u' takes reference to a value of type that implement Clone & Debug, and returns a value of type i32
// Function 1
fn some_function <T: Display+Clone, U: Clone+Debug>(t: &T, u: &U) -> i32{
 ...
}

// Function 2
fn some_function <T,U>(t: &T, u: &U) -> i32 where T: Display+Clone, U:Clone+Debug{
 ...
}

// Function 3
fn some_function (t: &(impl Display+Clone), u: &(impl Clone+Debug)) -> i32 where T: Display+Clone, U:Clone+Debug{
 ...
}

// We can also prescribe return value to be of a type that implements specific traits, for instance we can edit about function to return a value to implements Clone trait
fn some_function (t: &(impl Display+Clone), u: &(impl Clone+Debug)) -> (impl Clone) where T: Display+Clone, U:Clone+Debug{
 ...
}
```

## Advance Traits (Associated Types, Operator Overloading & Super Trait)
* **Associated Types:** are placeholders that you can add to your trait using `type TypeName`, and then methods can use these placeholders. This way, we can define trait for some type which is unknown until implementation. Associated types are different from generics in the sense that generics can have multiple concrete type implementations, whereas associated types have only one. The use of "Associated types" improves the overall readability of code by moving inner types locally into a trait as output types. Syntax for the trait definition is as follows:
```rust
// `A` and `B` are defined in the trait via the `type` keyword. (Note: `type` in this context is different from `type` when used for aliases).
trait Contains {
    type A;
    type B;

    // Updated syntax to refer to these new types generically.
    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
}

// Then when implementing the Trait, we can specify the type
impl Contains for Container {
    // Specify what types `A` and `B` are. If the `input` type  is `Container(i32, i32)`, the `output` types are determined as `i32` and `i32`.
    type A = i32;
    type B = i32;
    ...
 }
 
// Without using associated types
fn difference<A, B, C>(container: &C) -> i32 where
    C: Contains<A, B> { ... }

// Using associated types
fn difference<C: Contains>(container: &C) -> i32 { ... }
```
*  **Operator Overloading:** We can implement `Add` trait `(std::ops::Add)` on some struct to make them addable.
*  Multiple traits can have same method names implemented for same type. In that case you need to call the specific traits explicitly, for example
 ```rust
 // On Human type which implements Wizard trait, call fly() method
 <Human as Wizard>::fly()
 ```

* **Super Traits:** If your trait depends on implementation of some super trait, you need to specify that when defining trait using e.g. `trait TraitName: SuperTraitName`
* To implement a trait on some predefined type (say i32), we need to wrap it in a Struct and then implement for that struct

## Trait Objects
Rust doesn't support classic inheritance, but we can achieve same kind of Polymorphism using Trait Objects. For Example, `draw()` method can be inherited to different shapes in GUI librairy of other language by inheritance, but in Rust, it must be done differently because there is no inheritance.

A trait object can be defined using anything that implements that trait. To define a trait object, we need to wrap it inside some smart pointer like Box, and use `dyn` keyword for dynamic dispactch.

```rust
// Dynamic dipatch must be used in Box Smart Pointer
Box <dyn TraitName>

// Here we define a vector of trait objects i.e. the objects must be implementing Draw Trait
Vec<Box<dyn Draw>>
```

**NOTE:** You might be asking why use trait objects, when you can use generics for same purpose, for example in this case we can pass the type of element that Vec contain. But recognise a crucial difference that a generic can contain only one type of object, however a trait object can be any object that implements that trait i.e. in case of `Draw` trait, it could be a Button, Slider, InputField etc i.e. anything type that implements `Draw` trait 

## Static vs Dynamic Dispatch
* Static dispatch is when the compiler knows the concrete functions that you are calling at compile time.
* Dynamic dispatch is when the compiler doen't knows the concrete functions that you are calling at compile time, and instead figures it out at runtime. We use `dyn` keyword in case of trait objects because compiler doesn't know all the concrete types that will be used at compile time, and it figures it out during runtime. Instead the compiler adds code that figures out correct method to call at runtime, and thus have some performance overload, but gain in flexibility.

**NOTE:** You can only make object safe traits into trait bounds. A trait is `Object safe` when all methods implemented on trait have these 2 properties:
    * Return type isn't self
    * No generic parameters
    
    If a trait doesn't have these two properties, compiler can't figure out concrete types at compile time, adn thus doen't know what methods to call.

## Smart Pointers, Box Smart Pointer (Deref and Drop Traits)
* **Smart Pointers** are pointers with extra capabilities and metadata than a normal pointer. They are usually implemented using structs, and have `deref` and `drop` traits implemented on them. Smart pointers own the data they refer to, unlike references which only refer to them.
    * **Smart Box Pointer:** Allows to store data on heap, and a pointer to the data on the stack. It is very useful for storing data type for which we don't know size at compile time.
* **Deref Trait:** Allows to customize behavior of dereferencing operator i.e. `*i`
* **Drop Train:** Allows you to customize what happens when a value goes out of scope.
    
## Smart Pointers (RC, ARC, Interior Mutability, Reference Cycles)
* **RC Smart Pointer (Reference Count):** Are used to do python like reference count type memory management. Remember that they aren't thread safe. They can be initialized using `Rc::new()`. To increment the reference, we need to use `Rc::clone(&rc_pointer_variable)`. We can know the current reference count, we can use `Rc::strong_count(&rc_pointer_variable)`
* **Ref Cell & Interior Mutability:** When a variable (data structure) is externally immutable, but internally (internal items) mutable using the methods implemented inside to change the value.
* **ARC Smart Pointers: ** are exactly like Rc, but are thread safe.
**NOTE:** Cyclic references can lead to memory leak. It is a logical bug and must be prevented otherwise it will lead to an orphaned memory on heap.

## Closures
* Closures are anonymous functions that can be assigned to variables and passed along to other functions. For example: `let closure = |num|{println!(num)}'
* We can also cache the value of the closure when called for the first time by using structs:
```rust
struct cacher<T> where T:fn(u32)->u32{
calculation: T,
value: Option<u32>
}
```
## Iterators 
* Iterators are used to iterate over iterable types i.e. `.iter()`, `.iter_mut()`, `.into_iter()`,`.next()`,`.iter().map()
* `map()` has `lazy execution`, so we need to call `collect()` on `map()` to get the output.
## Module System
* **Workspace:** At top level, we have workspace, which are group of projects which have many dependencies in common. You need to configure workspace by configuring/adding members to workspace section in `cargo toml`
* The structure inside workspace is something like this:
        `Package`-->`Crates(bin, lib)`-->`Modules`
* Modules are deined using `mod module_name{}`
* Modules can referred usign relative apth or absolute path. The absolute path starts from `crate::`, and everything else is added on top of that.
## Concurrency
* Threads can be spawned using `thread.Spawn(||{})` which returns a type on which you can call `.join().unwrap()`. Note that `||{}` is a closure syntax.
* Channels (to pass data between threads) can be created using MPSC (Multi-Producer Single-Consumer) library, for example `let (tx: , rx: ) = mpsc.channel()`
* When using `Shared State`, you must use mutex to prevent data race.
## Macros (Declarative & Procedural)
Macros in Rust are provided for meta programming i.e. `(Input is Code)' -> `(Output is tranformed Code)`. Some examples of macro in Rust that we use everyday are `println!` and `vec!`
* **Declarative Macros:** Allows to write something similar to a match expression that operates on provided Rust code.
```rust
#[macro_export]
macro_rules! some_name{
 patter_to_match =>{{EMITTED CODE}}
}
```

* **Procedural Macro:** Allows to operate on Abstract Syntax Tree of the Rust code it is given. They must be defined in their own crate with custom crate types. They are of 3 types:
    1. **Custom Derived:** Works also on Functions
    2. **Attribute Like:** Works only on Structs & ENUMs
    3. **Function Like:** 

Here is a very high level syntax:
```macro
use proc_macro;
#[some_attribute]
pub fn some_name(input TokenStream)->TokenStream{
 ...
}
```
