; add_prog.asm

global _start

section .text

_start:
    mov 	rdi, 10
    mov 	rsi, 32
    call 	add

    mov 	rdi, rax
    mov		rax, 60
    syscall

add:
    mov 	rax, rdi
    add 	rax, rsi
    ret
