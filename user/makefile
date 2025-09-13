# ===================================================================
# echOS User Space Application Build System
# ===================================================================
# This makefile builds user-space applications for the echOS operating system.
# It compiles Rust applications to RISC-V binaries that can run on echOS.

# =============
# Build Configuration
# =============

# Target triple for Rust compilation - RISC-V 64-bit with GC extensions, no standard library
TARGET            := riscv64gc-unknown-none-elf

# Build mode - using release for optimized binaries
MODE              := release

# Directory containing user application source files
APP_DIR           := src/bin

# Output directory for compiled binaries
TARGET_DIR        := ../target/$(TARGET)/$(MODE)

# Path to the custom linker script for user applications
LINKER_SCRIPT_DIR := user/src/linker.ld

# =============
# File Discovery
# =============

# Find all Rust source files in the application directory
APPS              := $(wildcard $(APP_DIR)/*.rs)

# Generate ELF executable paths from source files
ELFS              := $(patsubst $(APP_DIR)/%.rs, $(TARGET_DIR)/%, $(APPS))

# Generate binary file paths from ELF executables
BINS              := $(patsubst $(APP_DIR)/%.rs, $(TARGET_DIR)/%.bin, $(APPS))

# =============
# Tool Configuration
# =============

# RISC-V object dump utility for disassembly and analysis
OBJDUMP := rust-objdump --arch-name=riscv64

# RISC-V object copy utility for format conversion and stripping
OBJCOPY := rust-objcopy --binary-architecture=riscv64

# =============
# Build Targets
# =============

# Primary target: Build ELF executables from Rust source
# Uses custom linker script and builds standard library from source
elf: $(APPS)
	@python3 build.py

# Convert ELF executables to raw binary format
# Strips debug symbols and metadata, creating pure machine code
binary: elf
	@$(foreach elf, $(ELFS), $(OBJCOPY) $(elf) --strip-all -O binary $(patsubst $(TARGET_DIR)/%, $(TARGET_DIR)/%.bin, $(elf));)

# Complete build process: compile and convert to binary
build: binary

clean:
	@cargo clean

.PHONY: elf binary build clean