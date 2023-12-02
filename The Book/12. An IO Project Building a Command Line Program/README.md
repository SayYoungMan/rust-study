# 12. An I/O Project: Building a Command Line Program

- We'll build a command line tool that interacts with file and command line input/output to practice some of the Rust concepts.
- We are making a `grep` that searches a specified file for a specified string.
- To do so, `grep` takes arguments a file path and a string, then it reads the file, finds lines that contain the string and prints those lines.
- We'll read environment variables to allow the user to configure the behavior of our tool.
- We'll also print error messages to the standard error console stream instead of standard output so the user can redirect successful output to a file while still seeing error messages onscreen.

## 12.1. Accepting Command Line Arguments

- `std::env::args` will panic if any argument contains invalid Unicode. If you need it to accept invalid Unicode, use `std::env::args_os` instead.

## 12.3. Refactoring to Improve Modularity and Error Handling

- Currently there are four problems with the program:
  1. `main` function performs multiple tasks. It's best to separate functionality so each function is responsible for one task.
  2. The more variables we have in scope, the harder it will be to keep track of purpose of each. It's best to group the configuration variables into one structure to make purpose clear.
  3. Reading a file can fail in a number of ways so we should handle them better.
  4. Not specifying enough arguments would cause `index out of bounds` error

### Separation of Concerns for Binary Projects

- The guidelines for splitting the separate concerns of a binary program is:

  - Split your program into `main.rs` and `lib.rs` and move program's logic to `lib.rs`.
  - As long as your command line parsing logic is small, it can remain in `main.rs`.
  - When the command line parsing logic starts getting complicated, extract it from `main.rs` and move it to `lib.rs`

- The responsibilities that remain in the `main` function should be limited to:
  - Calling the command line parsing logic with the argument values
  - Setting up any other configuration
  - Calling a `run` function in `lib.rs`
  - Handling the error if `run` returns an error

### The Trade-Offs of Using clone

- There is tendency to avoid using `clone` to fix ownership problems due to its runtime cost.

## 12.4. Developing the Library’s Functionality with Test-Driven Development

- Test-driven development (TDD) process can be done by following steps:
  1. Write a test that fails and run it to make sure it fails for the reason you expect.
  2. Write or modify just enough code to make the new test pass.
  3. Refactor the code you just added or changed and make sure the tests continue to pass.
  4. Repeat from step 1!

## 12.6. Writing Error Messages to Standard Error Instead of Standard Output

- In most terminals, there are two kinds of output:
  - Standard output (`stdout`) for general information
  - Standard error (`stderr`) for error messages
- This distinction enables users to choose to direct the successful output of a program to a file but still print error messages to the screen.
