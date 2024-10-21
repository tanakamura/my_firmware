
	.text
	.code32
	.globl enable_car

enable_car:
	## disable cache
	mov	%cr0, %eax
	or	$0x60000000, %eax
	mov	%eax, %cr0

	## enable mtrr
	mov	$((1<<11)|0), %eax
	mov	$0, %edx
	mov	$0x2ff, %ecx
	wrmsr

	## wb : 0xfffc0000 - 0xffffffff
	xor	%edx, %edx
	mov	$0xfffc0006, %eax
	mov	$0x200, %ecx 	# mtrr base
	wrmsr
	mov	$(0xfffc0000 | (1<<11)), %eax
	mov	$0x201, %ecx	# mtrr mask
	wrmsr

	## enable cache
	mov	%cr0, %eax
	and	$0x9fffffff, %eax
	mov	%eax, %cr0

	## fill
	mov	$0xfffc0000, %esi
	mov	$(256*1024/4), %ecx
	rep	lodsl
1:

	jmp	*%ebp
