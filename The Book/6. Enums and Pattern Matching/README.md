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

## 6.2. The match Control Flow Construct

- `match` allows you to compare a value against a series of patterns and then execute code based on which pattern matches.
- The power of `match` comes from the expressiveness of the patterns and the fact that the compiler confirms that all possible cases are handled.
- The first value that fits will be used during execution.

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin:: Penny => 1,
        Coin:: Nickel => 5,
        Coin:: Dime => 10,
        Coin:: Quarter => 25,
    }
}
```

- Match arm has two parts: a pattern and an expression. `=>` operator separates the pattern and the code.

### Patterns That Bind to Values

- Match arms can bind to the parts of the values that match the pattern and extract values out of enum variants.

```rust
enum UsState {
    Alabama,
    ...
}
enum Coin {
    ...
    Quarter(UsState),
}
match coin {
    Coin::Quarter(state) => {
        println!("{state}")
    }
}
```

### Matches Are Exhaustive

- Matches in Rust are _exhaustive_: we must exhaust every last possibility in order for the code to be valid.

### Catch-all Patterns and the \_ Placeholder

```rust
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other),
}
```

- Last pattern will match all values not specifically listed.
- It needs to be placed last because the patterns are evaluated in order.
- If you want to catch-all but don't want to use the value in the pattern, `_` is a special pattern that matches any value and does not bind to that value.
- If you want nothing to happen on catch-all case by using `()`.

## 6.3. Concise Control Flow with if let

- `if let` syntax lets you handle values that match one pattern while ignoring the rest

```rust
let config_max = Some(3u8);
match config_max {
    Some(max) => println!("The max is {}", max),
    _ => (),
}
```

- The above code can be refactored using `if let` into:

```rust
let config_max = Some(3u8);
if let Some(max) = config_max {
    println!("The max is {}", max);
}
```

- You can think of `if let` as syntax sugar for a `match` that runs code when the value matches one pattern and ignores all other values.
- We can include `else` with `if let` and it is the same as the block of code that would go with catch-all case.

```rust
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("{state}");
} else {
    count += 1
}
```
