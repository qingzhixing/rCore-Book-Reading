	.section .text.entry
	.global _start
_start:
	la sp, boot_stack_lower_bound
	call rust_main

	.section .bss.stack
	.global boot_stack_lower_bound

boot_stack_lower_bound:
	.space 4096 * 16		// .bss 中静态分配 64KiB 栈空间
	.global boot_stack_upper_bound
boot_stack_top:
