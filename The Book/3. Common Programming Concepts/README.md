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

## 3.2. Data Types

- Rust is a `statically typed` language, which means that it must know the types of all variables at compile time.
- If multiple types are possible, compiler will display an error to add type annotation.

### Scalar Types

A `scalar` type represents a single value.

#### Integer Types

- An `integer` is a number without a fractional component.
- The built-in integer types are:
  | **Length** | **Signed** | **Unsigned** |
  | :--------: | :--------: | :----------: |
  | 8-bit | i8 | u8 |
  | 16-bit | i16 | u16 |
  | 32-bit | i32 | u32 |
  | 64-bit | i64 | u64 |
  | 128-bit | i128 | u128 |
  | arch | isize | usize |
- `Signed` and `unsigned` refer to whether it's possible for the number to be negative.
- Signed number are stored using `two's` complement representation.
- Each signed variant can store numbers from -2^(n-1) to 2^(n-1) - 1.
- Each unsigned variants can store numbers from 0 to 2^n - 1.
- `isize` and `usize` types depend on the architecture of the computer, program is running on.
- Default type is `i32`
- Integer literals can be written in any of the following form:
  | **Number literals** | **Example** |
  |:-------------------:|:-----------:|
  | Decimal | 98_222 |
  | Hex | 0xff |
  | Octal | 0o77 |
  | Binary | 0b1111_0000 |
  | Byte (u8 only) | b'A' |

#### Integer Overflow

- If you try to change the integer variable to a value outside its range, `integer overflow` will occur which can lead to:
  - If compiling in debug mode, it can be caught.
  - If compiling in release mode, it will not be caught and if happened at runtime, it will perform `two's complement wrapping` (value will start from 0 or lowest value again).

#### Floating-Point Types

- They are numbers with decimal points.
- `f32` and `f64`, which are 32-bit and 64-bit in size are available.
- Default type is `f64` as on modern CPUs they are roughly the same speed.
- They are all signed.
- They are represented according to `IEEE-754` standard.
- `f32` is a single-precision float and `f64` has double precision.

#### Numeric operations

- Rust supports mathematical operations for all number types:
  - addition (+)
  - subtraction (-)
  - multiplication (\*)
  - division (/)
  - remainder (%)

#### Boolean Type

- Has two possible values: true and false
- One byte in size
- Specified using `bool`

#### Character Type

- `char` literal is specified using single quotes, as opposed to string literals.
- Four bytes in size and represents a Unicode Scalar Value

### Compound Types

`Compound types` can group multiple values into one type.

#### Tuple Type

- General way of grouping together a number of values with a variety of types.
- Have a fixed length.
- To access the element, destructure using pattern matching or accessing by index

  ```rust
  fn main() {
      let tup: (i32, f64, u8) = (500, 6.4, 1);

      let (x, y, z) = tup;

      let six_point_four = x.1;
  }
  ```

- Tuple without any values is called `unit` and represents an empty value or an empty return type.
  - Expressions implicitly return the unit value if they don't return any other value.

#### Array Type

- Every element of an array must have the same type.
- Arrays in Rust have a fixed length.
- Useful when you want your data allocated on the stack rather than the heap or when you want to ensure you always have a fixed number of elements.
- `Vector` is a similar collection type provided by standard library that is allowed to change size.
- You write an array like this:
  ```rust
  let a: [i32; 5] = [1,2,3,4,5];
  ```
- You can also initialize an array to contain the same value by:
  ```rust
  let a = [3; 5]; // same as let a = [3,3,3,3,3]
  ```
- An array is a single chunk of memory of a known, fixed size that can be allocated on stack. You can access by indexing like `a[0]`.

#### Invalid Array Element Access

- Attempt to access invalid element will result in a runtime error and cause the program to panic.
- Other low-level languages can let invalid memory to be accessed but Rust protects by immediately exiting.

## 3.3. Functions

### Functions

- Rust use `snake case` as convention for function and variable names.
- Can call any function defined by entering name followed by a set of parentheses.
- Rust doesn't care where the function is defined, only cares if it's defined in a scope that can be seen by caller.

### Parameters

- `Parameters` are special variables that are part of a function's signature.
- When a function has parameters, you can provide it with arguments.
- In function signatures, you must declare the type of parameters.

### Statements and Expressions

- Function bodies are made up of a series of statements optionally ending in an expression.
- `Statements` are instructions that perform some action and do not return a value.
- `Expressions` evaluate to a resultant value.
  - Expressions to not include ending semicolons.

### Functions with Return Values

- We must declare the type of return value with `->`.
- In Rust, return value is synonymous with the value of the final expression in the block of the body of a function.
- You can return early by `return` keyword and specifying a value but if not it will return the last expression implicitly.

## 3.4. Comments

- Comments can be made by starting with `//`.
- Can be placed at the end of lines.
- Usually used on a separate line above the code it's annotating.
