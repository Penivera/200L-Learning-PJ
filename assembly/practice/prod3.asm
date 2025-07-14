;multiplying 3 numbers
section .data
    number1 db 3
    number2 db 4
    number3 db 7

section .text
    global _start

_start:
    movzx rax, byte [rel number1]
    mul byte [rel number2]
    movzx rax,al
    mul byte [rel number3]
    mov edi,eax ;NOTE - exit code can move rdi,ax (movzx rdi,ax) because it'd be moving the last. 16 bit to the rdi 
    mov rax,0x2000001 ;syscall for exit
    syscall