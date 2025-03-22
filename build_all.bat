@echo off
echo Building boot...
cd boot && cargo build && cd ..

echo Building loader...
cd loader && cargo build && cd ..

echo Building updater...
cd updater && cargo build && cd ..

echo Building application...
cd application && cargo build && cd ..

echo All components built successfully!

rem Преобразование в бинарные файлы с помощью objcopy
echo Creating binary files...
arm-none-eabi-objcopy -O binary target/thumbv7em-none-eabihf/debug/boot boot.bin
arm-none-eabi-objcopy -O binary target/thumbv7em-none-eabihf/debug/loader loader.bin
arm-none-eabi-objcopy -O binary target/thumbv7em-none-eabihf/debug/updater updater.bin
arm-none-eabi-objcopy -O binary target/thumbv7em-none-eabihf/debug/application application.bin

rem Объединение бинарных файлов
echo Merging binary files...
python scripts/merge_images.py boot.bin loader.bin updater.bin application.bin

probe-run --chip STM32F407VGTx --download-file merged_firmware.bin

echo Firmware flashed successfully!