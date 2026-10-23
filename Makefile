# Lindow OS — build & run via QEMU

BOOTLOADER_DIR = bootloader
RUST_DIR       = rust_bindings
BUILD_DIR      = build

NASM    = nasm
CARGO   = cargo
QEMU    = qemu-system-x86_64

BOOT_BIN   = $(BUILD_DIR)/bootloader.bin
KERNEL_BIN = $(BUILD_DIR)/kernel.bin
DISK_IMG   = $(BUILD_DIR)/disk.img

.PHONY: all build run clean

all: build

build: $(DISK_IMG)

$(BOOT_BIN): $(BOOTLOADER_DIR)/bootloader.asm
	@mkdir -p $(BUILD_DIR)
	$(NASM) -f bin -I$(BOOTLOADER_DIR)/ $< -o $@

$(KERNEL_BIN): $(RUST_DIR)/kernel/src/main.rs $(RUST_DIR)/Cargo.toml $(RUST_DIR)/.cargo/config.toml
	cd $(RUST_DIR) && $(CARGO) build
	cp $(RUST_DIR)/target/i686-lindow/debug/kernel $@

$(DISK_IMG): $(BOOT_BIN) $(KERNEL_BIN)
	@mkdir -p $(BUILD_DIR)
	cat $(BOOT_BIN) $(KERNEL_BIN) > $(DISK_IMG)
	truncate -s 8192 $(DISK_IMG)

run: build
	$(QEMU) -drive format=raw,file=$(DISK_IMG) -display none -serial stdio

clean:
	rm -rf $(BUILD_DIR)
