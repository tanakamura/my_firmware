
	.section .text.init32, "ax"
	.code32
	.globl enable_car
	.type enable_car, @function

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

	.size	enable_car, .-enable_car



	.globl enable_sdram_cache
	.type enable_sdram_cache, @function

enable_sdram_cache:

	## disable cache
	mov	%cr0, %eax
	or	$0x60000000, %eax
	mov	%eax, %cr0

	## MSR_BBL_CR_CTL3
	mov	$0x11e, %ecx
	rdmsr
	or	$(1<<8), %eax # enable L2
	wrmsr

	## invalidate L2
	invd

	## disable variable mtrr and fixed mtrr
	mov	$0x2ff, %ecx
	rdmsr
	mov	$~((1<<11)|(1<<10)), %ebx
	andl	%ebx, %eax
	wrmsr

.macro fixed_set_6 msr
	mov	$\msr, %ecx
	wrmsr
.endm

	mov	$0x06060606, %eax
	mov	%eax, %edx

	fixed_set_6	0x250
	fixed_set_6	0x258
	fixed_set_6	0x259
	fixed_set_6	0x259
	fixed_set_6	0x268
	fixed_set_6	0x269
	fixed_set_6	0x26A
	fixed_set_6	0x26B
	fixed_set_6	0x26C
	fixed_set_6	0x26D
	fixed_set_6	0x26E
	fixed_set_6	0x26F

	## wb : 0x0000_0000 - 0x7fffffff (SDRAM)
	xor	%edx, %edx
	mov	$0x00000006, %eax #base = 0, type = 6
	mov	$0x200, %ecx 	# mtrr base
	wrmsr
	mov	$(0x80000000 | (1<<11)), %eax # size = 0x80000000 (2GiB), valid=1
	mov	$0x201, %ecx	# mtrr mask
	wrmsr

	## wb : 0xfffc0000 - 0xffffffff (SPI Flash)
	xor	%edx, %edx
	mov	$0xfffc0006, %eax
	mov	$0x202, %ecx 	# mtrr base
	wrmsr
	mov	$(0xfffc0000 | (1<<11)), %eax
	mov	$0x203, %ecx	# mtrr mask
	wrmsr

	## enable variable mtrr and fixed mtrr
	mov	$((1<<11)|(1<<10)), %eax
	mov	$0, %edx
	mov	$0x2ff, %ecx
	wrmsr

	## enable cache
	mov	%cr0, %eax
	and	$0x9fffffff, %eax
	mov	%eax, %cr0


	ret


