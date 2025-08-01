# We cannot use $(shell pwd), which will return unix path format on Windows,
# making it hard to use.
cur_dir = $(dir $(abspath $(firstword $(MAKEFILE_LIST))))

TOP := $(cur_dir)
# RUSTFLAGS that are likely to be tweaked by developers. For example,
# while we enable debug logs by default here, some might want to strip them
# for minimal code size / consumed cycles.
CUSTOM_RUSTFLAGS := -C debug-assertions
# RUSTFLAGS that are less likely to be tweaked by developers. Most likely
# one would want to keep the default values here.
FULL_RUSTFLAGS := -C target-feature=+zba,+zbb,+zbc,+zbs,-a $(CUSTOM_RUSTFLAGS) \
	-C link-arg=-T$(cur_dir)ld_interface.ld
# Additional cargo args to append here. For example, one can use
# make test CARGO_ARGS="-- --nocapture" so as to inspect data emitted to
# stdout in unit tests
CARGO_ARGS :=
MODE := release
# Tweak this to change the clang version to use for building C code. By default
# we use a bash script with some heuristics to find clang in current system.
CLANG := $(shell $(TOP)/scripts/find_clang)
AR := $(subst clang,llvm-ar,$(CLANG))
# When this is set to some value, the generated binaries will be copied over
BUILD_DIR :=
# Generated binaries to copy. By convention, a Rust crate's directory name will
# likely match the crate name, which is also the name of the final binary.
# However if this is not the case, you can tweak this variable. As the name hints,
# more than one binary is supported here.
BINARIES := $(notdir $(shell pwd))

ifeq (release,$(MODE))
	MODE_ARGS := --release
endif

default: build test

build:
	RUSTFLAGS="$(FULL_RUSTFLAGS)" TARGET_CC="$(CLANG)" TARGET_AR="$(AR)" \
		cargo build --target=riscv64imac-unknown-none-elf $(MODE_ARGS) $(CARGO_ARGS)
	@set -eu; \
	if [ "x$(BUILD_DIR)" != "x" ]; then \
		for binary in $(BINARIES); do \
			echo "Copying binary $$binary to build directory"; \
			cp $(TOP)/target/riscv64imac-unknown-none-elf/$(MODE)/$$binary $(TOP)/$(BUILD_DIR); \
		done \
	fi

# This is a newly introduced task to allow adjusting stack size.
# Notice there are also other changes required for a stack-reordering
# contract, for example, FULL_RUSTFLAGS is also altered for new flags.
# One can compare this file with the Makefile for minimal-log sample
# for complete differences
STACK_SIZE := 0x80000
adjust_stack_size:
	$(TOP)/deps/ckb-stack-reorg-bootloader/regenerate $(STACK_SIZE) $(shell pwd)
.PHONY: adjust_stack_size

# test, check, clippy and fmt here are provided for completeness,
# there is nothing wrong invoking cargo directly instead of make.
test:
	cargo test $(CARGO_ARGS)

check:
	cargo check $(CARGO_ARGS)

clippy:
	cargo clippy $(CARGO_ARGS)

fmt:
	cargo fmt $(CARGO_ARGS)

# Arbitrary cargo command is supported here. For example:
#
# make cargo CARGO_CMD=expand CARGO_ARGS="--ugly"
#
# Invokes:
# cargo expand --ugly
CARGO_CMD :=
cargo:
	cargo $(CARGO_CMD) $(CARGO_ARGS)

clean:
	cargo clean

prepare:
	rustup target add riscv64imac-unknown-none-elf

.PHONY: build test check clippy fmt cargo clean prepare
