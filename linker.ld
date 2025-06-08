/* ENTRY directive:
    Specifies the entry point of the program. 
    Execution will begin at the symbol `_start`. */
ENTRY(_start)

/* MEMORY directive:
    Defines memory regions available to the linker. */
MEMORY
{
     /* Define a memory region named RAM.
         ORIGIN: Specifies the starting address of the RAM region (0x80000). it is the base address where the program will be loaded on RAM (see bootloader datasheet)
         LENGTH: Specifies the size of the RAM region (128 Megabytes). */
     RAM : ORIGIN = 0x80000, LENGTH = 128M
}

/* SECTIONS directive:
    Defines how input sections from object files are mapped into output sections in the final executable. */
SECTIONS
{
     /* Set the current location counter ('.') to 0x80000. 
         This means the first section will be placed at this address. */
     . = 0x80000; 

     
     /* Define an output section named .text.boot. */
     .text.boot : {
          /* KEEP directive:
              Ensures that all input sections matching the pattern *(.text.boot) are included,
              even if they are not referenced. This is used for startup code.
              text.boot assembly code */

          KEEP(*(.text.boot))
     } > RAM /* Place this section in the RAM memory region. */
     
     /* Define an output section named .text. */
     .text : {
          /* Include all input sections whose names start with .text (e.g., .text, .text.foo). */
          /*in our particular case we have a .text section for the main function in rust */
          *(.text*)
     } > RAM /* Place this section in the RAM memory region. */
     
     /* Define an output section named .rodata (read-only data). */
     .rodata : {
          /* Include all input sections whose names start with .rodata. */
          *(.rodata*)
     } > RAM /* Place this section in the RAM memory region. */
     
     /* Define an output section named .data (initialized data). */
     /* this is for the global variables that are initialized */
     .data : {
          /* Include all input sections whose names start with .data. */
          *(.data*)
     } > RAM /* Place this section in the RAM memory region. */
     
     /* Define an output section named .bss (uninitialized data). */
     .bss : {
          /* Define a symbol `__bss_start` to mark the beginning of the .bss section.
              '.' refers to the current location counter. */
          __bss_start = .;
          /* Include all input sections whose names start with .bss. */
          *(.bss*)
          /* Include all common symbols. Common symbols are uninitialized global variables
              that are not explicitly placed in .bss by the compiler. */
          *(COMMON)
          /* Align the current location counter to the next 8-byte boundary.
              This ensures that the .bss section (or the data following it) is properly aligned. */
          . = ALIGN(8);
          /* Define a symbol `__bss_end` to mark the end of the .bss section. */
          __bss_end = .;
     } > RAM /* Place this section in the RAM memory region. */
     
     /* Define a symbol `__bss_size` to calculate the size of the .bss section in 64-bit words (8 bytes).
         This is calculated as the difference between the end and start addresses, divided by 8. */
     __bss_size = (__bss_end - __bss_start) / 8;
     
     /* /DISCARD/ directive:
         Specifies sections that should be discarded and not included in the final output file. */
     /DISCARD/ : {
          /* Discard any input sections named .comment. */
          *(.comment)
          /* Discard any input sections whose names start with .gnu. */
          *(.gnu*)
          /* Discard any input sections whose names start with .note. */
          *(.note*)
          /* Discard any input sections whose names start with .eh_frame (exception handling frames). */
          *(.eh_frame*)
     }
}