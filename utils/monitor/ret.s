	mov	cx, 15

	sti
loop:
	mov	al, '.'
	mov	dx, 0x3f8
	out	dx, al
	dec	cx
	hlt
	jnz	loop

	cli

	retf
