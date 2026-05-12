sub rsp, 0x28
movzx eax, byte [r12 + 0x2]
mov rax, qword [r13 + rax * 8]
mov rcx, rax
not rcx
mov rdx, 0x7ffc000000000000
test rcx, rdx
jz 0x593d1
movzx ecx, byte [r12 + 0x1]
movzx edx, word [r12 + 0x4]
cvtsi2sd xmm0, edx
movq xmm1, rax
addsd xmm1, xmm0
movsd qword [r13 + rcx * 8], xmm1
movzx eax, byte [r12 + 0x8]
add r12, 0x8
lea rcx, qword [rip + 0x2882b]
movzx edi, dil
add rsp, 0x28
jmp qword [rcx + rax * 8]
mov edx, 0x29
xor ecx, ecx
call 0x6d070
test rax, rax
jz 0x59432
mov rsi, rax
movups xmm0, xmmword [rip + 0x285c7]
movups xmmword [rax + 0x19], xmm0
movups xmm0, xmmword [rip + 0x285b3]
movups xmmword [rax + 0x10], xmm0
movups xmm0, xmmword [rip + 0x28598]
movups xmmword [rax], xmm0
call 0x5bf70
mov rdx, rax
mov qword [rax], 0x0
mov qword [rax + 0x18], 0x29
mov qword [rax + 0x20], rsi
mov qword [rax + 0x28], 0x29
mov eax, 0x1
add rsp, 0x28
ret
mov ecx, 0x1
mov edx, 0x29
call 0x740ec
int 0x3