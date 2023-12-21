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

## 15.2. Treating Smart Pointers Like Regular References with the Deref Trait

- Implementing the `Deref` trait allows you to customize the behaviour of the dereference operator (\*).

### Using `Box<T>` Like a Reference

```rust
fn main() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

- You can dereference the value pointed by the box. However, it would be pointing to a copied value of x rather than a reference pointing to the value of x.

### Defining Our own Smart Pointer

```rust
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
```

- This will not work yet because we need to implement deref.

### Treating a Type Like a Reference by Implementing the Deref Trait

```rust
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
```

- `*y` behind the scenes will run `*(y.deref())`.
- The reason `deref` method returns a reference to a value is because if it returned the value directly, the value would be moved out of self.
- We don't want to take ownership of the inner value inside `MyBox<T>` in most cases where we use dereference operator.

### Implicit Deref Coercions with Functions and Methods

- `Deref coercion` converts a reference to a type that implements the `Deref` trait into a reference to another type.
- Deref coercion is a convenience Rust performs on arguments to functions and methods and works only on types that implement the Deref trait.
- It happens automatically when we pass a reference to a particular type's value as an argument to a function or method that doesn't match the parameter type in the function or method definition.
- A sequence of calls to the deref method converts the type we provided into the type the parameter needs.

```rust
fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}
```

- We can pass `&MyBox` to `&str` here because it calls `&String` by deref, and it returns `&str` by deref again.
- The number of timess that deref needs to be inserted is resolved at compile time so there is no runtime penalty.

### How Deref Coercion Interacts with Mutability

- You can use `DerefMut` trait to override the \* operator on mutable references.
- Rust does deref coercion when it finds types and trait implementations in three cases:
  - From &T to &U when `T: Deref<Target=U>`
  - From &mut T to &mut U when `T: Deref<Target=U>`
  - From &mut T to &U when `T: Deref<Target=U>`
- Rust will also coerce a mutable reference to an immutable one but the reverse is not possible.
- This is to guarantee borrowing rules, if you have a mutable reference, that mutable reference must be the only reference to that data.
