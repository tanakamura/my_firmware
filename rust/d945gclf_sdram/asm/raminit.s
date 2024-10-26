	.text
	.globl	raminit
	.type raminit, @function

	.equ PMCON2, 0xa2
	.equ PMCON3, 0xa4

	.equ D31F0_BASE, (0<<20) | (31<<15) | (0<<12)
	.equ PMCON2_ADDR, (0xF0000000 + D31F0_BASE + PMCON2)
	.equ PMCON3_ADDR, (0xF0000000 + D31F0_BASE + PMCON3)

raminit:
	mov	$'A', %al
	mov	$0x3f8, %dx
	outb	%al, %dx

	mov	PMCON2_ADDR, %eax
	test	$(1<<2), %eax
	jz	test_ok

	## restart
	mov	%eax, PMCON2_ADDR
	mov	PMCON3_ADDR, %eax
	or	$(1<<3), %eax
	mov	%eax, PMCON3_ADDR

	mov	$'B', %al
	mov	$0x3f8, %dx
	outb	%al, %dx

	mov	$0xa, %eax
	mov	$0xcf9, %edx
	outb	%al, %dx
	mov	$0xe, %eax
	outb	%al, %dx

2:
	jmp	2b

test_ok:
	mov	$'C', %al
	mov	$0x3f8, %dx
	outb	%al, %dx

// set MCH_BASE to 0xfed10000
movl $0xfed10001, %eax
movl %eax, 0xf0000044
//RB,MCH,c00,1
movb 0xfed10c00, %al
//WL,PCI,0,0,54,b8000009
movl $0xb8000009, %eax
movl %eax, 0xf0000054
//RB,PCI,1f,0,a4,1
movb 0xf00f80a4, %al
//WB,PCI,1f,0,a4,f5
movb $0xf5, %al
movb %al, 0xf00f80a4
//RB,PCI,1f,0,a2,f5
movb 0xf00f80a2, %al
//RB,PCI,1f,0,a4,5
movb 0xf00f80a4, %al
//RL,MCH,40,100
movl 0xfed10040, %eax
//WL,MCH,40,100
movl $0x00000100, %eax
movl %eax, 0xfed10040
//RL,MCH,40,100
movl 0xfed10040, %eax
//WL,MCH,40,100
movl $0x00000100, %eax
movl %eax, 0xfed10040
//RL,MCH,40,100
movl 0xfed10040, %eax
//WL,MCH,bd4,cc
movl $0x000000cc, %eax
movl %eax, 0xfed10bd4
//WL,MCH,bd0,ccffff00
movl $0xccffff00, %eax
movl %eax, 0xfed10bd0
//RB,PCI,1f,0,a4,2
movb 0xf00f80a4, %al
//WB,PCI,1f,0,a4,f5
movb $0xf5, %al
movb %al, 0xf00f80a4
//RB,MCH,c00,1
movb 0xfed10c00, %al
//RL,MCH,f14,101
movl 0xfed10f14, %eax
//RL,MCH,f14,101
movl 0xfed10f14, %eax
//WL,MCH,f14,103
movl $0x00000103, %eax
movl %eax, 0xfed10f14
//RL,MCH,80,0
movl 0xfed10080, %eax
//WL,MCH,80,10000000
movl $0x10000000, %eax
movl %eax, 0xfed10080
//RL,MCH,200,3
movl 0xfed10200, %eax
//WL,MCH,200,0
movl $0x00000000, %eax
movl %eax, 0xfed10200
//RL,MCH,c00,20000001
movl 0xfed10c00, %eax
//WL,MCH,c00,20000021
movl $0x20000021, %eax
movl %eax, 0xfed10c00
//RL,MCH,c00,20000021
movl 0xfed10c00, %eax
//RB,MCH,c09,0
movb 0xfed10c09, %al
//WB,MCH,c09,4
movb $0x04, %al
movb %al, 0xfed10c09
//WL,MCH,208,1000400
movl $0x01000400, %eax
movl %eax, 0xfed10208
//WL,MCH,20c,200
movl $0x00000200, %eax
movl %eax, 0xfed1020c
//WL,MCH,138,100401
movl $0x00100401, %eax
movl %eax, 0xfed10138
//WL,MCH,1b8,100401
movl $0x00100401, %eax
movl %eax, 0xfed101b8
//WL,MCH,13c,0
movl $0x00000000, %eax
movl %eax, 0xfed1013c
//WL,MCH,1bc,0
movl $0x00000000, %eax
movl %eax, 0xfed101bc
//RB,MCH,c09,4
movb 0xfed10c09, %al
//WB,MCH,c09,6
movb $0x06, %al
movb %al, 0xfed10c09
//RB,MCH,c09,4
movb 0xfed10c09, %al
//RB,MCH,c09,4
movb 0xfed10c09, %al
//WB,MCH,c09,0
movb $0x00, %al
movb %al, 0xfed10c09
//RH,MCH,10e,0
movw 0xfed1010e, %ax
//WH,MCH,10e,0
movw $0x0000, %ax
movw %ax, 0xfed1010e
//RH,MCH,10e,0
movw 0xfed1010e, %ax
//WH,MCH,10e,5
movw $0x0005, %ax
movw %ax, 0xfed1010e
//RH,MCH,18e,0
movw 0xfed1018e, %ax
//WH,MCH,18e,0
movw $0x0000, %ax
movw %ax, 0xfed1018e
//RL,MCH,114,2483d22
movl 0xfed10114, %eax
//WL,MCH,114,2493d22
movl $0x02493d22, %eax
movl %eax, 0xfed10114
//RL,MCH,194,2483d22
movl 0xfed10194, %eax
//WL,MCH,194,2493d22
movl $0x02493d22, %eax
movl %eax, 0xfed10194
//RL,MCH,120,40002802
movl 0xfed10120, %eax
//WL,MCH,120,40000806
movl $0x40000806, %eax
movl %eax, 0xfed10120
//RL,MCH,1a0,40002802
movl 0xfed101a0, %eax
//WL,MCH,1a0,40000806
movl $0x40000806, %eax
movl %eax, 0xfed101a0
//RL,MCH,110,a96038e8
movl 0xfed10110, %eax
//WL,MCH,110,b95020e8
movl $0xb95020e8, %eax
movl %eax, 0xfed10110
//RL,MCH,190,a96038e8
movl 0xfed10190, %eax
//WL,MCH,190,b95020e8
movl $0xb95020e8, %eax
movl %eax, 0xfed10190
//RL,MCH,114,2493d22
movl 0xfed10114, %eax
//WL,MCH,114,2618922
movl $0x02618922, %eax
movl %eax, 0xfed10114
//RL,MCH,194,2493d22
movl 0xfed10194, %eax
//WL,MCH,194,2618922
movl $0x02618922, %eax
movl %eax, 0xfed10194
//RL,MCH,118,800003ff
movl 0xfed10118, %eax
//WL,MCH,118,8000025f
movl $0x8000025f, %eax
movl %eax, 0xfed10118
//RL,MCH,198,800003ff
movl 0xfed10198, %eax
//WL,MCH,198,8000025f
movl $0x8000025f, %eax
movl %eax, 0xfed10198
//RL,MCH,220,264
movl 0xfed10220, %eax
//WL,MCH,220,3000364
movl $0x03000364, %eax
movl %eax, 0xfed10220
//RL,MCH,224,0
movl 0xfed10224, %eax
//WL,MCH,224,43000
movl $0x00043000, %eax
movl %eax, 0xfed10224
//RL,MCH,224,43000
movl 0xfed10224, %eax
//WL,MCH,224,3000
movl $0x00003000, %eax
movl %eax, 0xfed10224
//RL,MCH,124,0
movl 0xfed10124, %eax
//WL,MCH,124,2
movl $0x00000002, %eax
movl %eax, 0xfed10124
//RL,MCH,1a4,0
movl 0xfed101a4, %eax
//WL,MCH,1a4,2
movl $0x00000002, %eax
movl %eax, 0xfed101a4
//RL,MCH,228,100
movl 0xfed10228, %eax
//WL,MCH,228,3100
movl $0x00003100, %eax
movl %eax, 0xfed10228
//RL,MCH,10c,50000
movl 0xfed1010c, %eax
//WL,MCH,10c,50007
movl $0x00050007, %eax
movl %eax, 0xfed1010c
//WB,MCH,410,44
movb $0x44, %al
movb %al, 0xfed10410
//WB,MCH,490,44
movb $0x44, %al
movb %al, 0xfed10490
//WB,MCH,420,0
movb $0x00, %al
movb %al, 0xfed10420
//WB,MCH,428,0
movb $0x00, %al
movb %al, 0xfed10428
//WB,MCH,430,44
movb $0x44, %al
movb %al, 0xfed10430
//WB,MCH,438,44
movb $0x44, %al
movb %al, 0xfed10438
//WB,MCH,418,33
movb $0x33, %al
movb %al, 0xfed10418
//WL,MCH,680,8070706
movl $0x08070706, %eax
movl %eax, 0xfed10680
//RL,MCH,680,8070706
movl 0xfed10680, %eax
//WL,MCH,500,8070706
movl $0x08070706, %eax
movl %eax, 0xfed10500
//WL,MCH,684,a090908
movl $0x0a090908, %eax
movl %eax, 0xfed10684
//RL,MCH,684,a090908
movl 0xfed10684, %eax
//WL,MCH,504,a090908
movl $0x0a090908, %eax
movl %eax, 0xfed10504
//WL,MCH,688,d0c0b0a
movl $0x0d0c0b0a, %eax
movl %eax, 0xfed10688
//RL,MCH,688,d0c0b0a
movl 0xfed10688, %eax
//WL,MCH,508,d0c0b0a
movl $0x0d0c0b0a, %eax
movl %eax, 0xfed10508
//WL,MCH,68c,12100f0e
movl $0x12100f0e, %eax
movl %eax, 0xfed1068c
//RL,MCH,68c,12100f0e
movl 0xfed1068c, %eax
//WL,MCH,50c,12100f0e
movl $0x12100f0e, %eax
movl %eax, 0xfed1050c
//WL,MCH,690,1a181614
movl $0x1a181614, %eax
movl %eax, 0xfed10690
//RL,MCH,690,1a181614
movl 0xfed10690, %eax
//WL,MCH,510,1a181614
movl $0x1a181614, %eax
movl %eax, 0xfed10510
//WL,MCH,694,22201e1c
movl $0x22201e1c, %eax
movl %eax, 0xfed10694
//RL,MCH,694,22201e1c
movl 0xfed10694, %eax
//WL,MCH,514,22201e1c
movl $0x22201e1c, %eax
movl %eax, 0xfed10514
//WL,MCH,698,2a282624
movl $0x2a282624, %eax
movl %eax, 0xfed10698
//RL,MCH,698,2a282624
movl 0xfed10698, %eax
//WL,MCH,518,2a282624
movl $0x2a282624, %eax
movl %eax, 0xfed10518
//WL,MCH,69c,3934302d
movl $0x3934302d, %eax
movl %eax, 0xfed1069c
//RL,MCH,69c,3934302d
movl 0xfed1069c, %eax
//WL,MCH,51c,3934302d
movl $0x3934302d, %eax
movl %eax, 0xfed1051c
//WL,MCH,6a0,a090908
movl $0x0a090908, %eax
movl %eax, 0xfed106a0
//RL,MCH,6a0,a090908
movl 0xfed106a0, %eax
//WL,MCH,520,a090908
movl $0x0a090908, %eax
movl %eax, 0xfed10520
//WL,MCH,6a4,c0b0b0a
movl $0x0c0b0b0a, %eax
movl %eax, 0xfed106a4
//RL,MCH,6a4,c0b0b0a
movl 0xfed106a4, %eax
//WL,MCH,524,c0b0b0a
movl $0x0c0b0b0a, %eax
movl %eax, 0xfed10524
//WL,MCH,6a8,e0d0d0c
movl $0x0e0d0d0c, %eax
movl %eax, 0xfed106a8
//RL,MCH,6a8,e0d0d0c
movl 0xfed106a8, %eax
//WL,MCH,528,e0d0d0c
movl $0x0e0d0d0c, %eax
movl %eax, 0xfed10528
//WL,MCH,6ac,1211100f
movl $0x1211100f, %eax
movl %eax, 0xfed106ac
//RL,MCH,6ac,1211100f
movl 0xfed106ac, %eax
//WL,MCH,52c,1211100f
movl $0x1211100f, %eax
movl %eax, 0xfed1052c
//WL,MCH,6b0,19171513
movl $0x19171513, %eax
movl %eax, 0xfed106b0
//RL,MCH,6b0,19171513
movl 0xfed106b0, %eax
//WL,MCH,530,19171513
movl $0x19171513, %eax
movl %eax, 0xfed10530
//WL,MCH,6b4,211f1d1b
movl $0x211f1d1b, %eax
movl %eax, 0xfed106b4
//RL,MCH,6b4,211f1d1b
movl 0xfed106b4, %eax
//WL,MCH,534,211f1d1b
movl $0x211f1d1b, %eax
movl %eax, 0xfed10534
//WL,MCH,6b8,2d292623
movl $0x2d292623, %eax
movl %eax, 0xfed106b8
//RL,MCH,6b8,2d292623
movl 0xfed106b8, %eax
//WL,MCH,538,2d292623
movl $0x2d292623, %eax
movl %eax, 0xfed10538
//WL,MCH,6bc,3f393531
movl $0x3f393531, %eax
movl %eax, 0xfed106bc
//RL,MCH,6bc,3f393531
movl 0xfed106bc, %eax
//WL,MCH,53c,3f393531
movl $0x3f393531, %eax
movl %eax, 0xfed1053c
//WL,MCH,580,7070606
movl $0x07070606, %eax
movl %eax, 0xfed10580
//WL,MCH,584,e0c0a08
movl $0x0e0c0a08, %eax
movl %eax, 0xfed10584
//WL,MCH,588,17141210
movl $0x17141210, %eax
movl %eax, 0xfed10588
//WL,MCH,58c,201e1c1a
movl $0x201e1c1a, %eax
movl %eax, 0xfed1058c
//WL,MCH,590,28262422
movl $0x28262422, %eax
movl %eax, 0xfed10590
//WL,MCH,594,302e2c2a
movl $0x302e2c2a, %eax
movl %eax, 0xfed10594
//WL,MCH,598,38363432
movl $0x38363432, %eax
movl %eax, 0xfed10598
//WL,MCH,59c,3f3e3c3a
movl $0x3f3e3c3a, %eax
movl %eax, 0xfed1059c
//WL,MCH,5a0,13131212
movl $0x13131212, %eax
movl %eax, 0xfed105a0
//WL,MCH,5a4,16151414
movl $0x16151414, %eax
movl %eax, 0xfed105a4
//WL,MCH,5a8,211d1a18
movl $0x211d1a18, %eax
movl %eax, 0xfed105a8
//WL,MCH,5ac,28262422
movl $0x28262422, %eax
movl %eax, 0xfed105ac
//WL,MCH,5b0,302e2c2a
movl $0x302e2c2a, %eax
movl %eax, 0xfed105b0
//WL,MCH,5b4,38363432
movl $0x38363432, %eax
movl %eax, 0xfed105b4
//WL,MCH,5b8,3f3e3c3a
movl $0x3f3e3c3a, %eax
movl %eax, 0xfed105b8
//WL,MCH,5bc,3f3f3f3f
movl $0x3f3f3f3f, %eax
movl %eax, 0xfed105bc
//WL,MCH,5c0,7070606
movl $0x07070606, %eax
movl %eax, 0xfed105c0
//WL,MCH,5c4,e0c0a08
movl $0x0e0c0a08, %eax
movl %eax, 0xfed105c4
//WL,MCH,5c8,17141210
movl $0x17141210, %eax
movl %eax, 0xfed105c8
//WL,MCH,5cc,201e1c1a
movl $0x201e1c1a, %eax
movl %eax, 0xfed105cc
//WL,MCH,5d0,28262422
movl $0x28262422, %eax
movl %eax, 0xfed105d0
//WL,MCH,5d4,302e2c2a
movl $0x302e2c2a, %eax
movl %eax, 0xfed105d4
//WL,MCH,5d8,38363432
movl $0x38363432, %eax
movl %eax, 0xfed105d8
//WL,MCH,5dc,3f3e3c3a
movl $0x3f3e3c3a, %eax
movl %eax, 0xfed105dc
//WL,MCH,5e0,13131212
movl $0x13131212, %eax
movl %eax, 0xfed105e0
//WL,MCH,5e4,16151414
movl $0x16151414, %eax
movl %eax, 0xfed105e4
//WL,MCH,5e8,211d1a18
movl $0x211d1a18, %eax
movl %eax, 0xfed105e8
//WL,MCH,5ec,28262422
movl $0x28262422, %eax
movl %eax, 0xfed105ec
//WL,MCH,5f0,302e2c2a
movl $0x302e2c2a, %eax
movl %eax, 0xfed105f0
//WL,MCH,5f4,38363432
movl $0x38363432, %eax
movl %eax, 0xfed105f4
//WL,MCH,5f8,3f3e3c3a
movl $0x3f3e3c3a, %eax
movl %eax, 0xfed105f8
//WL,MCH,5fc,3f3f3f3f
movl $0x3f3f3f3f, %eax
movl %eax, 0xfed105fc
//WL,MCH,600,c0b0b0b
movl $0x0c0b0b0b, %eax
movl %eax, 0xfed10600
//WL,MCH,604,d0d0c0c
movl $0x0d0d0c0c, %eax
movl %eax, 0xfed10604
//WL,MCH,608,100f0e0d
movl $0x100f0e0d, %eax
movl %eax, 0xfed10608
//WL,MCH,60c,15131211
movl $0x15131211, %eax
movl %eax, 0xfed1060c
//WL,MCH,610,1d1b1917
movl $0x1d1b1917, %eax
movl %eax, 0xfed10610
//WL,MCH,614,2523211f
movl $0x2523211f, %eax
movl %eax, 0xfed10614
//WL,MCH,618,2a282927
movl $0x2a282927, %eax
movl %eax, 0xfed10618
//WL,MCH,61c,32302e2c
movl $0x32302e2c, %eax
movl %eax, 0xfed1061c
//WL,MCH,620,9090808
movl $0x09090808, %eax
movl %eax, 0xfed10620
//WL,MCH,624,c0b0b0a
movl $0x0c0b0b0a, %eax
movl %eax, 0xfed10624
//WL,MCH,628,100f0e0d
movl $0x100f0e0d, %eax
movl %eax, 0xfed10628
//WL,MCH,62c,14131211
movl $0x14131211, %eax
movl %eax, 0xfed1062c
//WL,MCH,630,18171615
movl $0x18171615, %eax
movl %eax, 0xfed10630
//WL,MCH,634,1e1c1a19
movl $0x1e1c1a19, %eax
movl %eax, 0xfed10634
//WL,MCH,638,26242220
movl $0x26242220, %eax
movl %eax, 0xfed10638
//WL,MCH,63c,2e2c2a28
movl $0x2e2c2a28, %eax
movl %eax, 0xfed1063c
//WL,MCH,640,c0b0b0b
movl $0x0c0b0b0b, %eax
movl %eax, 0xfed10640
//WL,MCH,644,d0d0c0c
movl $0x0d0d0c0c, %eax
movl %eax, 0xfed10644
//WL,MCH,648,100f0e0d
movl $0x100f0e0d, %eax
movl %eax, 0xfed10648
//WL,MCH,64c,15131211
movl $0x15131211, %eax
movl %eax, 0xfed1064c
//WL,MCH,650,1d1b1917
movl $0x1d1b1917, %eax
movl %eax, 0xfed10650
//WL,MCH,654,2523211f
movl $0x2523211f, %eax
movl %eax, 0xfed10654
//WL,MCH,658,2a282927
movl $0x2a282927, %eax
movl %eax, 0xfed10658
//WL,MCH,65c,32302e2c
movl $0x32302e2c, %eax
movl %eax, 0xfed1065c
//WL,MCH,660,9090808
movl $0x09090808, %eax
movl %eax, 0xfed10660
//WL,MCH,664,c0b0b0a
movl $0x0c0b0b0a, %eax
movl %eax, 0xfed10664
//WL,MCH,668,100f0e0d
movl $0x100f0e0d, %eax
movl %eax, 0xfed10668
//WL,MCH,66c,14131211
movl $0x14131211, %eax
movl %eax, 0xfed1066c
//WL,MCH,670,18171615
movl $0x18171615, %eax
movl %eax, 0xfed10670
//WL,MCH,674,1e1c1a19
movl $0x1e1c1a19, %eax
movl %eax, 0xfed10674
//WL,MCH,678,26242220
movl $0x26242220, %eax
movl %eax, 0xfed10678
//WL,MCH,67c,2e2c2a28
movl $0x2e2c2a28, %eax
movl %eax, 0xfed1067c
//WL,MCH,540,5050404
movl $0x05050404, %eax
movl %eax, 0xfed10540
//WL,MCH,544,b090706
movl $0x0b090706, %eax
movl %eax, 0xfed10544
//WL,MCH,548,13110f0d
movl $0x13110f0d, %eax
movl %eax, 0xfed10548
//WL,MCH,54c,1d1b1915
movl $0x1d1b1915, %eax
movl %eax, 0xfed1054c
//WL,MCH,550,1f1f1f1f
movl $0x1f1f1f1f, %eax
movl %eax, 0xfed10550
//WL,MCH,554,1f1f1f1f
movl $0x1f1f1f1f, %eax
movl %eax, 0xfed10554
//WL,MCH,558,1f1f1f1f
movl $0x1f1f1f1f, %eax
movl %eax, 0xfed10558
//WL,MCH,55c,1f1f1f1f
movl $0x1f1f1f1f, %eax
movl %eax, 0xfed1055c
//WL,MCH,560,e0e0d0d
movl $0x0e0e0d0d, %eax
movl %eax, 0xfed10560
//WL,MCH,564,100f0f0f
movl $0x100f0f0f, %eax
movl %eax, 0xfed10564
//WL,MCH,568,1b191310
movl $0x1b191310, %eax
movl %eax, 0xfed10568
//WL,MCH,56c,1f1f1f1d
movl $0x1f1f1f1d, %eax
movl %eax, 0xfed1056c
//WL,MCH,570,1f1f1f1f
movl $0x1f1f1f1f, %eax
movl %eax, 0xfed10570
//WL,MCH,574,1f1f1f1f
movl $0x1f1f1f1f, %eax
movl %eax, 0xfed10574
//WL,MCH,578,1f1f1f1f
movl $0x1f1f1f1f, %eax
movl %eax, 0xfed10578
//WL,MCH,57c,1f1f1f1f
movl $0x1f1f1f1f, %eax
movl %eax, 0xfed1057c
//WH,MCH,40c,55
movw $0x0055, %ax
movw %ax, 0xfed1040c
//WH,MCH,48c,0
movw $0x0000, %ax
movw %ax, 0xfed1048c
//RL,MCH,400,30800000
movl 0xfed10400, %eax
//WL,MCH,400,18800003
movl $0x18800003, %eax
movl %eax, 0xfed10400
//RL,MCH,400,18800003
movl 0xfed10400, %eax
//WL,MCH,400,19800103
movl $0x19800103, %eax
movl %eax, 0xfed10400
//RL,MCH,2a8,2000000
movl 0xfed102a8, %eax
//WL,MCH,2a8,200000c
movl $0x0200000c, %eax
movl %eax, 0xfed102a8
//RH,MCH,35a,3a00
movw 0xfed1035a, %ax
//WH,MCH,35a,3a00
movw $0x3a00, %ax
movw %ax, 0xfed1035a
//RH,MCH,2f4,7
movw 0xfed102f4, %ax
//WH,MCH,2f4,c
movw $0x000c, %ax
movw %ax, 0xfed102f4
//RL,MCH,2f8,5070f
movl 0xfed102f8, %eax
//WL,MCH,2f8,50a0f
movl $0x00050a0f, %eax
movl %eax, 0xfed102f8
//RH,MCH,35a,3a00
movw 0xfed1035a, %ax
//WH,MCH,35a,3a00
movw $0x3a00, %ax
movw %ax, 0xfed1035a
//WL,MCH,300,24242424
movl $0x24242424, %eax
movl %eax, 0xfed10300
//WL,MCH,304,24242424
movl $0x24242424, %eax
movl %eax, 0xfed10304
//WB,MCH,308,24
movb $0x24, %al
movb %al, 0xfed10308
//WL,MCH,380,24242424
movl $0x24242424, %eax
movl %eax, 0xfed10380
//WL,MCH,384,24242424
movl $0x24242424, %eax
movl %eax, 0xfed10384
//WB,MCH,388,24
movb $0x24, %al
movb %al, 0xfed10388
//WL,MCH,310,24242424
movl $0x24242424, %eax
movl %eax, 0xfed10310
//WL,MCH,314,24242424
movl $0x24242424, %eax
movl %eax, 0xfed10314
//WB,MCH,318,24
movb $0x24, %al
movb %al, 0xfed10318
//WL,MCH,390,24242424
movl $0x24242424, %eax
movl %eax, 0xfed10390
//WL,MCH,394,24242424
movl $0x24242424, %eax
movl %eax, 0xfed10394
//WB,MCH,398,24
movb $0x24, %al
movb %al, 0xfed10398
//WL,MCH,320,24242424
movl $0x24242424, %eax
movl %eax, 0xfed10320
//WL,MCH,324,24242424
movl $0x24242424, %eax
movl %eax, 0xfed10324
//WB,MCH,328,24
movb $0x24, %al
movb %al, 0xfed10328
//WL,MCH,3a0,24242424
movl $0x24242424, %eax
movl %eax, 0xfed103a0
//WL,MCH,3a4,24242424
movl $0x24242424, %eax
movl %eax, 0xfed103a4
//WB,MCH,3a8,24
movb $0x24, %al
movb %al, 0xfed103a8
//WL,MCH,330,24242424
movl $0x24242424, %eax
movl %eax, 0xfed10330
//WL,MCH,334,24242424
movl $0x24242424, %eax
movl %eax, 0xfed10334
//WB,MCH,338,24
movb $0x24, %al
movb %al, 0xfed10338
//WL,MCH,3b0,24242424
movl $0x24242424, %eax
movl %eax, 0xfed103b0
//WL,MCH,3b4,24242424
movl $0x24242424, %eax
movl %eax, 0xfed103b4
//WB,MCH,3b8,24
movb $0x24, %al
movb %al, 0xfed103b8
//RL,MCH,2a8,200000c
movl 0xfed102a8, %eax
//WL,MCH,2a8,200005c
movl $0x0200005c, %eax
movl %eax, 0xfed102a8
//WB,MCH,37c,1
movb $0x01, %al
movb %al, 0xfed1037c
//WB,MCH,3fc,1
movb $0x01, %al
movb %al, 0xfed103fc
//WH,MCH,360,1a5
movw $0x01a5, %ax
movw %ax, 0xfed10360
//RL,MCH,2a8,3ff005c
movl 0xfed102a8, %eax
//WL,MCH,2a8,3ff00fc
movl $0x03ff00fc, %eax
movl %eax, 0xfed102a8
//RL,MCH,2f8,50a0f
movl 0xfed102f8, %eax
//WL,MCH,2f8,50a0f
movl $0x00050a0f, %eax
movl %eax, 0xfed102f8
//RL,MCH,400,18800103
movl 0xfed10400, %eax
//WL,MCH,400,19800103
movl $0x19800103, %eax
movl %eax, 0xfed10400
//RL,MCH,124,2
movl 0xfed10124, %eax
//WL,MCH,124,102
movl $0x00000102, %eax
movl %eax, 0xfed10124
//RL,MCH,168,22049200
movl 0xfed10168, %eax
//WL,MCH,168,33624900
movl $0x33624900, %eax
movl %eax, 0xfed10168
//RL,MCH,16c,28798
movl 0xfed1016c, %eax
//WL,MCH,16c,e0218798
movl $0xe0218798, %eax
movl %eax, 0xfed1016c
//RL,MCH,1e8,22049200
movl 0xfed101e8, %eax
//WL,MCH,1e8,33624900
movl $0x33624900, %eax
movl %eax, 0xfed101e8
//RL,MCH,1ec,28798
movl 0xfed101ec, %eax
//WL,MCH,1ec,e0218798
movl $0xe0218798, %eax
movl %eax, 0xfed101ec
//RL,MCH,284,0
movl 0xfed10284, %eax
//WL,MCH,284,10000000
movl $0x10000000, %eax
movl %eax, 0xfed10284
//RL,MCH,16c,e0218798
movl 0xfed1016c, %eax
//WL,MCH,16c,e0218798
movl $0xe0218798, %eax
movl %eax, 0xfed1016c
//RL,MCH,1ec,e0218798
movl 0xfed101ec, %eax
//WL,MCH,1ec,e0218798
movl $0xe0218798, %eax
movl %eax, 0xfed101ec
//RL,MCH,124,102
movl 0xfed10124, %eax
//WL,MCH,124,502
movl $0x00000502, %eax
movl %eax, 0xfed10124
//RL,MCH,1a4,2
movl 0xfed101a4, %eax
//WL,MCH,1a4,402
movl $0x00000402, %eax
movl %eax, 0xfed101a4
//RL,MCH,284,10000000
movl 0xfed10284, %eax
//WL,MCH,284,10000040
movl $0x10000040, %eax
movl %eax, 0xfed10284
//RL,MCH,284,10000040
movl 0xfed10284, %eax
//WL,MCH,284,10004040
movl $0x10004040, %eax
movl %eax, 0xfed10284
//RL,MCH,40,100
movl 0xfed10040, %eax
//WL,MCH,40,102
movl $0x00000102, %eax
movl %eax, 0xfed10040
//RL,MCH,230,34020000
movl 0xfed10230, %eax
//WL,MCH,230,34020004
movl $0x34020004, %eax
movl %eax, 0xfed10230
//RL,MCH,200,0
movl 0xfed10200, %eax
//WL,MCH,200,400
movl $0x00000400, %eax
movl %eax, 0xfed10200
//RB,MCH,2e0,1
movb 0xfed102e0, %al
//WB,MCH,2e0,1
movb $0x01, %al
movb %al, 0xfed102e0
//RL,MCH,224,3000
movl 0xfed10224, %eax
//WL,MCH,224,1003000
movl $0x01003000, %eax
movl %eax, 0xfed10224
//RL,MCH,128,1f0
movl 0xfed10128, %eax
//WL,MCH,128,11f0
movl $0x000011f0, %eax
movl %eax, 0xfed10128
//RL,MCH,1a8,1f0
movl 0xfed101a8, %eax
//WL,MCH,1a8,11f0
movl $0x000011f0, %eax
movl %eax, 0xfed101a8
//RL,MCH,44,2000
movl 0xfed10044, %eax
//WL,MCH,44,2000
movl $0x00002000, %eax
movl %eax, 0xfed10044
//WL,MCH,620,11101010
movl $0x11101010, %eax
movl %eax, 0xfed10620
//WL,MCH,624,12121111
movl $0x12121111, %eax
movl %eax, 0xfed10624
//WL,MCH,628,15131312
movl $0x15131312, %eax
movl %eax, 0xfed10628
//WL,MCH,62c,1a181716
movl $0x1a181716, %eax
movl %eax, 0xfed1062c
//WL,MCH,630,22201e1c
movl $0x22201e1c, %eax
movl %eax, 0xfed10630
//WL,MCH,634,2a282624
movl $0x2a282624, %eax
movl %eax, 0xfed10634
//WL,MCH,638,2f2e2d2c
movl $0x2f2e2d2c, %eax
movl %eax, 0xfed10638
//WL,MCH,63c,37353331
movl $0x37353331, %eax
movl %eax, 0xfed1063c
//WB,MCH,100,8
movb $0x08, %al
movb %al, 0xfed10100
//WB,MCH,108,33
movb $0x33, %al
movb %al, 0xfed10108
//WB,MCH,108,33
movb $0x33, %al
movb %al, 0xfed10108
//WB,MCH,101,10
movb $0x10, %al
movb %al, 0xfed10101
//WB,MCH,102,10
movb $0x10, %al
movb %al, 0xfed10102
//WB,MCH,109,0
movb $0x00, %al
movb %al, 0xfed10109
//WB,MCH,103,10
movb $0x10, %al
movb %al, 0xfed10103
//WB,MCH,180,10
movb $0x10, %al
movb %al, 0xfed10180
//WB,MCH,188,0
movb $0x00, %al
movb %al, 0xfed10188
//WB,MCH,181,10
movb $0x10, %al
movb %al, 0xfed10181
//WB,MCH,182,10
movb $0x10, %al
movb %al, 0xfed10182
//WB,MCH,189,0
movb $0x00, %al
movb %al, 0xfed10189
//WB,MCH,183,10
movb $0x10, %al
movb %al, 0xfed10183
//WB,PCI,0,0,9c,20
movb $0x20, %al
movb %al, 0xf000009c
//RL,MCH,200,400
movl 0xfed10200, %eax
//WL,MCH,200,10400
movl $0x00010400, %eax
movl %eax, 0xfed10200
//RL,RAM,0
movl 0x00000000, %eax
//RL,MCH,200,10400
movl 0xfed10200, %eax
//WL,MCH,200,20400
movl $0x00020400, %eax
movl %eax, 0xfed10200
//RL,RAM,0
movl 0x00000000, %eax
//RL,MCH,200,20400
movl 0xfed10200, %eax
//WL,MCH,200,240400
movl $0x00240400, %eax
movl %eax, 0xfed10200
//RL,RAM,0
movl 0x00000000, %eax
//RL,MCH,200,240400
movl 0xfed10200, %eax
//WL,MCH,200,440400
movl $0x00440400, %eax
movl %eax, 0xfed10200
//RL,RAM,0
movl 0x00000000, %eax
//RL,MCH,200,440400
movl 0xfed10200, %eax
//WL,MCH,200,40400
movl $0x00040400, %eax
movl %eax, 0xfed10200
//RL,RAM,20
movl 0x00000020, %eax
//RL,MCH,200,40400
movl 0xfed10200, %eax
//WL,MCH,200,30400
movl $0x00030400, %eax
movl %eax, 0xfed10200
//RL,RAM,3a58
movl 0x00003a58, %eax
//RL,MCH,200,30400
movl 0xfed10200, %eax
//WL,MCH,200,20400
movl $0x00020400, %eax
movl %eax, 0xfed10200
//RL,RAM,0
movl 0x00000000, %eax
//RL,MCH,200,20400
movl 0xfed10200, %eax
//WL,MCH,200,60400
movl $0x00060400, %eax
movl %eax, 0xfed10200
//RL,RAM,0
movl 0x00000000, %eax
//RL,MCH,200,60400
movl 0xfed10200, %eax
//WL,MCH,200,30400
movl $0x00030400, %eax
movl %eax, 0xfed10200
//RL,RAM,3258
movl 0x00003258, %eax
//RL,MCH,200,30400
movl 0xfed10200, %eax
//WL,MCH,200,40400
movl $0x00040400, %eax
movl %eax, 0xfed10200
//RL,RAM,1c20
movl 0x00001c20, %eax
//RL,MCH,100,10101008
movl 0xfed10100, %eax
//RL,MCH,200,40400
movl 0xfed10200, %eax
//WL,MCH,200,10400
movl $0x00010400, %eax
movl %eax, 0xfed10200
//RL,RAM,10000000
movl 0x10000000, %eax
//RL,MCH,200,10400
movl 0xfed10200, %eax
//WL,MCH,200,20400
movl $0x00020400, %eax
movl %eax, 0xfed10200
//RL,RAM,10000000
movl 0x10000000, %eax
//RL,MCH,200,20400
movl 0xfed10200, %eax
//WL,MCH,200,240400
movl $0x00240400, %eax
movl %eax, 0xfed10200
//RL,RAM,10000000
movl 0x10000000, %eax
//RL,MCH,200,240400
movl 0xfed10200, %eax
//WL,MCH,200,440400
movl $0x00440400, %eax
movl %eax, 0xfed10200
//RL,RAM,10000000
movl 0x10000000, %eax
//RL,MCH,200,440400
movl 0xfed10200, %eax
//WL,MCH,200,40400
movl $0x00040400, %eax
movl %eax, 0xfed10200
//RL,RAM,10000020
movl 0x10000020, %eax
//RL,MCH,200,40400
movl 0xfed10200, %eax
//WL,MCH,200,30400
movl $0x00030400, %eax
movl %eax, 0xfed10200
//RL,RAM,10003a58
movl 0x10003a58, %eax
//RL,MCH,200,30400
movl 0xfed10200, %eax
//WL,MCH,200,20400
movl $0x00020400, %eax
movl %eax, 0xfed10200
//RL,RAM,10000000
movl 0x10000000, %eax
//RL,MCH,200,20400
movl 0xfed10200, %eax
//WL,MCH,200,60400
movl $0x00060400, %eax
movl %eax, 0xfed10200
//RL,RAM,10000000
movl 0x10000000, %eax
//RL,MCH,200,60400
movl 0xfed10200, %eax
//WL,MCH,200,30400
movl $0x00030400, %eax
movl %eax, 0xfed10200
//RL,RAM,10003258
movl 0x10003258, %eax
//RL,MCH,200,30400
movl 0xfed10200, %eax
//WL,MCH,200,40400
movl $0x00040400, %eax
movl %eax, 0xfed10200
//RL,RAM,10001c20
movl 0x10001c20, %eax
//RL,MCH,101,101010
movl 0xfed10101, %eax
//RL,MCH,102,1010
movl 0xfed10102, %eax
//RL,MCH,103,10
movl 0xfed10103, %eax
//RL,MCH,180,10101010
movl 0xfed10180, %eax
//RL,MCH,181,101010
movl 0xfed10181, %eax
//RL,MCH,182,1010
movl 0xfed10182, %eax
//RL,MCH,183,10
movl 0xfed10183, %eax
//WB,MCH,100,20
movb $0x20, %al
movb %al, 0xfed10100
//WB,MCH,108,33
movb $0x33, %al
movb %al, 0xfed10108
//WB,MCH,108,33
movb $0x33, %al
movb %al, 0xfed10108
//WB,MCH,101,40
movb $0x40, %al
movb %al, 0xfed10101
//WB,MCH,102,40
movb $0x40, %al
movb %al, 0xfed10102
//WB,MCH,109,0
movb $0x00, %al
movb %al, 0xfed10109
//WB,MCH,103,40
movb $0x40, %al
movb %al, 0xfed10103
//WB,MCH,180,40
movb $0x40, %al
movb %al, 0xfed10180
//WB,MCH,188,0
movb $0x00, %al
movb %al, 0xfed10188
//WB,MCH,181,40
movb $0x40, %al
movb %al, 0xfed10181
//WB,MCH,182,40
movb $0x40, %al
movb %al, 0xfed10182
//WB,MCH,189,0
movb $0x00, %al
movb %al, 0xfed10189
//WB,MCH,183,40
movb $0x40, %al
movb %al, 0xfed10183
//WB,PCI,0,0,9c,80
movb $0x80, %al
movb %al, 0xf000009c
//RL,MCH,40,102
movl 0xfed10040, %eax
//WL,MCH,40,100
movl $0x00000100, %eax
movl %eax, 0xfed10040
//RL,MCH,230,34020004
movl 0xfed10230, %eax
//WL,MCH,230,34000000
movl $0x34000000, %eax
movl %eax, 0xfed10230
//WL,MCH,130,6c4
movl $0x000006c4, %eax
movl %eax, 0xfed10130
//WL,MCH,134,871a066d
movl $0x871a066d, %eax
movl %eax, 0xfed10134
//WL,MCH,1b0,6c4
movl $0x000006c4, %eax
movl %eax, 0xfed101b0
//WL,MCH,1b4,871a066d
movl $0x871a066d, %eax
movl %eax, 0xfed101b4
//RL,MCH,218,a4000000
movl 0xfed10218, %eax
//WL,MCH,218,a2c00400
movl $0xa2c00400, %eax
movl %eax, 0xfed10218
//RL,MCH,220,3000364
movl 0xfed10220, %eax
//WL,MCH,220,3000364
movl $0x03000364, %eax
movl %eax, 0xfed10220
//RL,MCH,218,a2c00400
movl 0xfed10218, %eax
//WL,MCH,218,a2c00400
movl $0xa2c00400, %eax
movl %eax, 0xfed10218
//RL,MCH,124,502
movl 0xfed10124, %eax
//WL,MCH,124,502
movl $0x00000502, %eax
movl %eax, 0xfed10124
//RL,MCH,124,502
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,1a4,402
movl 0xfed101a4, %eax
//WL,MCH,1a4,402
movl $0x00000402, %eax
movl %eax, 0xfed101a4
//RL,MCH,1a4,402
movl 0xfed101a4, %eax
//WL,MCH,1a4,402
movl $0x00000402, %eax
movl %eax, 0xfed101a4
//RL,MCH,120,40000806
movl 0xfed10120, %eax
//WL,MCH,120,40000a06
movl $0x40000a06, %eax
movl %eax, 0xfed10120
//RL,MCH,1a0,40000806
movl 0xfed101a0, %eax
//WL,MCH,1a0,40000a06
movl $0x40000a06, %eax
movl %eax, 0xfed101a0
//RL,MCH,200,40400
movl 0xfed10200, %eax
//WL,MCH,200,f0400
movl $0x000f0400, %eax
movl %eax, 0xfed10200
//RL,MCH,2f8,50a0f
movl 0xfed102f8, %eax
//WL,MCH,2f8,50acf
movl $0x00050acf, %eax
movl %eax, 0xfed102f8
//RL,MCH,120,40000a06
movl 0xfed10120, %eax
//WL,MCH,120,40000a06
movl $0x40000a06, %eax
movl %eax, 0xfed10120
//RL,MCH,1a0,40000a06
movl 0xfed101a0, %eax
//WL,MCH,1a0,40000a06
movl $0x40000a06, %eax
movl %eax, 0xfed101a0
//WB,MCH,340,0
movb $0x00, %al
movb %al, 0xfed10340
//RL,MCH,114,2618922
movl 0xfed10114, %eax
//WL,MCH,114,4618922
movl $0x04618922, %eax
movl %eax, 0xfed10114
//RL,MCH,2f8,50acf
movl 0xfed102f8, %eax
//WL,MCH,2f8,50acf
movl $0x00050acf, %eax
movl %eax, 0xfed102f8
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,33f
movl 0xfed104ec, %eax
//WB,MCH,340,80
movb $0x80, %al
movb %al, 0xfed10340
//RL,MCH,114,4618922
movl 0xfed10114, %eax
//WL,MCH,114,4618922
movl $0x04618922, %eax
movl %eax, 0xfed10114
//RL,MCH,2f8,60acf
movl 0xfed102f8, %eax
//WL,MCH,2f8,60ac7
movl $0x00060ac7, %eax
movl %eax, 0xfed102f8
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,3
movl 0xfed104ec, %eax
//WB,MCH,340,0
movb $0x00, %al
movb %al, 0xfed10340
//RL,MCH,114,4618922
movl 0xfed10114, %eax
//WL,MCH,114,4618922
movl $0x04618922, %eax
movl %eax, 0xfed10114
//RL,MCH,2f8,40ac7
movl 0xfed102f8, %eax
//WL,MCH,2f8,40ac7
movl $0x00040ac7, %eax
movl %eax, 0xfed102f8
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//RL,MCH,114,4618922
movl 0xfed10114, %eax
//WL,MCH,114,4618922
movl $0x04618922, %eax
movl %eax, 0xfed10114
//RL,MCH,2f8,50ac7
movl 0xfed102f8, %eax
//WL,MCH,2f8,50ac7
movl $0x00050ac7, %eax
movl %eax, 0xfed102f8
//WB,MCH,340,0
movb $0x00, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,1
movb $0x01, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,2
movb $0x02, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,3
movb $0x03, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,4
movb $0x04, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,5
movb $0x05, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,6
movb $0x06, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,7
movb $0x07, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,10
movb $0x10, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,11
movb $0x11, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,12
movb $0x12, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,13
movb $0x13, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,14
movb $0x14, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,15
movb $0x15, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,16
movb $0x16, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,17
movb $0x17, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,20
movb $0x20, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,21
movb $0x21, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,22
movb $0x22, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,23
movb $0x23, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,24
movb $0x24, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,25
movb $0x25, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,26
movb $0x26, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,27
movb $0x27, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,30
movb $0x30, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,31
movb $0x31, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,32
movb $0x32, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,33
movb $0x33, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,34
movb $0x34, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,35
movb $0x35, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,36
movb $0x36, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,37
movb $0x37, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,40
movb $0x40, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,41
movb $0x41, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,42
movb $0x42, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,43
movb $0x43, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,44
movb $0x44, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,45
movb $0x45, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,46
movb $0x46, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,47
movb $0x47, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,50
movb $0x50, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,2
movl 0xfed104ec, %eax
//WB,MCH,340,51
movb $0x51, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,2
movl 0xfed104ec, %eax
//WB,MCH,340,52
movb $0x52, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,2
movl 0xfed104ec, %eax
//WB,MCH,340,53
movb $0x53, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,54
movb $0x54, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,3
movl 0xfed104ec, %eax
//WB,MCH,340,55
movb $0x55, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,2
movl 0xfed104ec, %eax
//WB,MCH,340,56
movb $0x56, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,3
movl 0xfed104ec, %eax
//WB,MCH,340,57
movb $0x57, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,2
movl 0xfed104ec, %eax
//WB,MCH,340,60
movb $0x60, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,2
movl 0xfed104ec, %eax
//WB,MCH,340,61
movb $0x61, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,3
movl 0xfed104ec, %eax
//WB,MCH,340,62
movb $0x62, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,3
movl 0xfed104ec, %eax
//WB,MCH,340,63
movb $0x63, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,3
movl 0xfed104ec, %eax
//WB,MCH,340,64
movb $0x64, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,3
movl 0xfed104ec, %eax
//WB,MCH,340,65
movb $0x65, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,3
movl 0xfed104ec, %eax
//WB,MCH,340,66
movb $0x66, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,3
movl 0xfed104ec, %eax
//WB,MCH,340,67
movb $0x67, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,3
movl 0xfed104ec, %eax
//WB,MCH,340,70
movb $0x70, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,3
movl 0xfed104ec, %eax
//WB,MCH,340,61
movb $0x61, %al
movb %al, 0xfed10340
//WB,MCH,340,e1
movb $0xe1, %al
movb %al, 0xfed10340
//RL,MCH,114,4618922
movl 0xfed10114, %eax
//WL,MCH,114,3618922
movl $0x03618922, %eax
movl %eax, 0xfed10114
//RL,MCH,2f8,40ac7
movl 0xfed102f8, %eax
//WL,MCH,2f8,40ac7
movl $0x00040ac7, %eax
movl %eax, 0xfed102f8
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,3
movl 0xfed104ec, %eax
//RL,MCH,114,3618922
movl 0xfed10114, %eax
//WL,MCH,114,2618922
movl $0x02618922, %eax
movl %eax, 0xfed10114
//RL,MCH,2f8,40ac7
movl 0xfed102f8, %eax
//WL,MCH,2f8,40ac7
movl $0x00040ac7, %eax
movl %eax, 0xfed102f8
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,1c
movl 0xfed104ec, %eax
//RL,MCH,114,2618922
movl 0xfed10114, %eax
//WL,MCH,114,2618922
movl $0x02618922, %eax
movl %eax, 0xfed10114
//RL,MCH,2f8,40ac7
movl 0xfed102f8, %eax
//WL,MCH,2f8,40acf
movl $0x00040acf, %eax
movl %eax, 0xfed102f8
//WB,MCH,340,61
movb $0x61, %al
movb %al, 0xfed10340
//WB,MCH,340,e1
movb $0xe1, %al
movb %al, 0xfed10340
//RL,MCH,114,2618922
movl 0xfed10114, %eax
//WL,MCH,114,2618922
movl $0x02618922, %eax
movl %eax, 0xfed10114
//RL,MCH,2f8,40acf
movl 0xfed102f8, %eax
//WL,MCH,2f8,40acf
movl $0x00040acf, %eax
movl %eax, 0xfed102f8
//WB,MCH,340,e1
movb $0xe1, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,e2
movb $0xe2, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,e3
movb $0xe3, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,e4
movb $0xe4, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,e5
movb $0xe5, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,e6
movb $0xe6, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,e7
movb $0xe7, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,f0
movb $0xf0, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,f1
movb $0xf1, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,f2
movb $0xf2, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,f3
movb $0xf3, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,f4
movb $0xf4, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,f5
movb $0xf5, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,f6
movb $0xf6, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,f7
movb $0xf7, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//RL,MCH,114,2618922
movl 0xfed10114, %eax
//WL,MCH,114,3618922
movl $0x03618922, %eax
movl %eax, 0xfed10114
//RL,MCH,2f8,50acf
movl 0xfed102f8, %eax
//WL,MCH,2f8,50ac7
movl $0x00050ac7, %eax
movl %eax, 0xfed102f8
//RL,MCH,114,3618922
movl 0xfed10114, %eax
//WL,MCH,114,3618922
movl $0x03618922, %eax
movl %eax, 0xfed10114
//RL,MCH,2f8,50ac7
movl 0xfed102f8, %eax
//WL,MCH,2f8,50ac7
movl $0x00050ac7, %eax
movl %eax, 0xfed102f8
//WB,MCH,340,0
movb $0x00, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,1
movb $0x01, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,2
movb $0x02, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,3
movb $0x03, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,4
movb $0x04, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,5
movb $0x05, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,6
movb $0x06, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,7
movb $0x07, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,10
movb $0x10, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,11
movb $0x11, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,12
movb $0x12, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,13
movb $0x13, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,14
movb $0x14, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,15
movb $0x15, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,16
movb $0x16, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,17
movb $0x17, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,20
movb $0x20, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,21
movb $0x21, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,22
movb $0x22, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,23
movb $0x23, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,24
movb $0x24, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,25
movb $0x25, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,26
movb $0x26, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,27
movb $0x27, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,30
movb $0x30, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,31
movb $0x31, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,32
movb $0x32, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,33
movb $0x33, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,34
movb $0x34, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,35
movb $0x35, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,36
movb $0x36, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,37
movb $0x37, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,40
movb $0x40, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,41
movb $0x41, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,42
movb $0x42, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,43
movb $0x43, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,44
movb $0x44, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,45
movb $0x45, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,46
movb $0x46, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,47
movb $0x47, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,50
movb $0x50, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,51
movb $0x51, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,52
movb $0x52, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,2
movl 0xfed104ec, %eax
//WB,MCH,340,53
movb $0x53, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,54
movb $0x54, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,55
movb $0x55, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,0
movl 0xfed104ec, %eax
//WB,MCH,340,56
movb $0x56, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,2
movl 0xfed104ec, %eax
//WB,MCH,340,57
movb $0x57, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,2
movl 0xfed104ec, %eax
//WB,MCH,340,60
movb $0x60, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,2
movl 0xfed104ec, %eax
//WB,MCH,340,61
movb $0x61, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,2
movl 0xfed104ec, %eax
//WB,MCH,340,62
movb $0x62, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,2
movl 0xfed104ec, %eax
//WB,MCH,340,63
movb $0x63, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,3
movl 0xfed104ec, %eax
//WB,MCH,340,64
movb $0x64, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,3
movl 0xfed104ec, %eax
//WB,MCH,340,65
movb $0x65, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,3
movl 0xfed104ec, %eax
//WB,MCH,340,66
movb $0x66, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,3
movl 0xfed104ec, %eax
//WB,MCH,340,67
movb $0x67, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,3
movl 0xfed104ec, %eax
//WB,MCH,340,70
movb $0x70, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,3
movl 0xfed104ec, %eax
//WB,MCH,340,71
movb $0x71, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,3
movl 0xfed104ec, %eax
//WB,MCH,340,72
movb $0x72, %al
movb %al, 0xfed10340
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,3
movl 0xfed104ec, %eax
//WB,MCH,340,63
movb $0x63, %al
movb %al, 0xfed10340
//RL,MCH,114,3618922
movl 0xfed10114, %eax
//WL,MCH,114,3618922
movl $0x03618922, %eax
movl %eax, 0xfed10114
//RL,MCH,2f8,40ac7
movl 0xfed102f8, %eax
//WL,MCH,2f8,40ac7
movl $0x00040ac7, %eax
movl %eax, 0xfed102f8
//WB,MCH,341,63
movb $0x63, %al
movb %al, 0xfed10341
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,e
movl 0xfed104ec, %eax
//WB,MCH,341,64
movb $0x64, %al
movb %al, 0xfed10341
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,f
movl 0xfed104ec, %eax
//WB,MCH,341,65
movb $0x65, %al
movb %al, 0xfed10341
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,e
movl 0xfed104ec, %eax
//WB,MCH,341,66
movb $0x66, %al
movb %al, 0xfed10341
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,e
movl 0xfed104ec, %eax
//WB,MCH,341,67
movb $0x67, %al
movb %al, 0xfed10341
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,f
movl 0xfed104ec, %eax
//WB,MCH,341,70
movb $0x70, %al
movb %al, 0xfed10341
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,e
movl 0xfed104ec, %eax
//WB,MCH,341,71
movb $0x71, %al
movb %al, 0xfed10341
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,f
movl 0xfed104ec, %eax
//WB,MCH,341,72
movb $0x72, %al
movb %al, 0xfed10341
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,e
movl 0xfed104ec, %eax
//WB,MCH,341,63
movb $0x63, %al
movb %al, 0xfed10341
//RL,MCH,114,3618922
movl 0xfed10114, %eax
//WL,MCH,114,3618922
movl $0x03618922, %eax
movl %eax, 0xfed10114
//RL,MCH,2f8,40ac7
movl 0xfed102f8, %eax
//WL,MCH,2f8,40ac7
movl $0x00040ac7, %eax
movl %eax, 0xfed102f8
//WB,MCH,342,63
movb $0x63, %al
movb %al, 0xfed10342
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,3f
movl 0xfed104ec, %eax
//WB,MCH,342,64
movb $0x64, %al
movb %al, 0xfed10342
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,3f
movl 0xfed104ec, %eax
//WB,MCH,342,65
movb $0x65, %al
movb %al, 0xfed10342
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,e
movl 0xfed104ec, %eax
//WB,MCH,342,66
movb $0x66, %al
movb %al, 0xfed10342
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,3e
movl 0xfed104ec, %eax
//WB,MCH,342,67
movb $0x67, %al
movb %al, 0xfed10342
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,3f
movl 0xfed104ec, %eax
//WB,MCH,342,70
movb $0x70, %al
movb %al, 0xfed10342
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,3f
movl 0xfed104ec, %eax
//WB,MCH,342,71
movb $0x71, %al
movb %al, 0xfed10342
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,3e
movl 0xfed104ec, %eax
//WB,MCH,342,72
movb $0x72, %al
movb %al, 0xfed10342
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,3f
movl 0xfed104ec, %eax
//WB,MCH,342,73
movb $0x73, %al
movb %al, 0xfed10342
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,3f
movl 0xfed104ec, %eax
//WB,MCH,342,74
movb $0x74, %al
movb %al, 0xfed10342
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,3f
movl 0xfed104ec, %eax
//WB,MCH,342,75
movb $0x75, %al
movb %al, 0xfed10342
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,3f
movl 0xfed104ec, %eax
//WB,MCH,342,66
movb $0x66, %al
movb %al, 0xfed10342
//RL,MCH,114,3618922
movl 0xfed10114, %eax
//WL,MCH,114,3618922
movl $0x03618922, %eax
movl %eax, 0xfed10114
//RL,MCH,2f8,40ac7
movl 0xfed102f8, %eax
//WL,MCH,2f8,40ac7
movl $0x00040ac7, %eax
movl %eax, 0xfed102f8
//WB,MCH,343,66
movb $0x66, %al
movb %al, 0xfed10343
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,33f
movl 0xfed104ec, %eax
//WB,MCH,343,67
movb $0x67, %al
movb %al, 0xfed10343
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,33f
movl 0xfed104ec, %eax
//WB,MCH,343,70
movb $0x70, %al
movb %al, 0xfed10343
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,33f
movl 0xfed104ec, %eax
//WB,MCH,343,71
movb $0x71, %al
movb %al, 0xfed10343
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,33f
movl 0xfed104ec, %eax
//WB,MCH,343,72
movb $0x72, %al
movb %al, 0xfed10343
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,33f
movl 0xfed104ec, %eax
//WB,MCH,343,73
movb $0x73, %al
movb %al, 0xfed10343
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,33f
movl 0xfed104ec, %eax
//WB,MCH,343,74
movb $0x74, %al
movb %al, 0xfed10343
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,33f
movl 0xfed104ec, %eax
//WB,MCH,343,75
movb $0x75, %al
movb %al, 0xfed10343
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,4ec,33f
movl 0xfed104ec, %eax
//WB,MCH,343,66
movb $0x66, %al
movb %al, 0xfed10343
//RL,MCH,114,3618922
movl 0xfed10114, %eax
//WL,MCH,114,2618922
movl $0x02618922, %eax
movl %eax, 0xfed10114
//RL,MCH,2f8,60ac7
movl 0xfed102f8, %eax
//WL,MCH,2f8,60acf
movl $0x00060acf, %eax
movl %eax, 0xfed102f8
//WB,MCH,340,76
movb $0x76, %al
movb %al, 0xfed10340
//WB,MCH,341,76
movb $0x76, %al
movb %al, 0xfed10341
//WB,MCH,342,81
movb $0x81, %al
movb %al, 0xfed10342
//WB,MCH,343,81
movb $0x81, %al
movb %al, 0xfed10343
//RL,MCH,340,81817676
movl 0xfed10340, %eax
//RL,MCH,124,80000502
movl 0xfed10124, %eax
//WL,MCH,124,80000542
movl $0x80000542, %eax
movl %eax, 0xfed10124
//RL,MCH,124,80000542
movl 0xfed10124, %eax
//WL,MCH,124,80000502
movl $0x80000502, %eax
movl %eax, 0xfed10124
//RL,MCH,1a4,402
movl 0xfed101a4, %eax
//WL,MCH,1a4,442
movl $0x00000442, %eax
movl %eax, 0xfed101a4
//RL,MCH,1a4,442
movl 0xfed101a4, %eax
//WL,MCH,1a4,402
movl $0x00000402, %eax
movl %eax, 0xfed101a4
//RL,MCH,400,18800103
movl 0xfed10400, %eax
//WL,MCH,400,18000103
movl $0x18000103, %eax
movl %eax, 0xfed10400
//RB,MCH,183,40
movb 0xfed10183, %al
//RB,MCH,183,40
movb 0xfed10183, %al
//WB,PCI,0,0,9c,80
movb $0x80, %al
movb %al, 0xf000009c

	jmp	*%ebp

	.size	raminit, .-raminit
