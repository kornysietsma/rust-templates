# Rust Templates

This is where I store my rust templates

There's a few so far:

- cli-simple - a simple command-line app in a single module
- cli - a sligtly more complex command-line app with a lib.rs and integration tests 

## using templates

For each template you can create an app based on it using [cargo-generate](https://cargo-generate.github.io/cargo-generate/index.html)

install cargo-generate:

```sh
cargo install cargo-generate
```

generate a project based on a template

```sh
cargo generate kornysietsma/rust-templates cli-simple
```

You will be prompted for the project name
