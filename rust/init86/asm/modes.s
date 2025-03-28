	.bss
	.comm state16_regs, 12 * 4
	.comm state32_esp, 4

	.text
	.globl	enter_to_16_asm
enter_to_16_asm:
	##  see lib.rs::State16
	pusha
	pushf

	mov	%esp, state32_esp

	cli

	##  see lib.rs::State16
	mov	state16_regs + 4*0, %eax
	mov	state16_regs + 4*1, %ecx
	mov	state16_regs + 4*2, %edx
	mov	state16_regs + 4*3, %ebx

	mov	state16_regs + 4*4, %esp

	mov	state16_regs + 4*9, %esi # cs
	mov	state16_regs + 4*10, %edi # ip
	mov	state16_regs + 4*11, %ebp # ds
	mov	%bp, %ds
	mov	%bp, %ss

	ljmp	$0x18,$switch_to_16_f0000_16bit

	.globl	leave_from_16
leave_from_16:
	mov	state32_esp, %esp
	popf
	popa

	ret

	.section	.text16, "ax"
	.code16

	.globl	switch_to_16_f0000_16bit
switch_to_16_f0000_16bit:
	mov	%cr0, %ebp
	and	$-2, %ebp
	mov	%ebp, %cr0

	push	%si
	push	%di

	lcall	*(%esp)

	mov	%cr0, %ebp
	and	$1, %ebp
	mov	%ebp, %cr0

	mov	$0x8, %si
	mov	%si, %ds
	mov	%si, %es
	mov	%si, %fs
	mov	%si, %ss

	ljmp	$0x18, $leave_from_16_f0000_32bit

	.code32
leave_from_16_f0000_32bit:
	ljmp	$0x10, $leave_from_16
