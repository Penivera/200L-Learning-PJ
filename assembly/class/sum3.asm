section .data
    number1 db 3
    number2 db 2
    number3 db 9

section .text
    global _start

_start:
    movzx rax, byte [rel number1]
    add al, [rel number2]
    add al, [rel number3]

    ; Exit with sum as return code
    mov rdi, rax         ; exit code
    mov rax, 0x2000001   ; syscall: exit
    syscall
