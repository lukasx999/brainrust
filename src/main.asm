    global _start

    section .text
_start:

    ; crate stack frame
    push    rbp
    mov     rbp, rsp

    mov BYTE [rbp-1], 65

    .loop_1:

    ; read()
    mov rax, 0
    mov rdi, 0
    lea rsi, [rbp-1]
    mov rdx, 1
    syscall

    ; write()
    mov rax, 1
    mov rdi, 1
    lea rsi, [rbp-1]
    mov rdx, 1
    syscall

    cmp BYTE [rbp-1], 120
    jne .loop_1

    ; destroy stack frame
    pop rbp

    ; exit(45)
    mov rax, 60
    mov rdi, 45
    syscall
