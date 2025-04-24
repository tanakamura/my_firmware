
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

.macro fixed_mtrr_set msr
	mov	$\msr, %ecx
	wrmsr
.endm

	mov	$0x06060606, %eax # WB
	mov	%eax, %edx

	fixed_mtrr_set	0x250 	# 0x0_0000-0x7_ffff 64k
	fixed_mtrr_set	0x258	# 0x8_0000-0x9_ffff 16k
	fixed_mtrr_set	0x268   # 0xc_0000-0xc_8fff 4k
	fixed_mtrr_set	0x269   # 0xc_8000-0xc_ffff 4k
	fixed_mtrr_set	0x26A   # 0xd_0000-0xd_8fff 4k
	fixed_mtrr_set	0x26B   # 0xd_8000-0xd_ffff 4k
	fixed_mtrr_set	0x26C   # 0xe_0000-0xe_8fff 4k
	fixed_mtrr_set	0x26D   # 0xe_8000-0xe_ffff 4k
	fixed_mtrr_set	0x26E   # 0xf_0000-0xf_8fff 4k
	fixed_mtrr_set	0x26F   # 0xf_8000-0xf_ffff 4k

	mov	$0x01010101, %eax # WC
	mov	%eax, %edx

	fixed_mtrr_set	0x259	# 0xa_0000-0xb_ffff 16k (VGA)


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


