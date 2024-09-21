.PHONY : all

PROGRAM = max_element

all: run

build:
	@cd user && make -s

run: build
	@qemu-riscv64 user/target/riscv64gc-unknown-none-elf/release/$(PROGRAM)

%:
	@cd user && make -s $@
