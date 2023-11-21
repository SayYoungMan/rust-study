# 4. Understanding Ownership

## 4.1. What Is Ownership?

- `Ownership` is a set of rules that govern how Rust program manages memory.
- Some languages have garbage collection and in others, programmer must explicitly allocate and free memory.
- In Rust, memory is managed through a system of ownership with a set of rules that the compiler checks.

### The Stack and the Heap

- In systems programming languages like Rust, whether a value is on the stack or heap affects how the language behaves.
- Both stack and heap are parts of memory available to the code at runtime but structured differently:
  - Stack stores value in the order it gets them and removes values in the opposite order. All data stored on the stack must have a known, fixed size.
  - Heap is less organized: when you put data on the heap, you request a certain amount of space. The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer.
- Pushing to stack is faster than allocating on heap because the allocator never has to search for a place to store new data but just store at the top of the stack.
- Allocating space on the heap requires more work because the allocator need to find big enough space first and perform bookkeeping to prepare for next allocation.
- Accessing data in heap is slower than on stack because you have follow a pointer to get there.
- When a function is called, the values passed into the function and its local variables get pushed onto the stack. When it's over, those values get popped off the stack.
- Ownership addresses problems like keeping what parts of code are using what data on heap, minimizing the amount of duplicate data on heap and cleaning up unused data on the heap.

### Ownership Rules

- Each value in Rust has an `owner`.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

### Variable Scope

- A scope is the range within a program for which an item is valid

```rust
{                       // s is not valid, as not declared yet
    let s = "hello";    // s is valid from here
    // Can do stuff with s
}                       // s is no longer valid again because scope is over
```

### The String Type

- The types covered in Ch. 3 are of known size so can be stored on the stack and can be quickly and trivially make a new instance.
- String is a complex data type that is stored on the heap. It differs from string literals that are immutable.

### Memory and Allocation

- With String type, in order to support mutability, we need to allocate a memory unknown at compile time. This means:
  - The memory must be requested from the memory allocator at runtime.
  - Need a way of returning this memory to the allocator when we are done with String.
- When a variable, goes out of scope, Rust calls a `drop` function at the closing curly bracket to return the memory to the allocator.

### Variables and Data Interacting with Move

```rust
let s1 = String::from("hello");
let s2 = s1;
```

- `s1` will store pointer, length (how many bytes String is using) and capacity (how many bytes that String received from allocator) in a stack and pointer points to the heap that holds the content `hello`.
- When we assign `s1` to `s2`, the pointer, length and capacity are copied but not the data on the heap.
- Now when `s2` and `s1` go out of scope, they will both try to free the same memory. This is known as `double free error`. It can cause memory corruption, leading to security vulnerabilities.
- In Rust, to ensure memory safety, `s1` is considered no longer valid after `let s2 = s1`. This is known as a `move`.

### Variables and Data Interacting with Clone

- If we want to deep copy String, we can use common method called `clone`.
- This is an expensive operation.

### Stack-Only Data: Copy

```rust
let x = 5;
let y = x;
println!("x = {}, y = {}", x, y);
```

- This is valid because types like integers have known size at compile time and are stored entirely on stack. Therefore, it's quick to copy an actual value and no reason to stop this.
- Rust has a special annotation called `Copy` trait that can be placed on types stored on the stack. A type can't be annotated with `Copy` if the type or any of its parts implemented `Drop` trait.
- Group of scalar values can implement `Copy` and nothing that requires allocation or is some form of resource can implement `Copy`.

### Ownership and Functions

- Passing a variable to a function will move or copy, just like assignment.

```rust
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
```

### Return Values and Scope

- Returning values can also transfer ownership.
- Assigning a value to another variable moves it.
- When a variable that includes data on the heap goes out of scope, the value will be dropped unless ownership of the data has been moved to another variable.

## 4.2. References and Borrowing

- A `reference` is like a pointer in that it's an address we can follow to access the data stored at that address.
- Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.

```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here s goes out of scope. But because it does not have ownership of what it refers to, it's not dropped.
```

- In `&String`, the ampersands represent references, allowing you to refer to the value without taking ownership of it.
- Opposite of `&` is `*` and it called `dereference` operator.
- Action of creating a reference is called `borrowing`.
- We cannot modify a variable that is borrowed as the references are immutable by default.

### Mutable References

- Mutable reference can be created by `&mut` when the variable it refers to is mutable.
- Restriction: if you have a mutable reference to a value, you can have no other references to that value.
- It prevents data races at a compile time. A `data race` is similar to a race condition and happens when these behaviors occur:
  - Two or more pointers access the same data at the same time.
  - At least one of the pointers is being used to write to the data.
  - There is no mechanism being used to synchronize access to the data.
- Similarly, mutable and immutable references cannot be used together if the scopes of two are overlapping with each other.

### Dangling References

- In languages with pointers, it's easy to create a `dangling pointer - a pointer that references a location in memory that may have been given to someone else - by freeing some memory while preserving a pointer to that memory.
- In Rust, the compile guarantees, there will never be dangling references: if you have a reference to data, the compiler will ensure that the data will not go out of scope before the reference to the data does.
