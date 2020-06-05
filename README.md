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

## Features / Roadmap

- [x] `core` support
- [x] PSP system library support
- [x] `alloc` support
- [x] `panic = "unwind"` support
- [x] Macro-based VFPU assembler
- [x] Full 3D graphics support
- [x] Migrate to LLVM-based linker
- [ ] Remove PSP toolchain dependency: rewrite `psp-prxgen`, `pack-pbp`, and
      `PRXEncrypter`

## Dependencies: Rust

To compile for the PSP, you will need a rust **nightly** version equal to or
later than `2020-06-04`.

```sh
$ rustup toolchain add nightly
```

You also need `xargo` installed (for compilation of rust-internal crates).

```sh
$ cargo install xargo
```

## Dependency: PSP Toolchain

You need the [psp toolchain] installed, and the binaries in your `$PATH`.

NB: The main binary we need is `psp-prxgen`, ideally this will eventually be
ported to rust.

[psp toolchain]: https://github.com/pspdev/psptoolchain

## Running Examples

Enter one of the example directories, `examples/hello-world` for instance, and
run `make`.

This will create an `EBOOT.PBP` under `target/mipsel-sony-psp/release/`

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

You can also use `psplink` and `pspsh` to run the `.prx` under
`target/mipsel-sony-psp/release` if you prefer. Refer to the installation and
usage guides for those programs.

### Debugging

`psp-gdb` is currently too old to support printing rust types.

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

And a `Makefile` file in the root as well, like the following (you will have to
modify `TARGET_NAME`):

```make
# Change this to the cargo package name
TARGET_NAME = package-name

.PHONY: release
release:
	@RUSTFLAGS="-C link-dead-code" \
		xargo build --target mipsel-sony-psp --release

	@psp-prxgen \
		"$(CARGO_TARGET_DIR)"/mipsel-sony-psp/release/$(TARGET_NAME) \
		"$(CARGO_TARGET_DIR)"/mipsel-sony-psp/release/$(TARGET_NAME).prx

	@mksfo "PSP Rust Cube" "$(CARGO_TARGET_DIR)"/mipsel-sony-psp/release/PARAM.SFO
	@pack-pbp "$(CARGO_TARGET_DIR)"/mipsel-sony-psp/release/EBOOT.PBP \
		"$(CARGO_TARGET_DIR)"/mipsel-sony-psp/release/PARAM.SFO NULL NULL NULL NULL NULL \
		"$(CARGO_TARGET_DIR)"/mipsel-sony-psp/release/$(TARGET_NAME).prx NULL

	@echo Saved to "$(CARGO_TARGET_DIR)"/mipsel-sony-psp/release/EBOOT.PBP
```

Now you can simply run `make` to build your `EBOOT.PBP` file. The executable
**must** be built with `--release` due to a bug in this crate, or it will not
work as expected.

The Makefile will eventually be replaced by a cargo subcommand, at which point
this README will be updated.

## Known Bugs

This crate **breaks** on debug builds. Likely due to the ABI mapper
implementation.

This can be worked around by enabling optimization for debug builds

```toml
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
