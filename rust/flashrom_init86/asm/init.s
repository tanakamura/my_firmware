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

	.equ	RAM16_FLAT_ADDR, 0x80000

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

.macro	pci_read_config bus, dev, fn, offset, AX
	mov	$0xcf8, %dx
	mov	$(0x80000000 + (\dev<<11) + (\fn<<8) + (\offset & 0xfc)), %eax
	out	%eax, %dx
	mov	$(0xcfc+(\offset&0x3)), %dx
	in	%dx, \AX
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

.macro	pci_read_config8 bus, dev, fn, offset
	pci_read_config \bus, \dev, \fn, \offset, %al
.endm
.macro	pci_read_config16 bus, dev, fn, offset
	pci_read_config \bus, \dev, \fn, \offset, %ax
.endm

.macro	call_nostack label
	mov	$1f, %ebp
	jmp	\label
1:
.endm

.macro uart_init
	#  init uart
	# init FIFO
	mov	$0b00000111, %al
	mov	$UART_FCR, %dx
	outb	%al, %dx

	# enable RX interrupt
	mov	$0b00000001, %al
	mov	$UART_IER, %dx
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
.endm


	.section .text.init32, "ax"
	.code32

init32:
	mov	$0x08, %ax
	mov	%ax, %ds
	mov	%ax, %es
	mov	%ax, %fs
	mov	%ax, %ss

	mov	$0x510, %dx # select qemu signature
	xor	%al, %al
	outb	%al, %dx

	mov	$0x511, %dx
	inb	%dx, %al
	cmpb	$'Q', %al
	jne	not_qemu

	inb	%dx, %al
	cmpb	$'E', %al
	jne	not_qemu

	inb	%dx, %al
	cmpb	 $'M', %al
	jne	not_qemu

	inb	%dx, %al
	cmpb	$'U', %al
	jne	not_qemu

	uart_init

	mov	$'G', %al
	mov	$UART_DATA, %dx
	outb	%al, %dx


	# run in qemu, skip dram initialization
	jmp	raminit_done

not_qemu:
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

	// comA=0x3f8, comB=0x2f8
	mov $0x0040, %cx
	pci_write_config16 0, 0x1f, 0x0, 0x80

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
	# irq = 4
	superio_write $0x70, $0x4

	# enable uart
	superio_write $0x30, $0x1

	# exit conf state
	mov	$0xaa, %al
	mov	$0x2e, %dx
	out	%al, %dx

init_uart:
	uart_init

#	mov	$1f, %ebp
#	jmp	enable_car
#1:

	mov	$'G', %al
	mov	$UART_DATA, %dx
	outb	%al, %dx

	mov	$1f, %ebp
	jmp	raminit
1:
raminit_done:

	mov	$'O', %al
	mov	$UART_DATA, %dx
	outb	%al, %dx

	movl	$__stack_bottom, %esp

	call	enable_sdram_cache

	// Set PAM as DRAM
	## 0x000c_0000 - 0x000f_ffff : use DRAM

.equ PAMBASE_Q35, 0x90
.equ PAMBASE_I440, 0x59

	# devid
	pci_read_config16 0x00, 0x00, 0x00, 0x02

	mov	$0x1237, %cx
	cmp	%ax, %cx
	je	pam_i440

	mov	$0x30, %cl
	pci_write_config8 0x00, 0x00, 0x00, (PAMBASE_Q35 + 0)
	mov	$0x33, %cl
	pci_write_config8 0x00, 0x00, 0x00, (PAMBASE_Q35 + 1)
	mov	$0x33, %cl
	pci_write_config8 0x00, 0x00, 0x00, (PAMBASE_Q35 + 2)
	mov	$0x33, %cl
	pci_write_config8 0x00, 0x00, 0x00, (PAMBASE_Q35 + 3)
	mov	$0x33, %cl
	pci_write_config8 0x00, 0x00, 0x00, (PAMBASE_Q35 + 4)
	mov	$0x33, %cl
	pci_write_config8 0x00, 0x00, 0x00, (PAMBASE_Q35 + 5)
	mov	$0x33, %cl
	pci_write_config8 0x00, 0x00, 0x00, (PAMBASE_Q35 + 6)
	jmp	pam_done

pam_i440:
	mov	$0x30, %cl
	pci_write_config8 0x00, 0x00, 0x00, (PAMBASE_I440 + 0)
	mov	$0x33, %cl
	pci_write_config8 0x00, 0x00, 0x00, (PAMBASE_I440 + 1)
	mov	$0x33, %cl
	pci_write_config8 0x00, 0x00, 0x00, (PAMBASE_I440 + 2)
	mov	$0x33, %cl
	pci_write_config8 0x00, 0x00, 0x00, (PAMBASE_I440 + 3)
	mov	$0x33, %cl
	pci_write_config8 0x00, 0x00, 0x00, (PAMBASE_I440 + 4)
	mov	$0x33, %cl
	pci_write_config8 0x00, 0x00, 0x00, (PAMBASE_I440 + 5)
	mov	$0x33, %cl
	pci_write_config8 0x00, 0x00, 0x00, (PAMBASE_I440 + 6)
	jmp	pam_done


pam_done:

	## fill conventional memory
	movl	$0, %edi
	movl	$(640*1024/4), %ecx
	mov	$-1, %eax
	rep	stosl

	movl	$0x100000, %edi
	movl	$(15*1024*1024/4), %ecx
	rep	stosl

	xorl	%eax, %eax

	leal	__LOAD_ROM_START, %esi
	leal	__LOAD_RAM_START, %edi
	leal	__LOAD_SIZE_DW, %ecx
	rep	movsl

	leal	__LOAD_ROM16_START, %esi
	leal	__LOAD_RAM16_START_FLAT32, %edi
	leal	__LOAD_ROM16_SIZE_DW, %ecx
	rep	movsl

	leal	__BSS_RAM_START, %edi
	leal	__BSS_RAM_SIZE_DW, %ecx
	rep	stosl

	leal	__BSS_RAM16_START_FLAT32, %edi
	leal	__BSS_RAM16_SIZE_DW, %ecx
	rep	stosl

	movl	$0x400, %edi
	movl	$256/4, %ecx
	rep	stosl

	mov	$'L', %al
	mov	$UART_DATA, %dx
	outb	%al, %dx

	# enable fpu, sse
	mov	%cr0, %eax
	and	$~(1<<2), %eax	# ~emulate coprocessor
	or	$(1<<1), %eax	# monitor coprocessor
	mov	%eax, %cr0

	mov	%cr4, %eax
	or	$((1<<9)|(1<<10)), %eax # enable sse
	mov	%eax, %cr4

	## setup service function table
	mov	$set_16state, %eax
	mov	%eax, 0xf0000 + 4*0
	mov	$get_16state, %eax
	mov	%eax, 0xf0000 + 4*1
	mov	$enter_to_16, %eax
	mov	%eax, 0xf0000 + 4*2
	mov	$alloc_from_16, %eax
	mov	%eax, 0xf0000 + 4*3
	mov	$free_to_16, %eax
	mov	%eax, 0xf0000 + 4*4
	mov	$install_int_handler, %eax
	mov	%eax, 0xf0000 + 4*5

	call	flashrom_init86_rs_init
	call	common_init_from_flash

	mov	$'F', %al
	mov	$UART_DATA, %dx
	outb	%al, %dx

	mov	$0x0d, %al
	outb	%al, %dx
	mov	$0x0a, %al
	outb	%al, %dx

	call	rmain

1:
	hlt
	jmp	1b

	.type	init32, @function
	.size	init32, .-init32

	.section .rodata.init32, "a"
	.align	16
gdt_table:
	.include "asm/gdt.s"

	.section	.text16.rom, "ax"
	.code16

init:
	cli
	cld

	mov	$0x1b, %ecx
	rdmsr
	mov	%eax, %ebp
	and	$~(1<<11), %eax # disable local APIC to use 8259 PIC
	wrmsr

	test	$(1<<8), %ebp

	jnz	bsp

1:
	hlt
	jmp	1b

bsp:
	lgdtl	%cs:gdt
	mov	%cr0, %eax
	or	$1, %eax
	mov	%eax, %cr0
	ljmpl	$0x10,$init32

	.align	16
gdt:
	.word	(8*5)-1
	.long	gdt_table

	.section	.reset16, "ax"
	.globl	reset
reset:
	ljmp	$0xf000,$init

