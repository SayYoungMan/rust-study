# 9. Error Handling

- Rust groups errors into two categories:
  - For a `recoverable` error, we most likely want to report the problem and retry the operation.
  - `Unrecoverable` errors are always symptoms of bugs so we want to stop the program.

## 9.1. Unrecoverable Errors with panic!

- In Rust, you can cause the code to panic by doing invalid actions or explicitly call `panic!` macro.
- By default, these panics will print a failure message, unwind, clean up the stack and quit.
- Via an environment variable, you can also display the call stack when a panic occurs.

### Unwinding the Stack or Aborting in Response to a Panic

- By default, when a panic occurs, the program starts `unwinding`, which means Rust walks back up the stack and cleans up the data from each function it encounters.
- However, it is a lot of work. Therefore, you can choose to immediately `abort`, which ends the program without cleaning up.
- Memory that the program was using will then need to be cleaned up by OS.
- If your project need to make resulting binary as small as possible, you can switch to aborting by adding `panic = 'abort'` to appropriate `[profile]` sections in `Cargo.toml` file.

### Using a panic! Backtrace

- In C, attempting to read beyond the end of data structure reads the location in memory that would correspond to that element. This is called `buffer overread` and can lead to security vulnerabilities.
- To protect from this, Rust will stop execution and refuse to continue.
- `RUST_BACKTRACE` environment variable can get a backtrace of what happened to cause the error. You need to set it to any value except 0 to enable.
- A `backtrace` is a list of all the functions that have been called to get to this point.

## 9.2. Recoverable Errors with Result

- `Result` enum is defined as follows:
  ```rust
  enum Result<T, E> {
      Ok(T),
      Err(E),
  }
  ```
- T represents the type of value returned in a success case and E represents the type at failure case.
- `Result` enum conveys whether the function call succeeded or failed and at the same time give us either file handle or error information.
- It's part of the prelude so we don't need to specify `Result::`.

### Matching on Different Errors

- We can add inner match expression inside error arm to behave differently based on `.kind()` of error.
- The method `kind` returns `io::ErrorKind` enum which is provided by the standard library and has variants representing the different kinds of errors.

### Alternatives to Using match with `Result<T, E>`

- Instead of `match`, you can also use methods like `upwrap_or_else` to deal with errors and be more concise.

### Shortcuts for Panic on Error: unwrap and expect

- `unwrap` method is a shortcut method implemented just like the match expression. It will return the value if `Ok` and will call `panic!` if `Err`.
- Similarly, `expect` method lets us choose the `panic!` error message.
- Use `expect` rather than `unwrap` to give more context about why.

### Propagating Errors

- When a function's implementation calls something that might fail, instead of handling the error within the function itself, you can return the error to the calling code so that it can decide what to do. This is known as `propagating` the error.
- We don't have enough information on what the calling code is trying to do on error, so we propagate all the success or error information.

#### A Shortcut for Propagating Errors: the ? Operator

- When `?` is placed after a `Result` and if the value of the `Result` is `Ok`, the value inside `Ok` will get returned from this expression and the program will continue. If value is `Err`, the `Err` will be returned from the whole function.
- Error values that have `?` operator called on them go through the `from` function defined in the `From` trait, which is used to convert values from one type to another.
- When `?` calls the `from` function, the error type received is converted into the error type defined in the return type of the current function.
- It eliminates a lot of boilerplate and makes the function's implementation simpler.

#### Where the ? Operator can be Used

- The `?` operator can only be used in functions whose return type is compatible with the value the `?` is used on.
- This is because the `?` operator is defined to perform an early return of a value of the function.
- The `?` operator is only allowed in a function that returns `Result`, `Option`, or another type that implements `FromResidual`.
- When `?` operator is called on `Option<T>`, if the value is `None`, the `None` will be returned early from the function and if the value is `Some`, the value inside `Some`is the resulting value of the expression.
- `main` can return `Result<(), E>` by specifying `fn main() -> Result<(), Box<dyn Error>>`.
- When `main` function returns `Ok(())`, it will exit with 0 and if it returns `Err` value, it will exit with a nonzero value.
- The `main` function may return any types that implement `std::process:Termination` trait, which contains a function `report` that returns an `ExitCode`.
