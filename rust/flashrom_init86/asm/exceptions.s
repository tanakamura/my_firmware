
	.text16.ram

int_handler_0:
	mov	$0, %cs:int_number_in_segment
	jmp	int_handler


int_handler:
	mov	%eax, %cs:state16_regs_in_segment + 4*0
	mov	%ecx, %cs:state16_regs_in_segment + 4*1
	mov	%edx, %cs:state16_regs_in_segment + 4*2
	mov	%ebx, %cs:state16_regs_in_segment + 4*3
	mov	%esp, %cs:state16_regs_in_segment + 4*4
	mov	%ebp, %cs:state16_regs_in_segment + 4*5
	mov	%esi, %cs:state16_regs_in_segment + 4*6
	mov	%edi, %cs:state16_regs_in_segment + 4*7


