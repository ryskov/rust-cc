.globl main
main:
movq $1, %rax
push %rax
movq $2, %rax
pop %rbx
subq %rbx, %rax
ret
