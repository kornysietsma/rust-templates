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

## Updating the templates

This is more of a reminder for me than anything!

You can't actually compile the templates in place, so you basically need to work on copies.

First open the `cli` and `cli-smple templates here and:

- update cargo.toml
- update anything else like cargo-generate.toml
- make a new blank directory
- run cargo generate:

```shell
cargo install cargo-generate
cargo generate --path $HOME/Dropbox/prj/dev/rust/rust-templates/cli-simple
# and
cargo generate --path $HOME/Dropbox/prj/dev/rust/rust-templates/cli
```

then check each works:

```shell
cargo update
cargo build
cargo test
cargo run -- -h
```

then copy back `Cargo.lock` and anything else that might need updating.
