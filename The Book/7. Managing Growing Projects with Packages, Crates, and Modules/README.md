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
