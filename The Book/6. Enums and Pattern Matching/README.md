# 6. Enums and Pattern Matching

- `Enums` allow you to define a type by enumerating its possible `variants`.

## 6.1. Defining an Enum

- Enums give you a way of saying a value is one of a possible set of values.

```rust
// There are two types of IP addresses: v4 and v6
enum IpAddrKind {
    V4,
    V6,
}
```

### Enum values

- We can create instances of each variants like this:
- And We can define a function that takes any of two:

```rust
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;

fn route(ip_kind: IpAddrKind) {}
```

- Representing type of address and address value associated with it can be concisely done with enum than struct:

```rust
enum IpAddr {
    V4(String),
    V6(String),
}
let home = IpAddr::V4(String::from("127.0.0.1"));
let loopback = IpAddr::V6(String::from("::1"));
```

- The name of each enum variant that we define also becomes a function that constructs an instance of the enum.
- Each variant can have different types and amounts of associated data too.
- We are able to define methods on enums the same way as structs too.

### The Option Enum and Its Advantages Over Null Values

- `Option` type encodes the very common scenario in which a value could be something or nothing.
- Rust does not have the null feature like other languages.
- Option is included in prelude and defined by standard library as follows:
  ```rust
  enum Option<T> {
      None,
      Some(T),
  }
  ```
- Variants `None` and `Some` are also included in prelude so no need to use `Option::` prefix.
- Having Option is better than null because `Option<T>` and `T` are different types so the compiler won't let us use `Option<T>` value as if it were definitely a valid value. So the compiler will make sure to handle the null case before using its value.
- In order to use `Option<T>`, you want to have code that will handle each variant.
