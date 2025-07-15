.global _start
.extern stack_top

.section ".text.boot"

_start:
    ; get the core_id
    mrs x0, mpidr_el1
    and x0, x0, #7

    move x19, x0 ;save core_id

    ldr   x30       , =stack_top      //; stack top pointer
    mov   sp        , x30             //; transfer
    ; mrs   x5        , CurrentEL       //; current exception level moves to x5
    ; ubfx  x5, x5, #2, #2
    ; cmp   x5        , 3         //; check if its el3
    ; b.eq  el3_entry                   //; if yes，enter el3_entry
    ; b     el2_entry                   //; otherwise，enter el2_entry

    mov x0, sp
    bl     init                        //; never return
    b     .                           //; never run here
