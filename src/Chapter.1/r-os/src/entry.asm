	.section .text.entry
	.global _start
_start:
	la sp, boot_stack_lower_bound
	call rust_main

	.section .bss.stack
	.global boot_stack_lower_bound
boot_stack_lower_bound:		// 栈的理论最低部分，栈顶能到达的最小位置	
	.space 4096 * 16		// .bss 中静态分配 64KiB 栈空间
	.global boot_stack_upper_bound
bool_stack_bottom:			// 栈底在高内存
boot_stack_top:				// 栈顶目前在的位置
