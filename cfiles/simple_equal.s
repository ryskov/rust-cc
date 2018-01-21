.globl main
main:
movq $1, %rax
push %rax
movq $2, %rax
pop %rbx
movq $0,%rdx
idivq %rbx
push %rax
movq $5, %rax
pop %rbx
addq %rbx, %rax
push %rax
movq $3, %rax
push %rax
movq $2, %rax
pop %rbx
imul %rbx, %rax
pop %rdx
cmpq %rax,%rdx
movq $0,%rax
sete %al
ret
