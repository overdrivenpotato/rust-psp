# Installing Rust and Rust-PSP tools

Follow the directions on [https://rustup.rs/](https://rustup.rs) to install Rust for the operating system of your choice. 
Note, Windows users will need to install Microsoft Visual Studio Build tools as a prerequisite.

To confirm you have rust installed correctly, run the following commands:
```sh
cargo new --bin hello
cd hello
cargo run --release
```
Your output should look something like this:
```
[paul@P50-Arch hello]$ cargo run --release
   Compiling hello v0.1.0 (/tmp/hello)
    Finished release [optimized] target(s) in 0.23s
     Running `target/release/hello`
Hello, world!
```

Next we need to add some special components to our rust installation that rust-psp needs.

Firstly we need the nightly compiler, because there are special unstable features
needed by rust-psp that aren't available in stable rust.

```sh
rustup default nightly
```

Refer to the [rust-psp README](https://github.com/overdrivenpotato/rust-psp/blob/master/README.md#dependencies)
for the current minimum rust version needed to use rust-psp. 
Compare to what you have installed by running `rustc --version`.

Next we need the source code for the rust compiler.
```sh
rustup component add rust-src
```

Finally, we're ready to install the rust-psp tools
```sh
cargo install cargo-psp
```
