.globl main
main:
movq $2, %rax
push %rax
movq $1, %rax
pop %rbx
addq %rbx, %rax
ret
