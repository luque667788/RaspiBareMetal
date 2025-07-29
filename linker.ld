/* Entry point for the program */
ENTRY(_start)

/* Define RAM region: start at 0x80000, size 128MB */
MEMORY { RAM : ORIGIN = 0x80000, LENGTH = 128M }

SECTIONS {
     . = 0x80000; /* Place everything at 0x80000 */

     /* Startup code (always keep) */
     .text.boot : { KEEP(*(.text.boot)) } > RAM
    /*why > RAM? a: To place the startup code in the RAM region for execution.*/

     /* Main code */
     .text : { *(.text*) } > RAM

     /* Read-only data */
     .rodata : { *(.rodata*) } > RAM

     /* Initialized data */
     .data : { *(.data*) } > RAM

    /* Uninitialized data (BSS) section.
       This section holds uninitialized global and static variables.
       The linker sets __bss_start to the start address of the BSS,
       and __bss_end to the end address (after aligning to 8 bytes).
       The BSS size is calculated in 8-byte units for zeroing in the startup code.
    */
    .bss : {
        __bss_start = .;    /* Mark start of BSS section */
        *(.bss*)            /* Place all .bss input sections here */
        *(COMMON)           /* Place COMMON symbols (uninitialized data) here */
        . = ALIGN(8);       /* Align end of BSS to 8 bytes */
        __bss_end = .;      /* Mark end of BSS section */
    } > RAM
    __bss_size = (__bss_end - __bss_start) / 8; /* Size in 8-byte units */

     /* Discard unneeded sections */
     /DISCARD/ : { *(.comment) *(.gnu*) *(.note*) *(.eh_frame*) }
}