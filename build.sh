#!/bin/bash
# Build script to compile all components and merge binaries

# Build all components
echo "Building bootloader..."
cargo build --release --bin boot

echo "Building loader..."
cargo build --release --bin loader

echo "Building updater..."
cargo build --release --bin updater

echo "Building application..."
cargo build --release --bin application

# Get binary locations
TARGET_DIR="target/thumbv7em-none-eabihf/release"
BOOT_BIN="${TARGET_DIR}/boot.bin"
LOADER_BIN="${TARGET_DIR}/loader.bin"
UPDATER_BIN="${TARGET_DIR}/updater.bin"
APP_BIN="${TARGET_DIR}/application.bin"

# Convert ELF to binary using objcopy
arm-none-eabi-objcopy -O binary ${TARGET_DIR}/boot ${BOOT_BIN}
arm-none-eabi-objcopy -O binary ${TARGET_DIR}/loader ${LOADER_BIN}
arm-none-eabi-objcopy -O binary ${TARGET_DIR}/updater ${UPDATER_BIN}
arm-none-eabi-objcopy -O binary ${TARGET_DIR}/application ${APP_BIN}

# Patch headers (using the existing Python script)
echo "Patching headers..."
python scripts/patch_image_header.py ${BOOT_BIN} --skip-header
python scripts/patch_image_header.py ${LOADER_BIN}
python scripts/patch_image_header.py ${UPDATER_BIN}
python scripts/patch_image_header.py ${APP_BIN}

# Merge binaries
echo "Merging binaries..."
python scripts/merge_images.py ${BOOT_BIN} ${LOADER_BIN} ${UPDATER_BIN} ${APP_BIN}

echo "Build complete! Final firmware is merged_firmware.bin"