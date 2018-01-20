.globl main
main:
movq $2, %rax
push %rax
movq $4, %rax
pop %rbx
movq $0,%rdx
idivq %rbx
ret
