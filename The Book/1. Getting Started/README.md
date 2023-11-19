# 1. Getting Started

## 1.1. Installation

On Linux or MacOS, run the following code to install Rust:

```sh
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

To verify installation:

```sh
rustc --version
```

To update to latest Rust version:

```sh
rustup update
```

## 1.2. Hello, World!

Creating a simple program that prints `Hello, world!`.
Source code can be found in [hello_world folder](./hello_world/README.md)

> If using more than one word for a filename, use an underscore to separate them. (e.g. hello_world.rs rather than helloworld.rs)

### Anatomy of a Rust Program

```Rust
fn main() {
}
```

- `main` function is special that it is always the first code to be run in Rust program.
- Good style to place opening curly bracket on the same line as function declaration.
- Standard Rust style can be ensured by using a formatter tool called `rustfmt`.
- Rust style is to indent with 4 spaces **not tab**.
- Using `!` in means you are calling a macro instead of a normal function.
- End the line with `;`, indicating that the expression is over and next one is ready to begin.

### Compiling and Running are Separate Steps

- Rust is an `ahead-of-time compiled language`.
  - You can compile and give the executable to someone else and they can run it without having Rust installed.

## 1.3. Hello, Cargo!

- `Cargo` is Rust's build system and package manager. It handles things like building code, downloading libraries, etc.

### Creating a Project with Cargo

You can create a new project using:

```sh
cargo new <package-name>
```

- It will initialize a new Git repo, if not run within an existing Git repo.
- You should be able to see `Cargo.toml` file is generated:
  - `[package]` section contains configuration of the package.
  - `[dependencies]` section contains the list of project's dependencies.
- Cargo expects your source files to live in `src` directory.
- Top-level project directory is just for README files, license, configs, etc.

### Building and Running a Cargo Project

- Running `cargo build` in the project directory will create an executable in `target/debug/hello_cargo` because the default build is a debug build.
- It will create `Cargo.lock` file that keeps track of the exact versions of deps in the project.
- We can also compile and run at the same time using `cargo run`.
- If source code remains unchanged, Cargo will figure out and will not rebuild the project.
- `cargo check` is used to check if the project compiles but it doesn't produce an executable.

### Building for Release

- `cargo build --release` will compile the project with optimizations.
- It will create an executable in `target/release` directory.
- The optimization makes the Rust code run faster but it takes longer to compile.
