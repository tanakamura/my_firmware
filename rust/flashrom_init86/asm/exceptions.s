	.include "asm/regs.inc"

	.code16
	.section	.text16.ram, "ax"

	.globl	int_handler_0h
	.globl	int_handler_4h
	.globl	int_handler_10h

	## real mode

int_handler_0h:
	movl	$0, %cs:int_number_in_segment
	jmp	int_handler

int_handler_4h:
	movl	$0x4, %cs:int_number_in_segment
	jmp	int_handler

int_handler_10h:
	movl	$0x10, %cs:int_number_in_segment
	jmp	int_handler


int_handler:
	switch_regs_real_to_protect

	ljmpl	$0x10, $invoke_int_handler


return_from_int_handler:
	switch_regs_protect_to_real

	iret


	.code32
	## protect mode
	.text
invoke_int_handler:
	call	handle_exceptions
	ljmpl	$0x18, $return_from_int_handler
