.PHONY: blinky blinky-elf clean

blinky:
	cargo build --example blinky

blinky-elf:
	objdump -h target/riscv32imac-unknown-none-elf/debug/examples/blinky

clean:
	cargo clean
