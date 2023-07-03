# Elastic Stream

Elastic Stream is an open-source, elastic streaming platform, designed to leverage the strength of cloud infrastructure and distinguish itself with outstanding scalability, fault-tolerance, cost-effectiveness, and security.

## Architecture

![Arch](docs/images/elastic-stream-arc.png)

Elastic Stream comprises three components: [Placement Driver(PD)](./pd), [Range Server](./range-server/), and [SDKs](./sdks).

## How to Build

This project employs several distinct programming languages. The Placement Driver is developed utilizing Go, whereas the Range Server leverages the Rust language. In addition, we furnish two SDK implementations with Java and Rust.

For the parts crafted in Go and Java, you can locate the corresponding build commands in the [`./pd`](./pd) and [`./sdks/frontend-java`](./sdks/frontend-java) directories respectively.

### Rust Crates

Considering the necessity for unified build and testing process, streamlined dependency management, and consistent project-wide coding practices, we've placed all our Rust crates within one workspace.

Particularly, some crates might have additional dependencies when building. please run `./scripts/install_deps.sh` before you kick things off. Given that the top-level directory of the project doubles as a crate root, you're empowered to run commands below to build it.

```sh
# debug mode.
cargo build
# release mode.
cargo build --release
```

[cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov) is a tool used for generating test coverage reports for Rust codebase. FYI, Here are some related commands.

```sh
# run tests(via cargo test) and print the coverage summary to stdout.
cargo llvm-cov
# use cargo run rather than cargo test.
cargo llvm-cov run
# generate html report and open it.
cargo llvm-cov --open
```

## How to Run

Begin by starting PD by following the instructions provided in [`pd/README.md`](./pd/README.md), then launch the Range Server, there are three differents ways:
* After completing the build process, execute `./target/debug/range-server -c etc/range-server.yaml --log etc/range-server-log.yaml`.
* Alternatively, you can directly use `cargo run --bin range-server -- -c etc/range-server.yaml --log etc/range-server-log.yaml`.
* Additionally, we have configured a shortcut entry in the "Run and Debug" panel of VSCode for you to quickly start the Range Server.

Finally, start the client.

## Contributing
See [CONTRIBUTING.md](./CONTRIBUTING.md).

## Notes

### **communicating-between-sync-and-async-code**
`Store` module is built on top of io-uring directly. The `Server` module, however, is built using `tokio-uring`, following thread-per-core paradigm, which as a result is fully async. Reading and writing records between these two modules involve communication between async and sync code, as shall comply with [the following guideline](https://docs.rs/tokio/latest/tokio/sync/mpsc/index.html#communicating-between-sync-and-async-code)

## Run with Address Sanitizer

Sometimes you have to deal with low-level operations, for example, interacting with DMA requires page alignment memory. Unsafe code is required to handle these cases and address sanitizer would be helpful to maintain memory safety.

```sh
RUSTFLAGS=-Zsanitizer=address cargo test test_layout -Zbuild-std --target x86_64-unknown-linux-gnu
```
Read the [following link](https://doc.rust-lang.org/beta/unstable-book/compiler-flags/sanitizer.html) for more advanced usage.
