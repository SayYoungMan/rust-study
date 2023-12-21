# 15. Smart Pointers

- `References` have no special capabilities other than referring to data and have no overhead.
- `Smart pointers` are data structures that act like a pointer but also have additional metadata and capabilities.
- `Reference counting` smart pointer type enables you to allow data to have multiple owners by keeping track of owners and when no owners remain, clean up the data.
- While references only borrow data, in many cases, smart pointers own the data they point to.
- `String` is an example of smart pointer that own some memory and allow you to manipulate it. Also have capacity as metadata and has extra ability to ensure the data will always be valid UTF-8.
- Smart pointers are usually implemented using structs. Unlike ordinary struct, they implement `Deref` and `Drop` traits.
  - `Deref` trait allows an instance of smart pointer struct to behave like a reference.
  - `Drop` trait allows you to customize the code that's run when an instance of the smart pointer goes out of scope.

## 15.1. Using `Box<T>` to Point to Data on the Heap

- Boxes allow you to store data on the heap rather than the stack. What remains on the stack is the pointer to the heap data.
- Boxes don't have performance overhead, other than storing their data on the heap instead of on the stack.
- Use them most often in these situations:
  - When you have a type whose size can't be known at compile time and you want to use a value of that type in a context that requires an exact size.
  - When you have a large amount of data and you want to transfer ownership but ensure the data won't be copied when you do so.
  - When you want to own a value and you care only that it's a type that implements a particular trait rather than being of a specific type.

### Using a `Box<T>` to Store Data on the Heap

```rust
fn main() {
  let b = Box::new(5);
  println!("b = {}", b);
}
```

- The value 5 will be allocated on the heap.
- Just like any owned value, when a box goes out of scope, box in the stack and the data in the heap will both be deallocated.

### Enabling Recursive Types with Boxes

- Value of `recursive type` can have another value of the same type as part of itself. Recursive types pose an issue because at compile time Rust needs to know how much space it needs. But it can theoretically continue infinitely.
- Because boxes are known size, we can enable recursive types.

#### More Information About the Cons List

- A `cons list` is a data structure that is made up of nested pairs, and is the Lisp version of a linked list.
- Each item in a cons list contains two elements: the value of the current item and the next item.

```rust
enum List {
  Cons(i32, List),
  Nil,
}

fn main() {
  let list = Cons(1, Cons(2, Cons(3, Nil)));
}
```

- By just doing this, it won't work yet because it will complain that the type possibly has infinite size.

#### Computing the Size of a Non-Recursize Type

- To determine how much space to allocate for enum, Rust goes through each of the variants to see which variant needs the most space.

#### Using `Box<T>` to Get a Recursive Type with a Known Size

- Indirection means that instead of storing a value directly, should change the data structure to sotre the value indirectly by storing a pointer to the value instead.
- Therefore, since `Box<T>` has finite size as it's a pointer, we can put box inside Cons variant.

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
```
