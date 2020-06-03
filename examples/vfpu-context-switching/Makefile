CWD = $(shell pwd)
CARGO_TARGET_DIR ?= ./target

# Change this to the cargo package name
TARGET_NAME = vfpu-context-switching-example

# The name displayed on the PSP Menu
DISPLAY_NAME = "Rust VFPU Context Switching Example"

.PHONY: release
release:
	@RUSTFLAGS="-C link-dead-code" \
		xargo build --target mipsel-sony-psp --release

	@psp-prxgen \
		"$(CARGO_TARGET_DIR)"/mipsel-sony-psp/release/$(TARGET_NAME) \
		"$(CARGO_TARGET_DIR)"/mipsel-sony-psp/release/$(TARGET_NAME).prx

	@mksfo "$(DISPLAY_NAME)" "$(CARGO_TARGET_DIR)"/mipsel-sony-psp/release/PARAM.SFO
	@pack-pbp "$(CARGO_TARGET_DIR)"/mipsel-sony-psp/release/EBOOT.PBP \
		"$(CARGO_TARGET_DIR)"/mipsel-sony-psp/release/PARAM.SFO NULL NULL NULL NULL NULL \
		"$(CARGO_TARGET_DIR)"/mipsel-sony-psp/release/$(TARGET_NAME).prx NULL

	@echo Saved to "$(CARGO_TARGET_DIR)"/mipsel-sony-psp/release/EBOOT.PBP
