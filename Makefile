# Makefile Explanation:
# A Makefile consists of rules and variables.
# Rules define how to build a target file from prerequisite files.
# The basic syntax of a rule is:
# target: prerequisites
#   command1
#   command2
#   ...
# - 'target' is usually the name of a file to be generated.
# - 'prerequisites' (also called dependencies) are files used as input to create the target.
# - 'commands' are shell commands that create the target. They MUST be preceded by a TAB character.
#
# Variables are defined using '=':
# VARIABLE_NAME = value
# They are referenced using $(VARIABLE_NAME) or ${VARIABLE_NAME}.

# --- Variable Definitions ---
# These variables define names and paths used throughout the Makefile.

# TARGET: Specifies the Rust compilation target triple for AArch64 bare-metal.
TARGET = aarch64-unknown-none
# KERNEL: The name of the final kernel image file.
KERNEL = kernel8.img
# ELF: Path to the compiled ELF (Executable and Linkable Format) file.
# $(TARGET) here expands the TARGET variable defined above.
#TODO! change this to release
ELF = target/$(TARGET)/debug/rpi4-baremetal

# Tools: Variables for cross-compilation toolchain commands.
# OBJCOPY: Used to copy and translate object files. Here, to convert ELF to a raw binary.
OBJCOPY = aarch64-linux-gnu-objcopy
# OBJDUMP: Used to display information from object files, like disassembly.
OBJDUMP = aarch64-linux-gnu-objdump
# NM: Used to list symbols from object files.
NM = aarch64-linux-gnu-nm

# --- Phony Targets ---
# .PHONY declares targets that are not actual files.
# This prevents 'make' from getting confused if a file with the same name as a phony target exists.
# It also ensures the commands for these targets run every time they are invoked, regardless of file timestamps.
.PHONY: all clean debug run qemu install-deps docker-build docker-compile docker-shell docker-qemu-debug

#run will just redirect to qemu run




# --- Main Build Rule ---
# 'all' is the default target. When you run 'make' without specifying a target,
# it will try to build the first target in the Makefile that is not a phony target (unless it's explicitly called).
# This rule says that to build 'all', the '$(KERNEL)' file must be built first.
all: $(KERNEL)
run: qemu
# --- Dependency Installation Rule ---
# 'install-deps' is a phony target to set up the development environment.
install-deps:
# Adds the specified Rust target for cross-compilation.
	rustup target add $(TARGET)
# Updates the system's package list. 'sudo' is used for administrative privileges.
	sudo apt update
# Installs the AArch64 GCC cross-compiler and QEMU system emulator for AArch64.
# '-y' automatically confirms prompts.
	sudo apt install -y gcc-aarch64-linux-gnu qemu-system-aarch64

# --- ELF Build Rule ---
# This rule defines how to build the ELF file specified by the $(ELF) variable.
# It depends on several source files: src/main.rs, src/boot.S, linker.ld, build.rs, and Cargo.toml.
# If any of these prerequisite files are newer than the $(ELF) file (or if $(ELF) doesn't exist),
# the commands below will be executed.
$(ELF): src/main.rs src/boot.S linker.ld build.rs Cargo.toml
# Uses Cargo (Rust's build system and package manager) to build the project.
# '--release' builds an optimized version.
# '--target $(TARGET)' specifies the cross-compilation target.
#TODO LATER change this to release build "--release"
	cargo build --target $(TARGET)

# --- Kernel Image Creation Rule ---
# This rule defines how to create the raw kernel binary image '$(KERNEL)'.
# It depends on the ELF file '$(ELF)'. So, 'make' will ensure '$(ELF)' is up-to-date before running these commands.
$(KERNEL): $(ELF)
# Uses $(OBJCOPY) to convert the ELF file to a raw binary format ('-O binary').
# The input is '$(ELF)' and the output is '$(KERNEL)'.
	$(OBJCOPY) -O binary $(ELF) $(KERNEL)
# '@' at the beginning of a command line suppresses the printing of the command itself to the console.
# This command prints a confirmation message.
	@echo "Kernel image created: $(KERNEL)"
# Lists the details of the created kernel image.
	@ls -la $(KERNEL)

# --- Debug Information Rule ---
# 'debug' is a phony target to display debugging information about the ELF file.
# It depends on '$(ELF)'.
debug: $(ELF)
	@echo "=== ELF Sections ==="
# Displays the section headers of the ELF file.
	$(OBJDUMP) -h $(ELF)
	@echo "\n=== Symbol Table ==="
# Lists symbols from the ELF file and filters for specific symbols like 'main', '_start', and '__bss'.
	$(NM) $(ELF) | grep -E '(main|_start|__bss)'
	@echo "\n=== Disassembly of boot section ==="
# Disassembles the '.text.boot' section of the ELF file.
	$(OBJDUMP) -d -j .text.boot $(ELF)

# --- QEMU Execution Rule ---
# 'qemu' is a phony target to run the kernel image in QEMU.
# It depends on '$(KERNEL)'.
qemu: $(KERNEL)
# Runs the QEMU AArch64 system emulator.
# The backslash '\' is used to continue a long command onto the next line.
	qemu-system-aarch64 \
		-machine raspi4b \
		-cpu cortex-a72 \
		-kernel $(KERNEL) \
		-serial stdio \
		-display none \
		-d guest_errors,unimp,mmu
# Emulate a Raspberry Pi 4 Model B.
# Use Cortex-A72 CPU.
# Load the specified kernel image.
# Redirect serial output to standard I/O.
# Disable graphical display.
# Log guest errors. 

# --- QEMU Debug Execution Rule ---
# 'qemu-debug' is a phony target to run the kernel in QEMU with debugging enabled.
# It depends on '$(KERNEL)'.
qemu-debug: $(KERNEL)
	qemu-system-aarch64 \
		-machine raspi4b \
		-cpu cortex-a72 \
		-kernel $(KERNEL) \
		-serial stdio \
		-display none \
		-monitor telnet:127.0.0.1:4444,server,nowait \
		-d guest_errors,int \
		-gdb tcp::1234 \
		-S
# Log guest errors and interrupts.
# Shorthand for -gdb tcp::1234. Use this if you want to connect with gdb.
# Freeze CPU at startup (use 'c' in GDB to continue).             
#Open a GDB server on telnet port 1234. '-S' freezes CPU at startup (waits for GDB).

# --- Clean Rule ---
# 'clean' is a common phony target to remove build artifacts.
clean:
# Runs Cargo's clean command to remove Rust build artifacts (e.g., in the 'target' directory).
	cargo clean
# Removes the kernel image file. '-f' forces removal without prompting and ignores non-existent files.
	rm -f $(KERNEL)

# === Docker Targets ===

# Build Docker image
docker-build:
	docker build -t rpi4-build .

# Build project in Docker
docker-compile: docker-build
	docker run --rm -u $(shell id -u):$(shell id -g) -v $(PWD):/app rpi4-build make all

# Interactive shell in Docker
docker-shell: docker-build
	docker run --rm -it -u $(shell id -u):$(shell id -g) -v $(PWD):/app rpi4-build bash

# Run QEMU in Docker
docker-qemu: docker-build
	docker run --rm --init --sig-proxy=true -u $(shell id -u):$(shell id -g) -v $(PWD):/app -p 1234:1234 rpi4-build make qemu

# Run QEMU with debug in Docker
docker-qemu-debug: docker-build
	docker stop $$(docker ps -q --filter ancestor=rpi4-build --filter publish=1234 --filter publish=4444) >/dev/null 2>&1 || true
	docker rm $$(docker ps -a -q --filter ancestor=rpi4-build --filter publish=1234 --filter publish=4444) >/dev/null 2>&1 || true
	docker run --rm --init --sig-proxy=true -u $(shell id -u):$(shell id -g) -v $(PWD):/app -p 1234:1234 -p 4444:4444 rpi4-build make qemu-debug


# Start a debug session with proper ARM64 configuration
debug-session: $(KERNEL)
	./debug.sh