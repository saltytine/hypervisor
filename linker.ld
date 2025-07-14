ENTRY(_start)
SECTIONS
{
    . = 0x40000000; /* QEMU physical memory address, file loading address */
    .text.boot : {*(.text.boot)} /* specify the code segment .text of the start.o file after start.s is compiled and output as the .startup segmen */
    .text : {*(.text)}
    .data : {*(.data)}
    .rodata : {*(.rodata)}
    .bss : {*(.bss)}

    . = ALIGN(8); /* 8 byte alignment, stack below */

    . = . + 0x4000; /* the size of the stack is 0x4000=16KB */
    stack_top = .; /* corresponding to stack_top in start.s */
}
