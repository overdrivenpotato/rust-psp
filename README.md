<h1 align="center">rust-psp</h1>

<p align="center"><img src="demo.gif" /></p>
<p class="" align="center">
    <a href="https://ci.mijalkovic.ca/teams/rust-psp/pipelines/rust-psp/jobs/run-tests-for-master/">
        <img src="https://ci.mijalkovic.ca/api/v1/teams/rust-psp/pipelines/rust-psp/jobs/run-tests-for-master/badge">
    </a>
    <a href="https://crates.io/crates/psp">
        <img src="https://img.shields.io/crates/v/psp.svg?style=flat-square">
    </a>
    <a href="https://docs.rs/psp">
        <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square">
    </a>
</p>
<p align="center">
    A library for building full PSP modules, including both PRX plugins and regular
    homebrew apps.
</p>

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
- [x] No dependency on PSPSDK / PSPToolchain
- [ ] Reach full parity with user mode support in PSPSDK
- [ ] Add support for creating kernel mode modules
- [ ] Port definitions to `libc` crate
- [ ] Add `std` support
- [ ] Automatically sign EBOOT.PBP files to run on unmodified PSPs
- [ ] Implement / reverse undiscovered libraries

## Dependencies

To compile for the PSP, you will need a Rust **nightly** version equal to or
later than `2020-06-04`. Please install Rust using https://rustup.rs/

Use the following if you are new to Rust. (Feel free to set an override manually
per-project).

```sh
$ rustup toolchain add nightly
```

You also need `cargo-psp` installed:

```sh
$ cargo install cargo-psp
```

## Running Examples

Enter one of the example directories, `examples/hello-world` for instance, and
run `cargo psp`.

This will create an `EBOOT.PBP` file under `target/mipsel-sony-psp/debug/`

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
`target/mipsel-sony-psp/debug/` if you prefer. Refer to the installation and
usage guides for those programs.

### Debugging

`psp-gdb` is currently too old to support printing Rust types. `rust-lldb` may
be possible but it has not be experimented with yet.

## Usage

To use the `psp` crate in your own Rust programs, add it to `Cargo.toml` like
any other dependency:

```toml
[dependencies]
psp = "x.y.z"
```

In your `main.rs` file, you need to setup a basic skeleton like so:

```rust
#![no_std]
#![no_main]

// Create a module named "sample_modules" with version 1.0
psp::module!("sample_module", 1, 0);

fn psp_main() {
    psp::dprintln!("Hello PSP from rust!");
}
```

Now you can simply run `cargo psp` to build your `EBOOT.PBP` file. You can also
invoke `cargo psp --release` to create a release build.

If you would like to customize your EBOOT with e.g. an icon or new title, you
can create a `Psp.toml` file in the root of your project. Note that all keys are
optional:

```toml
title = "XMB title"
xmb_icon_png = "path/to/24bit_144x80_image.png"
xmb_background_png = "path/to/24bit_480x272_background.png"
xmb_music_at3 = "path/to/ATRAC3_audio.at3"
```

More options can be found in the schema defintion [here](/cargo-psp/src/main.rs#L11-L91).

## Known Bugs

This crate **breaks** on builds with `opt-level=0`. Likely due to a bug in EABI
interop. `cargo-psp` patches over this by passing `-C opt-level=3`.

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
