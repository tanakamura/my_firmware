all: rom rom_qemu test.bin

CFLAGS=-Os -Wall -fstack-check=no -fno-stack-protector -m32 -fno-pic -fno-pie -fno-PIC
ASFLAGS=-32

OBJS=init.o cache.o print.o
rom: rlib $(OBJS) link.lds
	ld -T link.lds $(OBJS) r/target/i686-unknown-linux-gnu/release/libr.a -o $@ -Map rom.map

OBJS_QEMU=qemu_init.o cache.o print.o
rom_qemu: rlib $(OBJS_QEMU) link.lds
	ld -T link.lds $(OBJS_QEMU) r/target/i686-unknown-linux-gnu/release/libr.a -o $@ -Map rom_qemu.map

rlib:
	cd r/; RUSTFLAGS="-C target-feature=-sse -C relocation-model=static" cargo build --release --target=i686-unknown-linux-gnu
.POHNY: rlib

clean:
	rm -f *.o rom *.map test.bin

test.bin: test.asm
	nasm $< -o $@
