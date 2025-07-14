.global _start
.extern stack_top

.section ".text.boot"

_start:
    ldr  x30     , =stack_top //; stack top pointer
    mov  sp      , x30        //; transfer
    bl   not_main             //; jump
    b    .                    //; it will never run here
