# 2. Programming a Guessing Game

### Processing a Guess

```rust
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
```

- To obtain user input, we need `io` input/output library, which comes from the `std` standard library:
  ```rust
  use std::io;
  ```
- Rust has set of items defined in the standard library. This set is called the `prelude`.

> Even if we don't import io library, we could still use the function by calling `std::io::stdin`.

#### Storing Values with Variables

- We use `let` statement to create a variable
- In Rust, variables are immutable by default. To make a variable mutable, we add `mut` before the variable name.
  ```rust
  let apples = 5; // immutable
  let mut bananas = 5; // mutable
  ```
- Calling `String::new` function returns new instance of a `String`, a string type provided by standard library that is a growable, UTF-8 encoded text.
- `::` syntax indicates that `new` is an `associated function` of the `String` type, which means it's a function that's implemented on a type.

#### Receiving User Input

- `.read_line(&mut guess)` calls the `read_line` method on the standard input handle.
- Passing `&mut guess` to tell what string to store the user input in.
- `&` indicates that this argument is a `reference`, which gives you a way to let other parts of code access one data without needing to copy that data into memory.

#### Handling Potential Failure with Result

- `read_line` returns a `Result` enum value, which is a type that can be in one of multiple possible states (`variants`).
- `Result` variants are `Ok` (indicate successful operation) and `Err` (operation failed).
  - If Result is `Err`, the program will crash and display the message you passed as an argument to `expect`.
  - If Result is `Ok`, `expect` will take the return value that `Ok` is holding and return that value. In this case, that value is the number of bytes in the user's input.

#### Printing Values with println! Placeholders

- When printing the value of a variable, the variable name can go inside curly brackets.
- When printing the result of evaluating an expression, place empty curly brackets in the format string, then follow the string with a comma-separated list of expressions.

```rust
let x = 5;
let y = 10;

println!("x = {x} and y + 2 = {}", y + 2);
```

### Generating a Secret Number

```rust
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
```

#### Using a Crate to Get More Functionality

- `Crate` is a collection of Rust source code files.
  - `Binary crate` is an executable that we build.
  - `Library crate` contains code that is intended to be used in other programs.
- Adding crate beneath `[dependencies]` section in `Cargo.toml` file, specifies which external crates to be used in the project.
- Semantic version specifier `0.8.5` is short for `^0.8.5`, which is at least 0.8.5 but below 0.9.0.
- `Crates.io` is where people post their open source Rust projects for others to use.

#### Ensuring Reproducible Builds with the Cargo.lock File

- Cargo will use only the versions of dependencies you specified unless specified, to ensure you can rebuild the same artifact every time.
- This is done by writing all versions to `Cargo.lock` file when you build the project for the first time.

#### Updating a Crate to Get a New Version

- `cargo update` ignores the Cargo.lock file and figure out all the latest versions that fits the specification.

### Generating a Random Number

- `Rng` trait defines methods that random number generators implement, and it must be in scope to use those methods.
- `rand::thread_rng` functions gives RNG that is local to the current thread of execution and is seeded by the OS.

> `cargo doc --open` will build documentation provided by the dependencies and open it in the browser.

### Comparing the Guess to the Secret Number

```rust
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // --snip--

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
```

- `cmp` method compares two values and returns a variant of the `Ordering` enum.
- We use `match` expression to decide what to do based on variant of `Ordering`.
  - `match` expression is made up of `arms`, which consists of a `pattern` to match against and the code that should be run if it fits arm's pattern.
  - It finds the first arm's pattern that matches.
- It will complain because the type of `guess` which is String cannot be compared with `secret_number` which is i32. (Rust's default number type) Therefore, convert String to number by:

```rust
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

- `Shadowing` lets us reuse the `guess` variable name.
- `trim` method must be used as during `read_line`, it appends new line `\n` when user presses enter to confirm guess.
- `:` after variable tells Rust about the variable's type. Additionally, annotating as `u32` and comparing with `secret_number` tells Rust that it should be a `u32` as well.

### Allowing Multiple Guesses with Looping

```rust
// --snip--

println!("The secret number is: {secret_number}");

loop {
    println!("Please input your guess.");

    // --snip--

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
```

- `loop` keyword creates an infinite loop.

#### Quitting After a Correct Guess

```rust
match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => {
        println!("You win!");
        break;
    }
}
```

- Adding `break` line, when user guesses the number correctly will exit the loop.

#### Handling Invalid Input

```rust
// --snip--

io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");

let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};

println!("You guessed: {guess}");

// --snip--
```

- Instead of crashing on error, we use match expression.
  - On `Ok`, the match expression will just return the num value that `parse` produced and put inside the `Ok` value.
  - On `Err`, we just `continue`, which tells the program to go to next iteration of `loop`.
  - `_` is a catchall value saying we want to match all `Err` values.
