	.equ	UART_DATA, 0x3f8
	.equ	UART_IER, (UART_DATA+1)
	.equ	UART_IIR, (UART_DATA+2)
	.equ	UART_FCR, (UART_DATA+2)
	.equ	UART_LCR, (UART_DATA+3)
	.equ	UART_MCR, (UART_DATA+4)
	.equ	UART_LSR, (UART_DATA+5)
	.equ	UART_MSR, (UART_DATA+6)
	.equ	UART_SCR, (UART_DATA+7)
	.equ	UART_DIV_LO, 0x3f8
	.equ	UART_DIV_HI, 0x3f9

.macro superio_write port, data
	mov	\port, %al
	mov	$0x2e, %dx
	out	%al, %dx
	mov	\data, %al
	mov	$0x2f, %dx
	out	%al, %dx
.endm

.macro	pci_write_config bus, dev, fn, offset, AX, CX
	mov	$0xcf8, %dx
	mov	$(0x80000000 + (\dev<<11) + (\fn<<8) + (\offset & 0xfc)), %eax
	out	%eax, %dx
	mov	$(0xcfc+(\offset&0x3)), %dx
	mov	\CX, \AX
	out	\AX, %dx
.endm

.macro	pci_write_config32 bus, dev, fn, offset
	pci_write_config \bus, \dev, \fn, \offset, %eax, %ecx
.endm
.macro	pci_write_config16 bus, dev, fn, offset
	pci_write_config \bus, \dev, \fn, \offset, %ax, %cx
.endm
.macro	pci_write_config8 bus, dev, fn, offset
	pci_write_config \bus, \dev, \fn, \offset, %al, %cl
.endm

.macro	call_nostack label
	mov	$1f, %ebp
	jmp	\label
1:
.endm

	.section .text.keep, "ax"
	.code32

init32:
	mov	$0x08, %ax
	mov	%ax, %ds
	mov	%ax, %es
	mov	%ax, %fs
	mov	%ax, %ss

	# PCIEXBAR = 0xf0000000 and 128MB
	mov	$0xf0000003, %ecx
	pci_write_config32 0x00, 0x00, 0x00, 0x48

	# BIOS_CNTL, enable xip prefetch
	mov	$(2<<2), %ecx
	pci_write_config8 0x00, 0x1f, 0x00, 0xdc


init_lpc:
	// enable superio & coma
	mov $((1<<12)|(1<<0)), %cx
	pci_write_config16 0, 0x1f, 0x0, 0x82

init_superio_uart:
	# enter conf state
	mov	$0x55, %al
	mov	$0x2e, %dx
	outb	%al, %dx

	# UART : LDN=0x4
	superio_write $0x7, $0x4

	# disable uart
	superio_write $0x30, $0x0

	# iobase = 0x60=0x300, 0x61=0x0f8
	superio_write $0x60, $0x3
	superio_write $0x61, $0xf8

	# enable uart
	superio_write $0x30, $0x1

	# exit conf state
	mov	$0xaa, %al
	mov	$0x2e, %dx
	out	%al, %dx

init_uart:
	# init FIFO
	mov	$0b00000111, %al
	mov	$UART_FCR, %dx
	outb	%al, %dx

	## dlab=1, parity no, stop=1, data=8
	mov	$0b10000011, %al
	mov	$UART_LCR, %dx
	outb	%al, %dx

	# divider = 1, 115200
	mov	$UART_DIV_LO, %dx
	mov	$0x1, %al
	outb	%al, %dx
	mov	$UART_DIV_HI, %dx
	mov	$0x0, %al
	outb	%al, %dx

	## dlab=0, parity no, stop=1, data=8
	mov	$0b00000011, %al
	mov	$UART_LCR, %dx
	outb	%al, %dx

	mov	$1f, %ebp
	jmp	enable_car
1:

	mov	$1f, %ebp
	jmp	raminit
1:

	movl	$0x70000000, %esp # 2GiB-256M (GPU RAM)

	call	enable_sdram_cache

	# enable fpu, sse
	mov	%cr0, %eax
	and	$~(1<<2), %eax	# ~emulate coprocessor
	or	$(1<<1), %eax	# monitor coprocessor
	mov	%eax, %cr0

	mov	%cr4, %eax
	or	$((1<<9)|(1<<10)), %eax # enable sse
	mov	%eax, %cr4

	call	rmain

1:
	hlt
	jmp	1b

	.type	init32, @function
	.size	init32, .-init32

	.section .rodata, "a"
	.align	16
gdt_table:
	# selector[0x00]
	.quad	0
	# selector[0x08] : full data access
	.quad	((0xc)<<52) | (0xf<<48) | (0x93<<40) | (0xffff<<0)
	# selector[0x10] : full text access
	.quad	((0xc)<<52) | (0xf<<48) | (0x9b<<40) | (0xffff<<0)
	# selector[0x18] : initial cs compatible, base=0xf0000, limit=0xffff, 16bit, byte granularity
	.quad	((0x0)<<52) | (0x0<<48) | (0x9b<<40) | (0xf0000<<16) | (0xffff<<0)
	# selector[0x20] : initial ds compatible, base=0xf0000, limit=0xffff, 16bit, byte granularity
	.quad	((0x0)<<52) | (0x0<<48) | (0x93<<40) | (0xf0000<<16) | (0xffff<<0) # selector[0x20] : initial ds compatible

	.section	.text16, "ax"
	.code16

init:
	lgdtl	%cs:gdt
	mov	%cr0, %eax
	or	$1, %eax
	mov	%eax, %cr0
	ljmpl	$0x10,$init32

	.align	16
gdt:
	.word	(8*3)-1
	.long	gdt_table


	.section	.reset16, "ax"
	.globl	reset
reset:
	ljmp	$0xf000,$init

