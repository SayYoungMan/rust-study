# 11. Writing Automated Tests

## 11.1 How to Write Tests

- Tests are Rust functions that verify that the non-test code is functioning in the expected manner. The body of test functions typically: set up needed data or state, run the code you want to test and then assert the results you expect.

### The Anatomy of a Test Function

- To change a function into a test funciton, add `#[test]` on the line before `fn`.
- When you run your tests with the `cargo test` command, Rust builds a test runner binary that runs the annotated functions and reports the test result.
- Whenever we make a new library project with Cargo, a test module with a test function in it is automatically gnerated for us.
- We might also have non-test funcitons in the `tests` module to help set up common scenarios.
- Tests fail when something in the test function panics.
- Each test is run in a new thread, and when the main thread sees that a test thread has died, the test is marked as failed.

### Checking Results with the assert! Macro

- The `assert!` macro is useful when you want to ensure that some condition in a test evaluates to true.
- If the value if false, it calls `panic!` to cause the test to fail.

### Testing Equality with the assert_eq! and assert_ne! Macros

- Standard library provides `assert_eq!` and `assert_ne!` macros to compare two arguments for equality or inequality.
- In Rust, the order in which we specify the value we expect and the value the code produces doesn't matter.
- These macros print their arguments using debug formatting, which means the values being compared must implement `PartialEq` and `Debug` traits.
- They are derivable traits so it's usually as simple as adding `#[derive(PartialEq, Debug)]`

### Adding Custom Failure Messages

- Any arguments after the required arguments are passed along to the `format!` macro.
- Custom messages are useful for document what an assertion means.

### Checking for Panics with should_panic

- By adding the attribute `should_panic` to our test function, the test passes if the code inside the function panics.
- Tests that use `should_panic` can be imprecise as it would pass even if the test panics for a different reason. To make it more precise, we can add optional `expected` parameter to `should_panic` attribute.
- `expected` parameter is a substring of the message that the function panics with.

### Using `Result<T, E>` in Tests

- The test function with `Result<T, E>` will return `Ok(())` when test passes and `Err` with `String` when the test fails.
- Writing tests this way enables you to use question mark operator which can be convenient way to write tests that should fail if any operation within them returns `Err` variant.

## 11.2. Controlling How Tests are Run

- The default behaviour of the binary produced by `cargo test` is to run all the tests in parallel and capture output generated duing test runs, preventing the output from being displayed and making it easier to read the output related to the test results.

### Running Tests in Parallel or Consecutively

- By default the test runs parallelly using threads so you must make sure your tests don't depend on each other or on any shared state.
- To have more fine-grained control over the number of threads, you can send `--test-threads` flag.
- To not use any parallelism run tests by `cargo test -- --test-threads=1`.

### Showing Function Output

- By default, if a test passes, Rust's test library captures anything printed to standard output.
- If a test fails, we'll see whatever was printed to standard output with the rest of the failure message.
- If we want to see printed values for passing tests as well, pass `--show-output` flag.

### Running a Subset of Tests by Name

- You can choose which tests to run by passing `cargo test` the name or names of the tests you want to run as an argument.

### Ignoring Some Tests Unless Specifically Requested

- You can annnotate the tests using the `ignore` attribute to exclude them.
- If we want to run only the ignored tests, we can use `cargo test -- --ignored`.
