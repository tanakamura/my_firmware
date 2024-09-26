
	.text
	.code32
	.globl enable_car

enable_car:
	## disable cache
	mov	%cr0, %eax
	or	$0x60000000, %eax
	mov	%eax, %cr0

	## enable mtrr
	mov	$((1<<10)|(1<<11)), %eax
	mov	$0, %edx
	mov	$0x2ff, %ecx
	wrmsr

	mov	$0x06060606, %edx
	mov	%edx, %eax
	mov	$0x268, %ecx

1:
	wrmsr
	inc	%ecx
	cmp	$0x270, %ecx
	jnz	1b

	## enable cache
	mov	%cr0, %eax
	and	$0x9fffffff, %eax
	mov	%eax, %cr0

	## fill
	mov	$0xc0000, %esi
	mov	$(256*1024/4), %ecx
	rep	lodsl

	jmp	*%ebp
