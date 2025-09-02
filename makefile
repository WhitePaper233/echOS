TARGET = riscv64gc-unknown-none-elf

.PHONY: kernel run

kernel:
	cargo +nightly build -Zbuild-std=core,alloc --target $(TARGET)

run:
	qemu-system-riscv64 -machine virt -bios none -kernel target/$(TARGET)/debug/kernel

clean:
	cargo clean
