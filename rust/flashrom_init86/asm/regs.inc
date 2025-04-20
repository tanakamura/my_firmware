
.macro  switch_regs_real_to_protect
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
	mov	%eax, %cs: state16_regs_in_segment + 4*8 # flags
	mov	%cs: state32_esp_in_segment, %esp

	## switch to protect mode
	mov	%cr0, %ebp
	or	$1, %ebp
	mov	%ebp, %cr0

	mov	$0x8, %si	# full data access
	mov	%si, %ds
	mov	%si, %es
	mov	%si, %fs
	mov	%si, %ss

.endm

.macro  switch_regs_protect_to_real

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

.endm
