// --- Boot section for Raspberry Pi 4 (AArch64) ---
.section ".text.boot"
.global _start // Entry point

_start:
    // Only run on core 0; others wait forever
    mrs     x1, mpidr_el1
    and     x1, x1, #3      // Mask for core ID
    cbz     x1, 2f          // If core 0, continue  
//q:what cbz 2f means. a: // If core ID is 0, branch to label 2
1:  wfe                     // Other cores: wait for event
    b       1b

2:  // Main core continues
    // Set up stack pointer below code
    ldr     x1, =_start
    mov     sp, x1

    // Zero BSS section
    //q: what is the BSS section? a: The BSS (Block Started by Symbol) section is used to hold uninitialized global and static variables in a program. It is typically zeroed out at program startup.
    // ldr is used to load the address of __bss_start in register x1
    // and __bss_size in register w2
    // This is necessary to clear the BSS section before running the main program
    ldr     x1, =__bss_start
    ldr     w2, =__bss_size
3:  cbz     w2, 4f
    str     xzr, [x1], #8   // Store 0, advance pointer
    sub     w2, w2, #1
    cbnz    w2, 3b // 3b means "branch to label 3 if w2 is not zero"
    //q:why sometimes it is f or b? a: f means "forward" and b means "backward" in branch instructions

    // Jump to Rust main()
4:  bl      main
    b       1b              // If main returns, halt
