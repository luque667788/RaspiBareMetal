

// --- Assembler Directives ---

// .section "name" [,"flags"[,%type[,@flag_specific_arguments]]]
//   Syntax: .section "section_name"
//   Functionality: This directive tells the assembler to switch to the specified
//                  section. Subsequent code or data will be assembled into this
//                  section. The linker then uses section names to organize the
//                  final executable image in memory.
//   In this code: .section ".text.boot"
//     - ".text.boot": A custom section name. The ".text" prefix conventionally
//                     indicates executable code. ".boot" suggests this code is
//                     part of the boot sequence. The linker script will typically
//                     place this section at the very beginning of the kernel image.

// .global symbol
//   Syntax: .global symbol_name
//   Functionality: This directive makes `symbol_name` visible to the linker.
//                  A global symbol can be referenced from other object files or
//                  used by the linker as an entry point or a defined address.
//   In this code: .global _start
//     - _start: A label that is declared as a global symbol. In this context,
//               it serves as the primary entry point for program execution.

// label:
//   Syntax: label_name:
//   Functionality: Defines `label_name` as a symbol representing the current
//                  memory address. Labels are used as targets for branch
//                  instructions or to refer to specific locations in code or data.
//   In this code: _start:, 1:, 2:, 3:, 4:
//     - `_start:`: Defines the global entry point.
//     - `1:`, `2:`, `3:`, `4:`: These are local labels. The GNU assembler allows
//       referencing these using `Nf` (forward) and `Nb` (backward), where `N` is
//       the digit of the label. For example, `2f` refers to the next occurrence
//       of label `2:`, and `1b` refers to the most recent previous occurrence
//       of label `1:`.

// --- AArch64 Instructions ---

// mrs <Xt>, <system_register> (Move from System Register to general-purpose register)
//   Syntax: mrs <Xt>, <system_register_name>
//   Functionality: Reads the value from a special-purpose system register
//                  (e.g., control registers, status registers) and stores it into
//                  the specified general-purpose register <Xt> (64-bit).
//   In this code: mrs x1, mpidr_el1
//     - x1: The destination 64-bit general-purpose register.
//     - mpidr_el1: The Multiprocessor Affinity Register (EL1). It provides
//                  information about the processor's affinity (e.g., core ID,
//                  cluster ID) within a multiprocessor system. EL1 refers to
//                  Execution Level 1 (typically the OS kernel level).

// and <Xd>, <Xn>, #<imm> (Bitwise AND immediate)
//   Syntax: and <Xd|SP>, <Xn|SP>, #<bitmask_immediate>
//   Functionality: Performs a bitwise AND operation between the value in the
//                  source register <Xn> (or SP) and an immediate bitmask value.
//                  The result is written to the destination register <Xd> (or SP).
//   In this code: and x1, x1, #3
//     - x1 (dest): The destination register, also the first source operand.
//     - x1 (src): The source register containing the value from mpidr_el1.
//     - #3: An immediate value (binary 0b11). This operation masks the value in
//           x1, keeping only its lower two bits. For mpidr_el1, these bits
//           typically represent the core ID within a processor cluster (Aff0).

// cbz <Rt>, <label> (Compare and Branch on Zero)
//   Syntax: cbz <Wt|Xt>, <label>
//   Functionality: Compares the value in the general-purpose register <Rt>
//                  (Wt for 32-bit, Xt for 64-bit) with zero. If the value is
//                  zero, it branches to the specified <label>. Otherwise,
//                  execution continues with the next instruction.
//   In this code: cbz x1, 2f
//     - x1: The 64-bit register to test (containing the core ID).
//     - 2f: The target label if x1 is zero. `2f` means "forward to the next
//           label named `2:`".

// wfe (Wait For Event)
//   Syntax: wfe
//   Functionality: This instruction puts the core into a low-power state,
//                  suspending execution until a wakeup event is signaled. Such
//                  events can include interrupts or explicit event signals from
//                  other cores (e.g., via an SEV instruction). If the internal
//                  event register of the core is already set, `wfe` clears it
//                  and continues execution without sleeping.
//   In this code: 1: wfe
//     - Used here to make non-primary cores wait indefinitely in a low-power state.

// b <label> (Branch)
//   Syntax: b <label>
//   Functionality: Causes an unconditional branch to the specified <label>.
//                  Execution continues from the address of the label.
//   In this code: b 1b
//     - 1b: The target label. `1b` means "backward to the most recent previous
//           label named `1:`". This creates an infinite loop (`1: wfe// b 1b//`).

// ldr <Rt>, =<literal> (Load Register - pseudo-instruction)
//   Syntax: ldr <Wt|Xt>, =<value_or_symbol>
//   Functionality: This is a pseudo-instruction that loads a register with either
//                  a constant value or an address. The assembler translates it
//                  into one or more actual instructions, often involving a
//                  PC-relative load from a literal pool if the value is an address
//                  or a large constant that cannot be encoded directly into a
//                  `mov` instruction.
//   In this code: ldr x1, =_start
//     - x1: The destination 64-bit register.
//     - =_start: Loads the memory address of the label `_start` into x1.
//   And: ldr x1, =__bss_start
//     - x1: The destination 64-bit register.
//     - =__bss_start: Loads the memory address of `__bss_start` (start of BSS section) into x1.
//   And: ldr w2, =__bss_size
//     - w2: The destination 32-bit register.
//     - =__bss_size: Loads the value of `__bss_size` into w2. In this context,
//                    `__bss_size` is interpreted as the number of 8-byte (64-bit)
//                    words in the BSS section that need to be cleared.

// mov <Rd>, <Rn> (Move register)
//   Syntax: mov <Wd|SP>, <Wn|SP>  OR  mov <Xd|SP>, <Xn|SP>
//   Functionality: Copies the value from the source register <Rn> to the
//                  destination register <Rd>. This is an alias for an `ORR`
//                  (Bitwise OR) instruction: `ORR <Rd>, <ZR>, <Rn>`, where <ZR>
//                  is the zero register (WZR or XZR).
//   In this code: mov sp, x1
//     - sp: The destination register, which is the Stack Pointer (SP).
//     - x1: The source 64-bit register, containing the address where the stack
//           should start (in this case, the address of `_start`).

// str <Rt>, [<Xn|SP>], #<simm> (Store Register - post-index addressing)
//   Syntax: str <Wt|Xt>, [<Xn|SP>], #<signed_immediate_offset>
//   Functionality: Stores the value from register <Rt> to the memory location
//                  addressed by the value in register <Xn> (or SP). After the
//                  store operation, the address in <Xn> (or SP) is updated by
//                  adding the <signed_immediate_offset> to it.
//   In this code: str xzr, [x1], #8
//     - xzr: The source 64-bit register (Zero Register). It always reads as 0.
//     - [x1]: The memory address for the store is taken from register x1.
//     - #8: The post-increment offset. After storing the 64-bit zero value from
//           xzr to the memory location `[x1]`, the value in x1 is incremented by 8.
//           This effectively clears 8 bytes of memory and advances the pointer.

// sub <Rd>, <Rn>, #<imm>{, <shift>} (Subtract immediate)
//   Syntax: sub <Wd|SP>, <Wn|SP>, #<unsigned_immediate>{, <shift_LSL_0_or_12>}
//           sub <Xd|SP>, <Xn|SP>, #<unsigned_immediate>{, <shift_LSL_0_or_12>}
//   Functionality: Subtracts an optionally shifted immediate value from the value
//                  in register <Rn> and stores the result in register <Rd>.
//   In this code: sub w2, w2, #1
//     - w2 (dest): The destination 32-bit register, also the first source operand.
//     - w2 (src): The source register (containing the BSS size counter).
//     - #1: The immediate value to subtract. This decrements the loop counter.

// cbnz <Rt>, <label> (Compare and Branch on Non-Zero)
//   Syntax: cbnz <Wt|Xt>, <label>
//   Functionality: Compares the value in the general-purpose register <Rt>
//                  (Wt for 32-bit, Xt for 64-bit) with zero. If the value is
//                  non-zero, it branches to the specified <label>. Otherwise,
//                  execution continues with the next instruction.
//   In this code: cbnz w2, 3b
//     - w2: The 32-bit register to test (the BSS size counter).
//     - 3b: The target label if w2 is non-zero. `3b` means "backward to the
//           most recent previous label named `3:`".

// bl <label> (Branch with Link)
//   Syntax: bl <label>
//   Functionality: Performs a subroutine call. It branches to the specified
//                  <label> and stores the address of the instruction immediately
//                  following the `bl` into the Link Register (LR, which is x30).
//                  The LR holds the return address for the subroutine.
//   In this code: bl main
//     - main: The label of the subroutine to call. This is expected to be the
//             entry point of the C `main()` function.
.section ".text.boot"  // Make sure the linker puts this at the start of the kernel image

.global _start  // Execution starts here
// this is the entry point label that can be referenced by the linker or other files
// this line make the _start label visible to the linker


// the actual start label is here
_start:
    // Check processor ID is zero (executing on main core), else hang
    mrs     x1, mpidr_el1
    and     x1, x1, #3 //#3 in binary is 0b11, which masks (saves) the lower two bits of the processor ID
    cbz     x1, 2f // 2f means "forward to the next label named `2:`" if x1 is zero (indicating we're on the main core)
    // We're not on the main core, so hang in an infinite wait loop
1:  wfe
    b       1b // b means branch, 1b means "backward to the most recent previous label named `1:`"
    // Infinite loop to wait for an event (e.g., interrupt) to wake up the core

2:  // We're on the main core! the jump target label `2:` is here

    // Set stack to start below our code
    ldr     x1, =_start
    mov     sp, x1 // q: why not mov sp _start? a: because the assembler will not know the address of _start until it is linked, so we use ldr to load the address into x1 first
    // remeber stack grows down


    //q: what is the bss section?
    // a: The BSS (Block Started by Symbol) section is used in C/C++ programs to hold 
    //    uninitialized global and static variables. It is a part of the program's
    //    memory layout that is reserved for these variables, and it is initialized to zero
    //    by the operating system when the program starts. The BSS section is typically
    //    located in the program's data segment, and it helps reduce the size of the executable
    //    by not requiring space for uninitialized variables in the binary file. 


    // Clean the BSS section
    ldr     x1, =__bss_start     // Start address
    ldr     w2, =__bss_size      // Size of the section
3:  cbz     w2, 4f               // Quit loop if zero

// this is the loop to clear the BSS section
    // Clear 8 bytes at a time
    str     xzr, [x1], #8 // Store zero to the address in x1, then increment x1 by 8 bytes
    sub     w2, w2, #1 // Decrement the size counter
    // w2 is the size of the BSS section in 8-byte words, so we decrement it by 1
    cbnz    w2, 3b               // Loop if non-zero

    // Jump to our main() routine in C (make sure it doesn't return)
4:  bl      main
    // In case it does return, halt the master core too
    b       1b
