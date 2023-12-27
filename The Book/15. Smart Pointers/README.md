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

## 15.3 Running Code on Cleanup with the Drop Trait

- `Drop` lets you customize what happens when a value is about to go out of scope.
- You can provide an implementation for the Drop trait on any type, and that code can be used to release resources like files or network connections.
- Functionality of the `Drop` trait is almost always used when implmenting a smart pointer. e.g. when a `Box<T>` is dropped it will deallocate the space on the heap that the box points to.
- Variables are dropped in the reverse order of their creation in the same scope.

### Dropping a Value Early with `std::mem::drop`

- Occasionally, you might want to clean up a value early. e.g. when using smart pointers that manage locks: you might want to force the `drop` method that releases the lock so that other code in the same scope can acquire the lock.
- You have to call `std::mem::drop` function provided by the standard library to force a value to be dropped early.
- Rust doesn't allow us to call `drop` explicitly because it's a `destructor` method and this would cause `double free` error because Rust would be trying to clean up the same value twice.

## 15.4. `Rc<T>`, the Reference Counted Smart Pointer

- There are cases when a single value might have multiple owners, like in graph structures, multiple edges might point to the same node.
- You have to enable multiple ownership explicitly by using the Rust type `Rc<T>`, which is an abbreviation of `reference counting`.
- `Rc<T>` keeps track of the number of references to a value to determine whether or not the value is still in use.
- If there are zero references to a value, the value can be cleaned up without any references becoming invalid.
- We use `Rc<T>` when we want to allocate some data on the heap for multiple parts of our program to read and we can't determine at compile time which part will finish using the data last.

### Using `Rc<T>` to Share Data

- You have to call `Rc::clone` when creating new reference to the object so that the number of references increase.
- The implementation of `Rc::clone` doesn't make a deep copy of all the data like `clone` do and the call to `Rc::clone` only increments the reference count. Therefore, it's Rust's convention to use `Rc::clone`.

### Cloning an `Rc<T>` increases the Reference Count

- We can get the counts of references by calling `Rc::strong_count` function.
- Its implementation of `Drop` trait automatically decreases the reference count when an `Rc<T>` value goes out of scope.
- Via immutable references, `Rc<T>` allows you to share data between multiple parts of your program for reading only.

## 15.5. `RefCell<T>` and the Interior Mutability Pattern

- `Interior mutability` is a design pattern in Rust that allows you to mutate data even when there are immutable references to that data.
- To mutate data, the pattern uses `unsafe` code inside a data structure to bend Rust's usual rules that govern mutation and borrowing.
- Unsafe code indicates to the compiler that we're checking the rules manually instead of relying on the compiler.

### Enforcing Borrowing Rules at Runtime with `RefCell<T>`

- The `RefCell<T>` type represents single ownership over the data it holds but differently from `Box<T>`, the borrowing rules' invariants are enforced at runtime. If you break these rules, your program will panic and exit.
- The advantage of checking the borrowing rules at runtime instead is that certain memory-safe scenarios are allowed, where they would've been disallowed by the compile-time checks.

### Interior Mutability: A Mutable Borrow to an Immutable Value

- A consequence of the borrowing rules is that when you have an immutable value, you can't borrow it mutably.
- However, there are situations in which it would be useful for a value to mutate itself in its methods but appear immutable to other code.

#### A Use Case for Interior Mutability: Mock Objects

- Sometimes during testing a programmer will use a type in place of another type, in order to observe particular behavior and assert it's implmented correctly. This placeholder type is called a `test double`.
- `Mock objects` are specific types of test doubles that record what happens during a test so you can assert that the correct actions took place.
- Rust doesn't have mock object functionality built into the standard library but you can create a struct that will serve the same purposes.
- When an actual implementation has a method that doesn't return anything useful to assert on unit tests. You would make a MockImplementation that implements the method in a way that it keeps the value we want to assert on. However, this is difficult when the method does not have mutable reference in the signature.
- This is where `RefCell` can help by setting that saving field as type of `Refcell<T>`.
- We need to call `borrow_mut` method to make changes to the field and `borrow` to get an immutable reference to the vector.

### Keeping Track of Borrows at Runtime with `RefCell<T>`

- The `borrow` method returns the smart pointer type `Ref<T>` and `borrow_mut` returns the smart pointer type `RefMut<T>`.
- Both types implement `Deref` so we can treat them like regular references.
- The `RefCell<T>` keeps track of how many `Ref<T>` and `RefMut<T>` smart pointers are currently active.
- If we try to violate these rules, rather than getting a compiler error, the implementation of `RefCell<T>` will panic at runtime.

### Having Multiple Owners of Mutable Data by Combining `Rc<T>` and `RefCell<T>`

- If you have an `Rc<T>` that holds a `RefCell<T>`, you can get a value that can have multiple owners and that you can mutate.

## 15.6. Reference Cycles Can Leak Memory

- Rust's memory safety guarantees make it difficult, but not impossible, to create memory that is never cleaned up (known as `memory leak`).
- Preventing memory leaks entirely is not one of Rust's guarantees, meaning memory leaks are memory safe in Rust.
- It's possible to create references where items refer to each other in a cycle. This creates memory leaks because the reference count of each item in the cycle will never reach 0.

### Creating a Reference Cycle

```rust
use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());
}
```

- Creating a reference cycle would be a logic bug in your program that you should use automated tests, code reviews and another software development practices to minimize.
- Another solution is reorganizing data structures so that some references express ownership and some references don't,

### Preventing Reference Cycles: Turning an `Rc<T>` into a `Weak<T>`

- You can create a `weak reference` to the value within an `Rc<T>` instance by calling `Rc::downgrade` and passing a reference to the `Rc<T>`.
- Strong references are how you can share ownership of an `Rc<T>` instance.
- Weak references don't express an ownership relationship and their count doesn't affect when an `Rc<T>` instance is cleaned up.
- They won't cause a reference cycle because any cycle involving some weak references will be broken once the strong reference count of values involved is 0.
- When you call `Rc::downgrade`, you get a smart pointer of type `Weak<T>` and `weak_count` keeps track of how many `Weak<T>` references exist.
- Because the value that `Weak<T>` references might have been dropped, to do anything with it, you must make sure the value still exists. This can be done by `upgrade` method, which will return an `Option<Rc<T>>`.

#### Creating a Tree Data Structure: a Node with Child Nodes

```rust
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
}
```

- There is no way to get from leaf to branch. Because leaf has no reference to branch and doesn't know they are related.

#### Adding a Reference from a Child to Its Parent

- If a parent node is dropped, child nodes should be dropped as well. However, a child should not own its parent: if we drop child node, the parent should still exist. This is perfect case for weak references.

```rust
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}
```
