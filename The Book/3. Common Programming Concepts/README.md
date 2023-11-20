# 3. Common Programming Concepts

## 3.1. Variables and Mutability

- Variables are immutable by default, thus reassigning value to it will cause a compile time error.
- To make it mutable, add `mut` in front of the variable name.

### Constants

- `Constants` are values that are bound to a name and are not allowed to change. Differences between constants and variables are:
  - You can't use `mut` with constants as they are always immutable.
  - Declare constants with `const` keyword and type of the value _must_ be annotated.
  - Constants can be declared in any scope, including global scope, making them useful for values that many parts of code need to know.
  - Constants can only be set to a constant expression, not result of a value computed at runtime.
  ```rust
  const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
  ```
- Naming convention for constants is to use all uppercase with underscores between words.
- Constants are valid for the entire time a program runs, within the scope they were declared.

### Shadowing

- You can declare a new variable with the same name as a previous variable. This is called `shadowing`.
- Shadowing is different from marking a variable as `mut` because we can perform transformations on a value but have the variable be immutable after. We are effectively creating a new variable when we use `let` keyword again, meaning we can change the type of value.
- This change of type is not valid when using `mut` and will generate compile-time error.
