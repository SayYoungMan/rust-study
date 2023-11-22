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
