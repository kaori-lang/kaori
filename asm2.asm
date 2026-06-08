push rbp
sub rsp, 0x50
mov r10, rdi
mov r8, r14
movzx r11d, byte [r12 + 0x1]
movzx ecx, byte [r12 + 0x2]
shl ecx, 0x4
movzx edx, byte [r13 + rcx * 1]
add r12, 0x8
lea rax, qword [rcx + r13 * 1]
inc rax
mov rcx, qword [r13 + rcx * 1 + 0x8]
cmp edx, 0x4
jz 0x66b2c
cmp edx, 0x5
jnz 0x66ce2
mov rsi, r10
shl rsi, 0x4
mov byte [r13 + rsi * 1], 0x5
mov edx, dword [rax]
movzx edi, word [rax + 0x4]
movzx eax, byte [rax + 0x6]
mov byte [r13 + rsi * 1 + 0x7], al
mov word [r13 + rsi * 1 + 0x5], di
mov dword [r13 + rsi * 1 + 0x1], edx
mov qword [r13 + rsi * 1 + 0x8], rcx
mov rdx, qword [r15 + 0x10]
cmp rcx, rdx
jnb 0x66d0a
add rsi, r13
shl rcx, 0x6
mov rax, qword [r15 + 0x8]
mov rbp, qword [r15 + 0x28]
lea rbx, qword [rax + rcx * 1]
mov r14, qword [rax + rcx * 1 + 0x20]
mov rdi, qword [rax + rcx * 1 + 0x30]
cmp rbp, qword [r15 + 0x18]
jz 0x66c92
mov rax, qword [r15 + 0x20]
lea rcx, qword [rbp * 4]
add rcx, rbp
mov qword [rax + rcx * 8], r12
mov qword [rax + rcx * 8 + 0x8], r13
mov qword [rax + rcx * 8 + 0x10], r8
mov qword [rax + rcx * 8 + 0x18], r10
mov byte [rax + rcx * 8 + 0x20], r11b
inc rbp
mov qword [r15 + 0x28], rbp
mov r12, qword [rbx + 0x8]
movzx eax, byte [r12]
lea rcx, qword [rip + 0x22ce8]
mov r13, rsi
add rsp, 0x50
pop rbp
jmp qword [rcx + rax * 8]
mov rsi, r10
shl rsi, 0x4
mov byte [r13 + rsi * 1], 0x4
mov edx, dword [rax]
movzx edi, word [rax + 0x4]
movzx eax, byte [rax + 0x6]
mov byte [r13 + rsi * 1 + 0x7], al
mov word [r13 + rsi * 1 + 0x5], di
mov dword [r13 + rsi * 1 + 0x1], edx
mov qword [r13 + rsi * 1 + 0x8], rcx
mov rdx, qword [r15 + 0x68]
lea rcx, qword [rcx + rcx * 2]
mov rax, qword [rdx + rcx * 8 + 0x10]
test rax, rax
jz 0x66cfa
mov r9, qword [rdx + rcx * 8 + 0x8]
mov rcx, qword [r9 + 0x8]
mov rdx, qword [r15 + 0x10]
cmp rcx, rdx
jnb 0x66d16
mov r14, r8
add rsi, r13
mov rdi, qword [r15 + 0x8]
shl rcx, 0x6
lea rbx, qword [rdi + rcx * 1]
mov rdx, qword [rdi + rcx * 1 + 0x30]
mov qword [rsp + 0x38], rdx
cmp rax, 0x1
jz 0x66bb0
movzx edx, byte [r9 + 0x10]
cmp dl, 0xa
jnz 0x66c0d
mov rbp, qword [rbx + 0x20]
mov rdi, qword [r15 + 0x28]
cmp rdi, qword [r15 + 0x18]
jz 0x66cbf
mov rax, qword [r15 + 0x20]
lea rcx, qword [rdi + rdi * 4]
mov qword [rax + rcx * 8], r12
mov qword [rax + rcx * 8 + 0x8], r13
mov qword [rax + rcx * 8 + 0x10], r14
mov qword [rax + rcx * 8 + 0x18], r10
mov byte [rax + rcx * 8 + 0x20], r11b
inc rdi
mov qword [r15 + 0x28], rdi
mov r12, qword [rbx + 0x8]
movzx eax, byte [r12]
lea rcx, qword [rip + 0x22c0f]
mov r13, rsi
mov r14, rbp
mov rdi, qword [rsp + 0x38]
add rsp, 0x50
pop rbp
jmp qword [rcx + rax * 8]
mov rcx, qword [rdi + rcx * 1 + 0x38]
lea r8, qword [r9 + 0x20]
mov rdi, qword [r9 + 0x11]
mov r9, qword [r9 + 0x18]
mov qword [rsp + 0x47], r9
mov qword [rsp + 0x40], rdi
inc rcx
shl rax, 0x4
add rax, -0x20
nop word [rax + rax * 1]
movzx edi, cl
shl edi, 0x4
mov byte [rsi + rdi * 1], dl
mov rdx, qword [rsp + 0x40]
mov r9, qword [rsp + 0x47]
mov qword [rsi + rdi * 1 + 0x1], rdx
mov qword [rsi + rdi * 1 + 0x8], r9
test rax, rax
jz 0x66bb0
movzx edx, byte [r8]
cmp dl, 0xa
jz 0x66bb0
mov rdi, qword [r8 + 0x1]
mov r9, qword [r8 + 0x8]
add r8, 0x10
mov qword [rsp + 0x47], r9
mov qword [rsp + 0x40], rdi
inc rcx
add rax, -0x10
jmp 0x66c40
lea rcx, qword [r15 + 0x18]
mov qword [rsp + 0x30], r10
mov byte [rsp + 0x2f], r11b
mov qword [rsp + 0x38], r8
call 0x7b1b0
mov r8, qword [rsp + 0x38]
movzx r11d, byte [rsp + 0x2f]
mov r10, qword [rsp + 0x30]
jmp 0x66ae2
lea rcx, qword [r15 + 0x18]
mov qword [rsp + 0x30], r10
mov byte [rsp + 0x2f], r11b
call 0x7b1b0
movzx r11d, byte [rsp + 0x2f]
mov r10, qword [rsp + 0x30]
jmp 0x66bc2
lea rcx, qword [rip + 0x228ff]
mov edx, 0x17
call 0x7db10
nop
add rsp, 0x50
pop rbp
ret
lea r8, qword [rip + 0x2289f]
xor ecx, ecx
xor edx, edx
call 0x7b898
lea r8, qword [rip + 0x228bf]
call 0x7b898
lea r8, qword [rip + 0x2289b]
call 0x7b898
int 0x3