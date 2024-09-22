.PHONY : all build objcopy clean clippy

ARCH := riscv64gc-unknown-none-elf
BUILD_MODE := release

PYTHON := python3
CARGO := cargo
OBJCOPY := rust-objcopy

APPS := $(wildcard src/bin/*.rs)
ELFS := $(patsubst src/bin/%.rs, target/$(ARCH)/$(BUILD_MODE)/%, $(APPS))

all: clean objcopy

build:
	@$(PYTHON) build.py

OBJCOPY_FLAG := --strip-all -O binary

objcopy: build
	@$(foreach elf, $(ELFS), \
		$(OBJCOPY) $(elf) $(OBJCOPY_FLAG) \
		$(patsubst target/$(ARCH)/$(BUILD_MODE)/%, target/$(ARCH)/$(BUILD_MODE)/%.bin, $(elf)); \
	)

clean:
	@$(CARGO) $@

clippy:
	@$(CARGO) $@

%:
	@$(CARGO) $@