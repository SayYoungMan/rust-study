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
