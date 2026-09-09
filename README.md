# Leptos `--split` + WebSocket Compilation Failure

[Repo](https://github.com/Ritzier/leptos-websocket-not-working)

Minimal reproduction for a compilation failure when using Leptos `--split` (**lazy loading**) together with WebSocket
functionality.

## Problem

Running `cargo leptos serve --split` causes `wasm-bindgen` to panic while processing generated WASM:

`wasm-bindgen` failed with:

```
thread 'main' panicked at crates/cli-support/src/descriptor.rs:324:15:
index out of bounds: the len is 0 but the index is 0
```

The application builds successfully without `--split`, but fails during the WASM/JS generation step when splitting is
enabled.

## Environment

### Leptos

```toml
leptos = { version = "0.8.20" }
leptos_router = { version = "0.8.15" }
leptos_axum = { version = "0.8.10", optional = true }
leptos_meta = { version = "0.8.6" }
```

### Tooling

```
Package           Installed  Latest    Needs update
cargo-leptos      v0.3.7     v0.3.7    No
wasm-bindgen-cli  v0.2.128   v0.2.128  No
```

Both are currently up to date in this environment.

## Reproduction

Clone this repository and run:

`cargo leptos serve --split`

The build progresses through WASM compilation and splitting, then fails when `wasm-bindgen` is invoked:

```
Front splitting out lazy-loaded WASM files
Finished WASM splitting in 240.67201ms
Front generating JS/WASM with wasm-bindgen wasm-bindgen failed with:

thread 'main' panicked at crates/cli-support/src/descriptor.rs:324:15:
index out of bounds: the len is 0 but the index is 0
```

## Expected Behavior

`cargo leptos serve --split` should successfully compile the application and generate the required JavaScript/WASM
output.

## Actual Behavior

`wasm-bindgen` panics with:

```
index out of bounds: the len is 0 but the index is 0
```

The failure happens after the WASM splitting step and while generating the JS/WASM bindings.

## Full command output:

<details> <summary>Full build log</summary>

```log
┌─{16:09:09}-[~/Projects/Ritzier/leptos-websocket-not-working]                                        (leptos-websocket-not-working:main X)
└─> cargo leptos serve --split
    Metadata keys ["env"] from metadata.leptos are not recognized and will be ignored
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling workspace v0.1.0 (/home/ritzier/Projects/Ritzier/leptos-websocket-not-working)
   Compiling workspace v0.1.0 (/home/ritzier/Projects/Ritzier/leptos-websocket-not-working)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.70s
warning: the following packages contain code that will be rejected by a future version of Rust: proc-macro-error2 v2.0.1
note: to see what the problems were, use the option `--future-incompat-report`, or run `cargo report future-incompatibilities --id 1`
       Cargo finished cargo build --package=workspace --lib --target-dir=/mnt/nvme/RustTarget/front --target=wasm32-unknown-unknown --no-de
fault-features --features=hydrate
       Front splitting out lazy-loaded WASM files
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.94s
warning: the following packages contain code that will be rejected by a future version of Rust: proc-macro-error2 v2.0.1
note: to see what the problems were, use the option `--future-incompat-report`, or run `cargo report future-incompatibilities --id 1`
    Finished WASM splitting in 240.67201ms
       Front generating JS/WASM with wasm-bindgen
       Cargo finished cargo build --package=workspace --bin=workspace --no-default-features --features=ssr
wasm-bindgen failed with:

thread 'main' (3752526) panicked at crates/cli-support/src/descriptor.rs:324:15:
index out of bounds: the len is 0 but the index is 0
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

Error:
   0: at `src/command/serve.rs:8:46`
   1: at `src/compile/front.rs:70:38`
   2: wasm-bindgen failed

Location:
   src/compile/front.rs:211

Backtrace omitted. Run with RUST_BACKTRACE=1 environment variable to display it.
Run with RUST_BACKTRACE=full to include source snippets.
```

</details>
