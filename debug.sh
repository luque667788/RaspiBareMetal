#!/bin/bash

# Colors for better readability
BLUE='\033[0;34m'
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[0;33m'
NC='\033[0m' # No Color

# Configuration
# TODO! change this to release
ELF_FILE="target/aarch64-unknown-none/debug/rpi4-baremetal"
HOST="localhost"
PORT="1234"

echo -e "${BLUE}=== ARM64 Kernel Debugger ===${NC}"

# Verify QEMU is running
if ! nc -z $HOST $PORT &>/dev/null; then
    echo -e "${RED}Error: No QEMU instance detected on $HOST:$PORT${NC}"
    echo -e "${YELLOW}Make sure to run 'make docker-qemu-debug' first in another terminal${NC}"
    echo -e "${YELLOW}Command: make docker-qemu-debug${NC}"
    exit 1
fi

echo -e "${GREEN}✓ QEMU detected on $HOST:$PORT${NC}"
echo -e "${GREEN}Loading symbols from $ELF_FILE${NC}"

# Check if file exists
if [ ! -f "$ELF_FILE" ]; then
    echo -e "${RED}Error: ELF file not found. Build the project first.${NC}"
    echo -e "${YELLOW}Command: make all${NC}"
    exit 1
fi

# Use gdb-multiarch if available, otherwise use gdb
if command -v gdb-multiarch &> /dev/null; then
    GDB_CMD="gdb-multiarch"
else
    GDB_CMD="gdb"
fi

# Create a temporary GDB init file
TMP_GDBINIT=$(mktemp)
cat > "$TMP_GDBINIT" << EOF
set architecture aarch64
file $ELF_FILE
set tcp connect-timeout 10
target remote $HOST:$PORT
break main
focus cmd
EOF

# Run GDB with the temporary init file
$GDB_CMD -x "$TMP_GDBINIT"

# Clean up
rm -f "$TMP_GDBINIT"