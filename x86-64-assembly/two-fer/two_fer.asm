default rel

section .rodata
first: db "One for ", 0
name: db "you", 0
second: db ", one for me.", 0

section .text
global two_fer
two_fer:
    ; Args
    ; name -- rdi
    ; buffer -- rsi
    mov r11, rdi ; name
    mov rdi, rsi

    ; Copy the first part of the sentence
    mov rsi, first
    call strcpy
    mov rdi, rax

    ; Copy the name of the sentence
    ; Load name constant if provided name pointer is NULL
    mov rsi, r11
    cmp rsi, 0
    jne cpy
    mov rsi, name

    cpy: call strcpy
    mov rdi, rax

    ; Copy the second part of the sentence
    mov rsi, second
    call strcpy
    mov rdi, rax

    ; Write trailing 0 byte
    mov byte [rdi], 0

    ret

strcpy:
    ; Args
    ; dst -- rdi
    ; src -- rsi
    ;
    ; Returns
    ; dst advanced by the length of the copied bytes

    loop:
    mov al, byte [rsi]
    cmp al, 0
    je end
    mov [rdi], al
    inc rdi
    inc rsi
    jmp loop

    end:
    mov rax, rdi

    ret
