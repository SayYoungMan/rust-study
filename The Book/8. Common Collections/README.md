# 8. Common Collections

- Standard library includes useful data structures called `collections`. They can contain multiple values and they are stored on the heap, meaning that the amount of data doesn't need to be known at compile time.
- `Vector` allows you to store a variable number of values next to each other.
- `String` is a collection of characters.
- `Hash map` allows you to associate a value with a particular key.

## 8.1. String Lists of Values with Vectors

- Vectors can only store values of the same type. They are useful when you have a list of items.

### Creating a New Vector

- Create a new empty vector using `Vec::new` function.
  ```rust
  let v: Vec<i32> = Vec::new();
  ```
- Note that we have to hint the type of its elements using a type annotation. But if you create it with initial values, Rust will infer the type.
- There is also `vec!` macro which will create a new vector:
  ```rust
  let v = vec![1, 2, 3];
  ```

### updating a Vector

- To add elements to a vector, use `push` method: `v.push(5)`;
- To change its value, we have to make the vector mutable too.

### Reading Elements of Vectors

- We can reference a value stored in a vector via indexing (`&v[2]`) or using `get` method (`v.get(2)`).
- When we use `get` method, we get an `Option<&T>` that we can use with `match`.
- There are two ways so you can choose how the program behaves when you try to use an index value outside the range.
  - `[]` method will cause the program to panic. It's best when you want your program to crash.
  - `get` method will return `None` without panicking. You use it if it can happen occasionally under normal circumstances.
- You can't make changes to a vector if you have immutable references to any of its elements.
- This is because vectors put the values next to each other and adding a new element might require allocating new memory and copying the old element to new space. In that case, the reference to first element would point to deallocated memory.

### Iterating over the Values in a Vector

- Use `for` loop to iterate over elements of a vector. Can also iterate over mutable references to each element in a mutable vector to make changes to all elements.
  ```rust
  for i in &mut v {
      *i += 50;
  }
  ```

### Using an Enum to Store Multiple Types

- To store a list of different types, use variants of the same enum type.
  ```rust
  enum SpreadsheetCell {
      Int(i32),
      Float(f64),
      Text(String),
  }
  let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
  ]
  ```
- You must know the exhaustive set of types a program will get at runtime to store in a vector. Otherwise, use trait object instead.

### Dropping a Vector Drops its Elements

- A vector is freed when it goes out of scope. All of its contents are also dropped.
- The borrow checker ensures that any references to contents of a vector are only used while the vector is valid.
