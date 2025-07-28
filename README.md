# Raspberry Pi 4 Baremetal Hello World

This project is a simple exploration of baremetal programming on the Raspberry Pi 4 Model B. It demonstrates how to set up the boot process, configure the linker, and use Rust for low-level ARM development. The main goals are to:

- Blink the ACT LED as a basic hardware test
- Enable UART communication for serial output (so you can use `println!` and see output via a USB-TTL adapter)
- Provide a minimal codebase for further future baremetal experiments (interrupts, memory management, task handling, etc.)

## Project Structure

- **src/boot.S**: Assembly code for the very first boot steps. Sets up the stack pointer, clears the BSS section, and jumps to Rust's entry point.
- **linker.ld**: Custom linker script to place code and data at the correct addresses for the Pi's memory map.
- **src/main.rs**: Main Rust code. Handles board initialization, LED blinking, and UART output.
- **src/hal/registers/**: Register definitions for GPIO, UART, and auxiliary peripherals, organized as Rust structs for safe access.
- **src/drivers/uart/**: Modular UART drivers for both Mini UART and PL011 UART (UART0), with clear comments and usage examples.
- **src/log.rs**: Implements a custom `print!` and `println!` macro for serial output over UART, so you can easily print debug/info messages from your baremetal code.
- **Makefile**: Build system for cross-compiling, running in QEMU, and Docker support.
- **Dockerfile**: For building and running the project in a containerized environment.
- **debug.sh**: Script to help set up remote GDB debugging with QEMU.

## Boot Process Explained

1. **Boot ROM**: The Pi's GPU loads `kernel8.img` from the SD card into RAM at address `0x80000` and jumps to it in EL2 (hypervisor mode).
2. **Assembly Startup (`boot.S`)**:
   - Sets up the stack pointer for each core (only core 0 continues; others are parked).
   - Clears the BSS section (uninitialized data) to zero.
   - Jumps to the Rust entry point (`_start` in `main.rs`).
3. **Rust Initialization**:
   - Initializes peripherals (GPIO, UART, etc.).
   - Blinks the ACT LED as a hardware check.
   - Sets up UART for serial output, so you can see logs and interact with the board.

## UART and GPIO Setup

- **GPIO**: The General Purpose Input/Output (GPIO) pins are configured by writing to the `GPFSEL` registers. For UART:
  - GPIO14 (TX) and GPIO15 (RX) are set to the correct alternate function (ALT0 for PL011 UART0, ALT5 for Mini UART).
  - This is done by clearing and then setting the appropriate bits in `GPFSEL1`.
- **UART**:
  - The project supports both Mini UART and PL011 UART (UART0). Each has its own driver module.
  - The drivers set the baud rate, enable FIFOs, and configure the UART for 8N1 (8 data bits, no parity, 1 stop bit).
  - The Mini UART is enabled via the AUX peripheral, while UART0 is enabled directly.
  - You can use `write_string`, `read_byte`, etc., for serial communication.

## Building and Running

- **Build**: `make` or `make all` (cross-compiles Rust code and creates `kernel8.img`)
- **Run in QEMU**: `make run` or `make qemu` (emulates the Pi and shows serial output)
- **Debug**: `make debug` shows information about the ELF binary (sections, symbols, disassembly). To debug with GDB, first start QEMU in debug mode (`make qemu-debug` or `make docker-qemu-debug`), then run `./debug.sh` in another terminal to connect GDB to the running instance.
- **Clean**: `make clean` removes build artifacts and the kernel image.
- **Docker**:
    - `make docker-build`: Builds the Docker image for cross-compiling and emulation.
    - `make docker-compile`: Builds the project inside a Docker container (cross-compiles and produces `kernel8.img`), but does not run QEMU.
    - `make docker-qemu`: Runs QEMU inside a Docker container to emulate the Pi and show serial output, using the previously built kernel image.
    - `make docker-qemu-debug`: Runs QEMU in debug mode inside Docker, exposing GDB and QEMU monitor ports for remote debugging.
    - `make docker-shell`: Opens an interactive shell inside the Docker container for manual builds or troubleshooting.

All of these are implemented as `.PHONY` targets in the Makefile, so they always run when invoked, regardless of file timestamps. The `run` target is a simple alias for `qemu` for convenience.

## Notes

- Comments and documentation were written with the help of AI coding assistants (GitHub Copilot for Students).