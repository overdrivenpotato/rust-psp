# rust-psp

![PSP Picture](psp-hello-world.jpg)

A library for building full PSP modules, including both PRX plugins and regular
homebrew apps.

```rust
#![no_std]
#![no_main]

psp::module!("sample_module", 1, 1);

fn psp_main() {
    psp::dprintln!("Hello PSP from rust!");
}
```

See `examples` directory for sample programs.

## Rust dependencies 

Currently, this crate depends on the `mipsel-sony-psp` target that was recently
added in Rust nightly. 

```sh
rustup update nightly
rustup default nightly

```
It also requires `xargo` to build libcore, liballoc, and libc.
```sh
cargo install xargo
```

## Other Dependencies

You will need the [psp toolchain] installed, and the binaries in your `$PATH`.

[psp toolchain]: https://github.com/pspdev/psptoolchain

Work is underway to remove this dependency.

## Usage

Enter one of the example directories, `examples/hello-world` for instance,
and type `make`. 

This will create an `EBOOT.PBP` under `target/mipsel-sony-psp/release`

Assuming you have a PSP with custom firmware
installed, you can simply copy this file into a new directory under `PSP/GAME`
and it will show up in your XMB menu. 

You can also use `psplink` and `pspsh`
to run the `.prx` under `target/mipsel-sony-psp/release` if you prefer.
Refer to the installation and usage guides for those programs.

`psp-gdb` is currently too old to support printing rust types.

To use the `psp` crate in your own rust programs, add it to `Cargo.toml`
as a git dependency:

```toml
[dependencies]
psp = { git = "https://github.com/overdrivenpotato/rust-psp" }
```

You will also need to copy `Xargo.toml`. Now you can run 

```sh
xargo build --target=mipsel-sony-psp
```
to build an `elf`.

Unfortunately, the PSP doesn't run unmodified
elfs, so you should copy and adapt our `Makefile` from one of the examples.
This Makefile will eventually be replaced by a cargo subcommand, at which point
this README will be updated.

Optionally, you can copy .cargo/config to avoid typing `--target=mipsel-sony-psp`
every time you build.

## Features / Roadmap

- [x] `core` support
- [x] PSP system library support
- [x] `alloc` support
- [x] `panic = "unwind"` support
- [x] Macro-based VFPU assembler
- [ ] Migrate to LLVM-based linker
- [ ] Rewrite `psp-prxgen` in rust?

## Known Bugs

This crate **breaks** on debug builds. Likely due to the ABI mapper
implementation. 

This can be worked around by adding

```toml
[profile.dev]
opt-level="z"
```
This enables optimization for debug builds.

to Cargo.toml, or simply building with --release.

## `error[E0460]: found possibly newer version of crate ...`

If you get an error like this:

```
error[E0460]: found possibly newer version of crate `panic_unwind` which `psp` depends on
 --> src/main.rs:4:5
  |
4 | use psp::dprintln;
  |     ^^^
  |
  = note: perhaps that crate needs to be recompiled?
```

Simply clean your target directory and it will be fixed:

```sh
$ cargo clean
```
