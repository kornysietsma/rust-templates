# rust script sample

This is not a template - it's more of a minimal sample.

It demonstrates using the experimental rust script mode, where you can create a single script which bundles all dependencies - in this case [script.rs](./script.rs)

## Running

Just run `./script.rs` - assuming it is executable

The magic here is the shebang at the top of [script.rs](./script.rs) :

```rust
#!/usr/bin/env cargo +nightly -Zscript
```

Note this assumes cargo is in your path - if you want to run this from outside a shell you need:

```rust
#!/usr/bin/env -S PATH=/Users/${USER}/.cargo/bin:${PATH} cargo +nightly -Zscript
```

This means cargo is run with the nightly features enabled, and `-Zscript` tells it to run the embedded script.

## Testing and other cargo commands

Cargo has no cargo.toml - so for cargo commands you need to enable :

- nightly features with `cargo +nightly`
- scripting with `-Zscript`
- point to the manifest with `--manifest-path script.rs`

So to run tests:

```sh
cargo +nightly run -Zscript --manifest-path script.rs
```

Or you can run the main command more verbosely with:

```sh
cargo +nightly run -Zscript --manifest-path script.rs
```

Note lots of things like `cargo outdated` won't work yet. Updating packages is up to you!  Similarly IDEs like VSCode probably won't cope for a while, but all the cli tools work.

## Logging

Logging defaults to info level.

For different log levels set RUST_LOG=debug etc. in the environment
