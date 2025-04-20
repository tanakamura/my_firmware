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

	## 0x18 = segment offset = 0xf0000, 16bit protect mode segment (see init.s)
	ljmp	$0x18,$switch_to_real_mode

	.globl	leave_from_16
leave_from_16:
	popf
	popa

	ret

	.section	.text16.ram, "ax"
	.code16
	.globl	switch_to_real_mode

switch_to_real_mode:		# seg=0xf000
	switch_regs_protect_to_real

	push	%cs: state16_regs_in_segment + 4*13 # cs
	push	%cs: state16_regs_in_segment + 4*11 # ip

	## esp[0] = bp = ip
	## esp[2] = di = cs
	lcall	*(%esp)

	switch_regs_real_to_protect

	ljmpl	$0x10, $leave_from_16
