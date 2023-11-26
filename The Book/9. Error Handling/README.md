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
