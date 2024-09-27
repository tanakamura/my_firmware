	.text
	.globl print_hex_nostack
	.code32

print_hex_nostack:
	mov	$8, %ecx

	mov	$0x3f8, %edx
1:
	mov	%esi, %eax
	shr	$28, %eax
	add	$hex_table, %eax
	mov	(%eax), %al
	shl	$4, %esi
	outb	%al, %dx

	dec	%ecx
	jnz	1b

	mov	$'\r', %al
	outb	%al, %dx
	mov	$'\n', %al
	outb	%al, %dx

	jmp	*%ebp

	.section .rodata, "a"

hex_table:
	.ascii "0123456789ABCDEF"
