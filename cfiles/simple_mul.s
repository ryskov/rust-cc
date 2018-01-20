.globl main
main:
movq $2, %rax
push %rax
movq $3, %rax
push %rax
movq $3, %rax
pop %rbx
imul %rbx, %rax
pop %rbx
addq %rbx, %rax
push %rax
movq $2, %rax
push %rax
movq $2, %rax
push %rax
movq $0, %rax
cmpq $0, %rax
movq $0, %rax
sete %al
pop %rbx
addq %rbx, %rax
pop %rbx
imul %rbx, %rax
pop %rbx
addq %rbx, %rax
ret
