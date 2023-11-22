# 5. Using Structs to Structure Related Data

- A `struct`, or structure, is a custom data type that lets you package together and name multiple related values that make up a meaningful group.

## 5.1. Defining and Instantiating Structs

- Like tuples, the pieces of a struct can be different types but unlike tuples, in a struct you have to name each piece of data.
- More flexible than tuples because you don't have to rely on order of data.
- Structs are defined like this:
  ```rust
  struct User {
      active: bool,
      name: String,
      email: String,
      sign_in_count: u64,
  }
  ```
- And instantiated like this:
  ```rust
  let user1 = User {
      active: true,
      name: String::from("name123"),
      email: String::from("some@example.com"),
      sign_in_count: 1,
  }
  ```
- To get specific value from a struct, use dot notation: `user1.email`.
- If instance is mutable, we can change a value by assigning into a particular field.
- Entire instance must be mutable, not just certain fields.

### Using the Field Init Shorthand

- If the parameter name and the struct field names are same, we can use `field init shorthand` syntax.
  ```rust
  fn build_user(name: String, email: String) -> User {
      User {
          active: true,
          name,
          email,
          sign_in_count: 1,
      }
  }
  ```

### Creating Instances from Other Instances with Struct Update Syntax

- To conveniently create a new instance that includes most of values from other instance, but changes some, you can use `struct update syntax`.

```rust
let user2 = User {
    name: String::from("anotherName"),
    ..user1
}
```

- `..user1` must come last to specify that any remaining fields should get their values from the corresponding fields in `user1`.
- The struct update syntax uses `=` like an assignment; this is because it moves the data. Therefore, we can no longer use `user1` as a whole because email field of `user1` was moved into `user2`.

### Using Tuple Structs Without Named Fields to Create Different Types

- `Tuple structs` have the added meaning the struct name provides but don't have names associated with their fields; rather, they just have the types of the fields.
- Useful when you want to give tuple a name and make it different type from other tuples, and when naming each field would be verbose or redundant.

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0)
    let origin = Point(0, 0, 0)
}
```

- Each struct defined is its own type, even though the fields within the struct have the same types.

### Unit-Like Structs Without Any Fields

- `Unit-like structs` don't have any fields and behave similarly to `()`.
- Can be useful when you need to implement a trait on some type but don't have any data that you want to store in the type itself.

```rust
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```

### Ownership of Struct Data

- It's possible for structs to store references to data owned by something else, but it requires the use of `lifetimes`.
- Lifetimes ensure that the data referenced by a struct is valid for as long as the struct is.

## 5.2. An Example Program Using Structs

- Accessing fields of borrowed struct instance does not move the field values, which is why you often see borrows of structs.
- You can't directly `println!` instance of `Rectangle` struct because it doesn't use formatting known as `Display`.
- Putting the specifier `:?` inside the curly brackets tells `println!` we want to use an output format called `Debug`.
- To use it, we have to add the outer attribute `#[derive(Debug)]` before the struct definition.
- `:#?` will also style the output.
- Another way to print a value using `Debug` format is to use `dbg!` macro, which takes ownership of an expression, prints the file and line number of where the call occurs along with the resultant value, and returns the ownership.
- Rust has a number of traits to use with the `derive` attribute that can add useful behavior to our custom types.

## 5.3. Method Syntax

- `Methods` are similar to functions but they are defined in within the context of a struct(or an enum or a trait object), and their first parameter is always `self`.
- The `&self` is short for `self: &Self`. Within `impl` block, the type `Self` is an alias for the type that the `impl` block is for.
- We still use `&` to indicate that the method borrows the instance. Methods can take ownership of self, borrow self immutably or borrow self mutably.
- Having a method that takes ownership of the instance is usually used when the method transforms self into something else and you want to prevent the caller from using the original instance after the transformation.
- Main reason for using methods, is for organization. We put all things that we can do with an instance in one `impl` block.
- Methods that have the same name as a field and only return the value in the field is called `getters`. They are useful to make a field private but the method public, thus enabling read-only access to that field.

### Where's the -> Operator?

- In C and C++, two operators are used for calling methods: `.` if you're calling a method on the object directly and `->` if you're calling the method on ta pointer to the object and need to dereference the pointer first.
- Rust has automatic referencing and dereferencing so this is not needed.
- Given the receiver and name of a method, Rust can figure out whether method is reading(`&self`), mutating(`&mut self`), or consuming(`self`).

### Associated Functions

- All functions defined within an `impl` block are called `associated functions` because they are associated with the type.
- We can define associated functions that don't have self as their first parameter. (like `String::from`).
- Associated functions that aren't methods are often used for constructors that will return a new instance of the struct. (Often called `new`)

### Multiple impl Blocks

- Each struct is allowed to have multiple `impl` blocks.
- But there is no reason to separate methods into multiple `impl` blocks.
