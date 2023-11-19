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
