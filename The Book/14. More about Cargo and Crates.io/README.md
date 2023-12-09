# 14. More about Cargo and Crates.io

## 14.1. Customizing Builds with Release Profiles

- `Release profiles` are predefined and customizable profiles with different configurations that allow a programmer to have more control over various options for compiling code.
- Cargo has two main profiles: `dev` profile and `release` profile.
- By adding `[profile.*]` sections for any profile you want to customize, you override any subset of default settings.

## 14.2. Publishing a Crate to Crates.io

### Making Useful Documentation Comments

- Rust has `documentation comment`, that will generate HTML documentation. It displays the contents of documentation comments for public API items intended for programmers interested in knowing how to use the crate.
- Documentation comments use `///` and support Markdown notation for formatting.
- We can generate the HTML documentation from the documentation comment by running `cargo doc --open`

#### Commonly Used Sections

- `Examples` to display how to use the code with example.
- `Panics` to show scenarios in which the function being documented could panic.
- `Errors` to describe kinds of errors that might occur under what conditions if function returns a `Result`.
- `Safety` to explaim why the function is unsafe and covering the invariants that the function expects callers to uphold.

#### Documentation Comments as Tests

- You can test an example in the documentation comments with `cargo test` if it has `assert` macros in the examples.

#### Commenting Contained Items

- `//!` adds documentation to the item that contains the comments rather than to the items following the comments.
- Typically they are used inside the crate root file or inside a module to document the crate or the module.

### Exporting a Convenient Public API with pub use

- You can re-export items to make a public structure that's different from your private structure by using `pub use`.
- Re-exporting takes a public item in one location and makes it public in another location, as if it were defined in the other location instead.
- Another common use of `pub use` is to re-export definitions of a dependency in the current crate to make that crate's definitions part of your crate's public API.

### Adding Metadata to a New Crate

- You need to add some metadata in the `[package]` section of the crate's `Cargo.toml` file before you publish the crate such as it's unique name, description and license.

### Deprecating Versions from Crates.io with cargo yank

- Yanking a version prevents new projects from depending on that version while allowing all existing projects that depend on it to continue.
- To yank a version, in the directory of the crate you published, run `cargo yank` and specify which version under `--vers` flag.

## 14.3. Cargo Workspaces

- Cargo offers a feature called `workspaces` that can help manage multiple related packages that are developed in tandem.

### Creating a Workspace

- A workspace is a set of packages that share the same `Cargo.lock` and output directory.
- By sharing one target directory, between crates of same workspace, the crates can avoid unnecessary rebuilding.
- Cargo doesn't assume that crates in a workspace will depend on each other, so we need to be explicit about the dependency relationships.
- To specify which package in the workspace we want to run, add `-p` argument and the package name with `cargo run`.

### Depending on an External Package in a Workspace

- There is only one `Cargo.lock` file at the top level, rather than having one in each crate. This ensures that all crates are using the same version of all dependencies.
- Making all crates in the workspace use the same dependencies means the crates will always be compatible with each other.

#### Adding a Test to a Workspace

- Running `cargo test` in a workspace will run the tests for all the crates in the workspace.
- If you publish the crates in the workspace to crates.io, each crate in the workspace will need to be published separately.
