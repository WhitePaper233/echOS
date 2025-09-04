# Target triple for Rust compilation
TARGET            = riscv64gc-unknown-none-elf

# Bootloader (RustSBI) configuration
BOOTLOADER_DIR    := bootloader
BOOTLOADER_GZ     := $(BOOTLOADER_DIR)/rustsbi-qemu-release.gz
BOOTLOADER_BIN    := $(BOOTLOADER_DIR)/rustsbi-qemu.bin
BOOTLOADER_URL    := https://github.com/rustsbi/rustsbi-qemu/releases/download/Unreleased/rustsbi-qemu-release.gz

# Phony targets
.PHONY: all kernel release run clean-bootloader clean clean-all

# Default target: ensure bootloader is ready
all: $(BOOTLOADER_BIN)

# Ensure bootloader directory exists
$(BOOTLOADER_DIR):
	mkdir -p $@

# Download RustSBI bootloader archive
$(BOOTLOADER_GZ): | $(BOOTLOADER_DIR)
	@echo "Downloading bootloader (rustsbi-qemu) ..."
	curl -fsSL -o $@ $(BOOTLOADER_URL)

# Extract and rename bootloader binary, then clean up
$(BOOTLOADER_BIN): $(BOOTLOADER_GZ)
	@echo "Extracting and renaming bootloader..."
	gunzip -c $< > $(BOOTLOADER_DIR)/rustsbi-qemu-release
	mv $(BOOTLOADER_DIR)/rustsbi-qemu-release $@
	@echo "Cleaning up archive..."
	rm -f $(BOOTLOADER_GZ)

# Build the kernel with nightly Rust in debug mode
kernel:
	RUSTFLAGS="-Clink-arg=-Tkernel/src/linker.ld -Cforce-frame-pointers=yes" \
	cargo +nightly build \
        -Z build-std=core,alloc \
        --bin kernel \
        --target $(TARGET)

# Build the OS in release mode
release:
	RUSTFLAGS="-Clink-arg=-Tkernel/src/linker.ld -Cforce-frame-pointers=yes" \
	cargo +nightly build \
		-Z build-std=core,alloc \
		--bin kernel \
		--target $(TARGET) \
		--release
	rust-objcopy --strip-all target/$(TARGET)/release/kernel -O binary target/$(TARGET)/release/kernel.bin

# Run the kernel in QEMU with the prepared bootloader
run: bootloader release
	qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios $(BOOTLOADER_BIN) \
		-device loader,file=target/$(TARGET)/release/kernel.bin,addr=0x80200000 \
		-s

# Open qemu gdb client
gc:
	riscv64-unknown-elf-gdb \
        -ex 'file target/$(TARGET)/release/kernel' \
        -ex 'set arch riscv:rv64' \
        -ex 'target remote localhost:1234'

# Remove bootloader directory
clean-bootloader:
	@echo "Removing bootloader directory..."
	rm -rf $(BOOTLOADER_DIR)

# Clean all build artifacts
clean:
	@echo "Cleaning project artifacts..."
	cargo clean

# Clean build artifacts and bootloader file
clean-all: clean clean-bootloader
