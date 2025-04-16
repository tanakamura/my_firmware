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
