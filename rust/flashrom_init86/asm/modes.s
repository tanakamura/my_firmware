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
	mov	state32_esp_flat32, %esp
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

	##  switch to real mode from protect mode
	mov	%cr0, %eax
	and	$-2, %eax
	mov	%eax, %cr0

	ljmp	$__RAM16_SEGMENT,$1f		# leave protect mode
1:

	mov	%cs: state16_regs_in_segment + 4*8, %eax # flags
	push	%ax
	popf

	##  see lib.rs::X86State
	mov	%cs: state16_regs_in_segment + 4*0, %eax
	mov	%cs: state16_regs_in_segment + 4*1, %ecx
	mov	%cs: state16_regs_in_segment + 4*2, %edx
	mov	%cs: state16_regs_in_segment + 4*3, %ebx
	mov	%cs: state16_regs_in_segment + 4*4, %esp
	mov	%cs: state16_regs_in_segment + 4*5, %ebp
	mov	%cs: state16_regs_in_segment + 4*6, %esi
	mov	%cs: state16_regs_in_segment + 4*7, %edi

	mov	%cs: state16_regs_in_segment + 4*9, %es # es
	mov	%cs: state16_regs_in_segment + 4*10, %ss
	mov	%cs: state16_regs_in_segment + 4*12, %ds

	push	%cs: state16_regs_in_segment + 4*13 # cs
	push	%cs: state16_regs_in_segment + 4*11 # ip

	## esp[0] = bp = ip
	## esp[2] = di = cs
	lcall	*(%esp)

	##  see lib.rs::X86State
	mov	%eax, %cs:state16_regs_in_segment + 4*0
	mov	%ecx, %cs:state16_regs_in_segment + 4*1
	mov	%edx, %cs:state16_regs_in_segment + 4*2
	mov	%ebx, %cs:state16_regs_in_segment + 4*3
	mov	%esp, %cs:state16_regs_in_segment + 4*4
	mov	%ebp, %cs:state16_regs_in_segment + 4*5
	mov	%esi, %cs:state16_regs_in_segment + 4*6
	mov	%edi, %cs:state16_regs_in_segment + 4*7

	mov	%es, %cs: state16_regs_in_segment + 4*9
	mov	%ss, %cs: state16_regs_in_segment + 4*10
	mov	%ds, %cs: state16_regs_in_segment + 4*12

	pushf
	pop	%ax
	mov	%eax, %cs: state16_regs_in_segment + 4*8

	## switch to protect mode
	mov	%cr0, %ebp
	or	$1, %ebp
	mov	%ebp, %cr0

	mov	$0x8, %si	# full data access
	mov	%si, %ds
	mov	%si, %es
	mov	%si, %fs
	mov	%si, %ss

	ljmpl	$0x10, $leave_from_16
