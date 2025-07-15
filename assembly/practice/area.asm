section .data
    pi dw 3142      ; fixed-point π (3.14 * 100)
    r db 5         ; radius

section .text
    global _start

_start:
    ; Step 1: r^2
    movzx rax, byte [rel r]    ; r = 5
    mul rax                    ; r * r → AX = 25

    ; Step 2: r² * π
    movzx rax, ax              ; move 25 into RAX (zero-extended)
    mul word [rel pi]         ; 25 * 314 = 7850

    ; Step 3: Scale down to compensate for *100 in pi
    mov rbx, 100
    ;xor rdx, rdx               ; clear remainder
    mov rdx,0                   ; same effect as xor rdx, rdx

    div rbx                    ; rax = 7850 / 100 = 78

    ; Step 4: syscall to exit with result
    mov edi, eax               ; exit code
    mov rax, 0x2000001         ; syscall: exit
    syscall
