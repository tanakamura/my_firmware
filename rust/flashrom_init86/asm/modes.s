	.include "asm/regs.inc"

	.section	.bda.bss, "ax"
	# state16_regs is defined by linker script
	# state32_esp is defined by linker script

	.text
	.globl	enter_to_16_asm
enter_to_16_asm:
	##  see lib.rs::State16
	pusha
	pushf

	mov	%esp, state32_esp_flat32

	cli

#	##  see lib.rs::State16
#	mov	state16_regs + 4*0, %eax
#	mov	state16_regs + 4*1, %ecx
#	mov	state16_regs + 4*2, %edx
#	mov	state16_regs + 4*3, %ebx
#
#	mov	state16_regs + 4*4, %esp
#
#	mov	state16_regs + 4*9, %edi # ip (saved in edi)
#
#	mov	state16_regs + 4*10, %ebp # es
#	mov	%bp, %es
#	mov	state16_regs + 4*11, %ebp # ss
#	mov	%bp, %ss
#
#	mov	state16_regs + 4*12, %ebp # cs (saved in bp)
#
#	mov	state16_regs + 4*13, %esi # ds (saved in esi)

	## ip in real mode = di
	## cs in real mode = bp
	## ds in real mode = si
	## 0x18 = segment offset = 0xf0000, 16bit protect mode segment (see init.s)
	ljmp	$0x18,$switch_to_real_mode

	.globl	leave_from_16
leave_from_16:
	popf
	popa

	ret

	.section	.text16.ram, "ax"
	.code16
	.globl	nop16

nop16:
	nop
	nop
	ret

	.section	.text16.ram, "ax"
	.code16
	.globl	switch_to_real_mode

switch_to_real_mode:		# seg=0xf000
	## ip in real mode = di
	## cs in real mode = bp
	## ds in real mode = si

	switch_regs_protect_to_real

	## esp[0] = bp = ip
	## esp[2] = di = cs
	lcall	*(%esp)

	switch_regs_real_to_protect

	ljmpl	$0x10, $leave_from_16
