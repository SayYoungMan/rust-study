# 13. Functional Language Features: Iterators and Closures

## 13.1. Closures: Anonymous Functions that Capture their Environment

- Rust's closures are anonymous functions you can save in a variable or pass as arguments to other functions.
- Unlike functions, closures can capture values from the scope in which they're defined.

### Capturing the environment with Closures

- We can pass a closure to `unwrap_or_else` method so that it knows what to do when it's called on `None` variant of `Option`.
- The closure captures an immutable reference to the `self` struct instance and passes it with the code we specify to the `upwrap_or_else` method.

### Closure Type Inference and Annotation

- Closures don't usually require you to annotate the types of the parameters or return value like functions do.
- This is because closures aren't used in an exposed interface as they are stored in variables and used without naming them and exposing them to users of library.
- The following are all valid closure syntaxes except for the first original function:
  ```rust
  fn  add_one_v1   (x: u32) -> u32 { x + 1 }
  let add_one_v2 = |x: u32| -> u32 { x + 1 };
  let add_one_v3 = |x|             { x + 1 };
  let add_one_v4 = |x|               x + 1  ;
  ```
- For closure definitions, the compiler will infer one concrete type for each of their parameters and for their return value. Therefore, you can't reuse same closure with different type values.

### Capturing References or Moving Ownership

- Closures can capture values from their environments in three ways: borrowing immutably, borrowing mutably and taking ownership.
- If you want to force the closure to tke ownership of the values it uses in the environment even though the body of the closure doesn't strictly need ownership, you can use the `move` keyword before the parameter list.
- This technique is useful when passing a closure to a new thread to move the data so that it's owned by the new thread.

### Moving Captured Values Out of Closures and the Fn Traits

- A closure body can do any of the following: move a captured value out of the closure, mutate the captured value, neither move nor mutate the value, or capture nothing from the environment.
- Closures will automatically implement one, two or all of these `Fn` traits, in an additive fashion, depending on how the closure's body handles the values:
  1. `FnOnce` applies to closures that can be called once. All closures implement at least this trait. A closure that moves captured values out of its body will only implement this.
  2. `FnMut` applies to closures that don't move captured values out of their body, but that might mutate the captured values. These closures can be called more than once.
  3. `Fn` applies to closures that don't move captured values out nor don't they mutate captured values. These closures can be called more than once without mutating their environment, which is important if calling a closure multiple times concurrently.

## 13.2. Processing a Series of Items with Iterators

- The iterator pattern allows you to perform some task on a sequence of items in turn.
- An iterator is responsible for the logic of iterating over each item and determining when the sequence has finished.
- In Rust, iterators are lazy, meaning they have no effect until you call methods that consume the iterator to use it up.

### The Iterator Trait and the next Method

- All iterators implement a trait named `Iterator` that looks like this:

  ```rust
  pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
  }
  ```

- Iterator trait requires you to also define an `Item` type that is used in the return type of `next` method.
- When the iteration is over, it returns None but otherwise returns Item wrapped in Some.
- Calling the `next` method on an iterator changes internal state that iterator uses to keep track of where it is in the sequence so this code consumes the iterator so it takes mutable reference to iterator.
- We get immutable references to the values in the vector from iterator. If we want it to take ownership and return owned values, we can call `into_iter`, or if we want to iterate over mutable references we can call `iter_mut`.

### Method that Consume the Iterator

- Methods that call `next` are called `consuming adaptors` because calling them uses up the iterator.
- One example is `sum` method, which takes ownership of iterator and iterates through the items by repeatedly calling next.

### Methods that Produce Other Iterators

- `Iterator adaptors` are methods defined on the `Iterator` trait that don't consume the iterator. Instead, they produce different iterators by changing some aspect of the original iterator.
- One example is method `map`, which takes a closure to call on each item and returns a new iterator that produces the modified items.
- `collect` method consumes the iterator and collects the resulting values into a collection data type.

### Using Closures that Capture Their Environment

- Commonly, the closures specified as arguments to iterator adapters will be closures that capture their environment.

## 13.4. Comparing Performance: Loops vs. Iterators

- The iterators, although a high-level abstraction, get compiled down to roughly the same code as if you'd written the lower-level code yourself.
- `Unrolling` of loop is an optimization that removes the overhead of the loop controlling code and instead generates repetitive code for each iteration of the loop.
