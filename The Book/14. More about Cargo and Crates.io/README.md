# 14. More about Cargo and Crates.io

## 14.1. Customizing Builds with Release Profiles

- `Release profiles` are predefined and customizable profiles with different configurations that allow a programmer to have more control over various options for compiling code.
- Cargo has two main profiles: `dev` profile and `release` profile.
- By adding `[profile.*]` sections for any profile you want to customize, you override any subset of default settings.
