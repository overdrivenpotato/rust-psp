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

## What about PSPSDK?

This project is a completely new SDK, with no dependency on the original C/C++
PSPSDK. It aims to be a **complete** replacement, with more efficient
implementations of graphics functions, and the addition of missing libraries.

## Features / Roadmap

- [x] `core` support
- [x] PSP system library support
- [x] `alloc` support
- [x] `panic = "unwind"` support
- [x] Macro-based VFPU assembler
- [x] Full 3D graphics support (faster than PSPSDK in some cases!)
- [x] No dependency on PSPSDK
- [ ] Reach full parity with user mode support in PSPSDK
- [ ] Add support for creating kernel mode modules
- [ ] Port definitions to `libc` crate
- [ ] Add `std` support
- [ ] Automatically sign EBOOT.PBP files to run on unmodified PSPs
- [ ] Implement / reverse all libraries missing from PSPSDK

## Dependencies

To compile for the PSP, you will need a rust **nightly** version equal to or
later than `2020-06-04`. Please install Rust using https://rustup.rs/

Use the following if you are new to rust. (Feel free to set an override manually
per-project).

```sh
$ rustup toolchain add nightly
```

You also need `xargo` and `cargo-psp` installed. (`xargo` version must be
relatively recent).

```sh
$ cargo install cargo-psp xargo
```

## Running Examples

Enter one of the example directories, `examples/hello-world` for instance, and
run `cargo psp --release`.

This will create an `EBOOT.PBP` file under `target/mipsel-sony-psp/release/`

Assuming you have a PSP with custom firmware installed, you can simply copy this
file into a new directory under `PSP/GAME` on your memory stick, and it will
show up in your XMB menu.

```
.
└── PSP
    └── GAME
        └── hello-world
            └── EBOOT.PBP
```

### Advanced usage: `PRXEncrypter`

If you don't have a PSP with CFW installed, you can manually sign the PRX using
`PRXEncrypter`, and then re-package it using `pack-pbp`.

### Advanced usage: PSPLink

If you have the PSPSDK installed and have built a working copy PSPLink manually,
you can also use `psplink` and `pspsh` to run the `.prx` under
`target/mipsel-sony-psp/release` if you prefer. Refer to the installation and
usage guides for those programs.

### Debugging

`psp-gdb` is currently too old to support printing rust types. `rust-lldb` may
be possible but it has not be experimented with yet.

## Usage

To use the `psp` crate in your own rust programs, add it to `Cargo.toml`
as a git dependency:

```toml
[dependencies]
psp = { git = "https://github.com/overdrivenpotato/rust-psp" }
```

You will also need a `Xargo.toml` file in the root of your project like so:

```toml
[target.mipsel-sony-psp.dependencies.core]
[target.mipsel-sony-psp.dependencies.alloc]
[target.mipsel-sony-psp.dependencies.panic_unwind]
stage = 1
```

Now you can simply run `cargo psp --release` to build your `EBOOT.PBP` file. The
executable **must** be built with `--release` due to a bug in this crate, or it
will not work as expected. *This should be fixed soon.*

## Known Bugs

This crate **breaks** on debug builds. Likely due to a bug in EABI interop.

This can be worked around by enabling optimization for debug builds

```toml
# Cargo.toml

[profile.dev]
opt-level="z"
```
or simply building with `--release`, like the examples.

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
