movzx eax, byte [r12 + 0x1]
shl eax, 0x4
add rax, r13
mov rcx, qword [r15 + 0x28]
test rcx, rcx
jz 0x66d88
dec rcx
mov qword [r15 + 0x28], rcx
mov rdx, qword [r15 + 0x20]
lea rcx, qword [rcx + rcx * 4]
mov r12, qword [rdx + rcx * 8]
mov r13, qword [rdx + rcx * 8 + 0x8]
mov r14, qword [rdx + rcx * 8 + 0x10]
mov rdi, qword [rdx + rcx * 8 + 0x18]
movzx ecx, byte [rdx + rcx * 8 + 0x20]
shl ecx, 0x4
movups xmm0, xmmword [rax]
movups xmmword [r13 + rcx * 1], xmm0
movzx eax, byte [r12]
lea rcx, qword [rip + 0x22a84]
jmp qword [rcx + rax * 8]
movups xmm0, xmmword [rax]
movups xmmword [r13], xmm0
xor eax, eax
ret