.globl main
main:
movq $1, %rax
push %rax
movq $0, %rax
pop %rdx
orq %rax,%rdx
movq $0,%rax
setne %al
ret
