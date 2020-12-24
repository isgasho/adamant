.text
.code64

.global _x86_64_asm_lgdt
.p2align 4
_x86_64_asm_lgdt:
    mov %ds, %ax
    lgdt (%rdi)
    retq

.global _x86_64_asm_lidt
.p2align 4
_x86_64_asm_lidt:
    lidt (%rdi)
    retq