OUTPUT_FORMAT(elf64-x86-64)
OUTPUT_ARCH(i386:x86-64)
ENTRY(krnl_main)

KERNEL_VMA = 0xffffffff80000000;
KERNEL_LMA = 0x00100000;
PAGE_SIZE = 0x1000;

PHDRS
{
    text PT_LOAD FLAGS(5);
    rodata PT_LOAD FLAGS(4);
    data PT_LOAD FLAGS(6);
}

SECTIONS
{
    . = KERNEL_VMA + KERNEL_LMA;
    __kernel_virtual_start = .;

    .text : AT(ADDR(.text) - KERNEL_VMA)
    {
        KEEP(*(.text.boot))
        *(.text .text.*)
    } :text

    . = ALIGN(PAGE_SIZE);
    .rodata : AT(ADDR(.rodata) - KERNEL_VMA)
    {
        *(.rodata .rodata.*)
        *(.eh_frame .eh_frame.*)
    } :rodata

    . = ALIGN(PAGE_SIZE);
    .data : AT(ADDR(.data) - KERNEL_VMA)
    {
        *(.data .data.*)
        *(.got .got.*)
        *(.init_array .init_array.*)
    } :data

    . = ALIGN(PAGE_SIZE);
    .bss (NOLOAD) : AT(ADDR(.bss) - KERNEL_VMA)
    {
        __bss_start = .;
        *(COMMON)
        *(.bss .bss.*)
        __bss_end = .;
    } :data

    . = ALIGN(PAGE_SIZE);
    __kernel_virtual_end = .;
    __kernel_physical_start = __kernel_virtual_start - KERNEL_VMA;
    __kernel_physical_end = __kernel_virtual_end - KERNEL_VMA;

    /DISCARD/ :
    {
        *(.comment)
        *(.note .note.*)
    }
}
