OUTPUT_ARCH(riscv)
ENTRY(_start)
BASE_ADDR = 0x80400000;

SECTIONS {
    . = BASE_ADDR;

    .text : {
        *(.text.entry)
        *(.text .text.*)
        *(.stext .stext.*)
    }

    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    }

    .data : {
        *(.data .data.*)
        *(.sdata .sdata.*)
    }

    .bss : {
        sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
        ebss = .;
    }

    /DISCARD/ : {
        *(.eh_frame)
        *(.debug*)
    }
}