# 7. Managing Growing Projects with Packages, Crates, and Modules

- You can create scopes and change which names are in or out of scope. You can't have two items with the same name in the same scope.
- `Module system` in Rust include:
  - `Packages`: A Cargo feature that lets you build, test, and share crates
  - `Crates`: A tree of modules that produces a library or executable
  - `Modules` and `use`: Let you control the organization, scop, and privacy of paths
  - `Paths`: A way of naming an item, such as a struct, function or module

## 7.1. Packages and Crates

- A `crate` is the smallest amount of code that the Rust compiler considers at a time.
- `Binary crates` are programs you can compile to an executable that you can run. Each must have a function called `main` that defines what happens when the executable runs.
- `Library crates` don't have a `main` function and don't compile to executable. Instead, they define functionality intended to be shared with multiple projects.
  - Most times, Rustaceans use `crate` interchangeably with general programming concept of `library`.
- The `crate root` is a source file that the Rust compiler starts from and makes up the root module of your crate.
- A `package` is a bundle of one or more crates that provides a set of functionality.
- A package contains a `Cargo.toml` file that describes how to build those crates.
- Can contain as many binary crates, but at most one library crate.
- If the package directory contains `src/lib.rs`, Cargo knows the package contains a library crate with the same name as the package.
- A package can have multiple binary crates by placing files in the `src/bin` directory.

## 7.2. Defining Modules to Control Scope and Privacy

### Modules Cheat Sheet

- The compiler first looks in the crate root file (`src/lib.rs` or `src/main.rs`) for code to compile.
- In the root file, you can declare modules (`mod garden`). The compiler will look for modules code in:
  - Curly brackets after module declaration
  - In `src/garden.rs` file
  - In `src/garden/mod.rs` file
- In other files than crate root, you can declare submodules. For example `mod vegetables;` in `src/garden.rs`
- You can now refer to code in that module from anywhere else in that same crate. `crate::garden::vegetables::Asparagus`
- Code within a module is private from its parent modules by default. To make it public, declare it with `pub mod` instead of `mod`. To make items within a public module public as well, use `pub` before their declarations.
- Within a scope, the `use` keyword creates shortcuts to items to reduce repetition of long paths.

### Grouping Related Code in Modules

- `Modules` let us organize code within a crate for readability and easy reuse. They also allow us to control the `privacy` of items.
- Private items are internal implementation details not available for outside use.
- Modules defined in the same module are called `siblings`.
- If module A is contained inside module B, we say module A is `child` of module B and module B is the `parent` of module A.

## 7.3. Paths for Referring to an Item in the Module Tree

- A path can be either:
  - An `absolute path` that is the full path starting from a crate root; for code from an external crate, the absolute path begins with crate name, and for code in current crate, it starts with the literal `crate`.
  - A `relative path` starts from the current module and uses `self`, `super` or an identifier in the current module.
- Choosing which depends on whether you are more likely to move item definition code separately from or together with the code that uses the item.
- Our preference in general is to specify absolute paths because it's more likely to move code definitions and item calls independently of each other.
- In Rust, all items are private to parent modules by default.
- Making module public doesn't make its contents public. We need to make items within the module public as well.

### Best Practices for Packages with a Binary and a Library

- Typically, binary crate will have just enough code to start executable that calls code in the library crate.
- This lets other projects benefit from the functionality that the package provides.
- Module tree should be defined in `src/lib.rs`. Then, any public items can be used in the binary crate by starting paths with the name of the package.

### Starting Relative Paths with super

- Using `super` allows us to reference an item in the parent module,

### Making Structs and Enums Public

- If we use `pub` before a struct definition, we make the struct public but the fields will still be private.
- If public struct has a private field, it needs to provide a public associated function that constructs an instance of it.
- In contrast, if we make a public enum, all of its variants are public.

## 7.4. Bringing Paths Into Scope with the use Keyword

- Having to write out the paths to call functions can be inconvenient, so we can create a shortcut to a path with `use` keyword once, and then use the shorter name everywhere else in the scope.
- Adding `use` and a path in a scope is similar to creating a symbolic link in the filesystem.
- `use` only creates the shortcut for the particular scope in which the `use` occurs.

### Creating Idiomatic use Paths

- Bringing a function's parent module into scope is more idiomatic because specifying the parent module when calling the function makes it clear that the function isn't locally defined.
- When brining structs, enums, or other items with `use`, it's idiomatic to specify the full path. (Just a convention) Exception is when bringing two items with same name.

### Providing New Names with the as Keyword

- We also can specify `as` and a new local name, or `alias` for the type. This solves problem of bringing two types of the same name.

```rust
use std::fmt::Result;
use std::io::Result as IoResult;
```

### Re-exporting Names with pub use

- When a name is brought to scope with `use` keyword, the name available in the new scope is private.
- To enable the code that calls our code to refer to that name as if it had been defined in that code's scope, we can do `pub use`.
- Re-exporting is useful when the internal structure of the code is different from how programmers calling your code would think about the domain.

### Using External Packages

- Pulling packages into my package can be done by: listing them in `Cargo.toml` file and using `use` to bring items from their crates into scope.

### Using Nested Paths to Clean Up Large use Lists

```rust
use std::{cmp::Ordering, io};
use std::io::{self, Write};
```

- We can use nested paths to bring the same items into scope in one line.
- We can use a nested path at any level in a path, which is useful when combining two `use` statements that share a subpath.

### The Glob Operator

```rust
use std::collections::*
```

- If we want to bring all public items defined in a path into scope, we can use `*` glob operator.
- The glob operator can make it harder to tell what names are in scope and where a name was defined.
- The glob operator is often used when testing to bring everything under test into the `tests` module.
