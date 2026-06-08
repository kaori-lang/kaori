sub rsp, 0x28
movzx eax, byte [r12 + 0x2]
movzx ecx, byte [r12 + 0x3]
shl eax, 0x4
shl ecx, 0x4
movzx edx, byte [r13 + rcx * 1]
or dl, byte [r13 + rax * 1]
jnz 0x65322
add rax, r13
add rcx, r13
movsd xmm0, qword [rcx + 0x8]
addsd xmm0, qword [rax + 0x8]
movzx eax, byte [r12 + 0x1]
shl eax, 0x4
mov byte [r13 + rax * 1], 0x0
movsd qword [r13 + rax * 1 + 0x8], xmm0
movzx eax, byte [r12 + 0x8]
add r12, 0x8
lea rcx, qword [rip + 0x25516]
add rsp, 0x28
jmp qword [rcx + rax * 8]
mov edx, 0x29
xor ecx, ecx
call 0x73ae0
test rax, rax
jz 0x6539d
mov rsi, rax
movups xmm0, xmmword [rip + 0x25308]
movups xmmword [rax + 0x19], xmm0
movups xmm0, xmmword [rip + 0x252f4]
movups xmmword [rax + 0x10], xmm0
movups xmm0, xmmword [rip + 0x252d9]
movups xmmword [rax], xmm0
mov edx, 0x28
xor ecx, ecx
call 0x73ae0
test rax, rax
jz 0x6538e
mov qword [rax], 0x29
mov qword [rax + 0x8], rsi
mov qword [rax + 0x10], 0x29
mov qword [rax + 0x18], 0x0
mov dword [rax + 0x20], 0x0
add rsp, 0x28
ret
mov ecx, 0x8
mov edx, 0x28
call 0x7bcc5
mov ecx, 0x1
mov edx, 0x29
call 0x7bb9c
int 0x3


sub rsp, 0x28
movzx eax, byte [r12 + 0x2]
movzx ecx, byte [r12 + 0x3]
shl eax, 0x4
shl ecx, 0x4
movzx edx, byte [r13 + rcx * 1]
or dl, byte [r13 + rax * 1]
jnz 0x657d2
add rax, r13
add rcx, r13
movsd xmm0, qword [rcx + 0x8]
mulsd xmm0, qword [rax + 0x8]
movzx eax, byte [r12 + 0x1]
shl eax, 0x4
mov byte [r13 + rax * 1], 0x0
movsd qword [r13 + rax * 1 + 0x8], xmm0
movzx eax, byte [r12 + 0x8]
add r12, 0x8
lea rcx, qword [rip + 0x25066]
add rsp, 0x28
jmp qword [rcx + rax * 8]
mov edx, 0x2e
xor ecx, ecx
call 0x73ae0
test rax, rax
jz 0x6584d
mov rsi, rax
movups xmm0, xmmword [rip + 0x24f9e]
movups xmmword [rax + 0x1e], xmm0
movups xmm0, xmmword [rip + 0x24f85]
movups xmmword [rax + 0x10], xmm0
movups xmm0, xmmword [rip + 0x24f6a]
movups xmmword [rax], xmm0
mov edx, 0x28
xor ecx, ecx
call 0x73ae0
test rax, rax
jz 0x6583e
mov qword [rax], 0x2e
mov qword [rax + 0x8], rsi
mov qword [rax + 0x10], 0x2e
mov qword [rax + 0x18], 0x0
mov dword [rax + 0x20], 0x0
add rsp, 0x28
ret
mov ecx, 0x8
mov edx, 0x28
call 0x7bcc5
mov ecx, 0x1
mov edx, 0x2e
call 0x7bb9c
int 0x3

