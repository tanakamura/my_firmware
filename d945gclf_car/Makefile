all: rom

CFLAGS=-Os -Wall -fstack-check=no -fno-stack-protector -m32 -fno-pic -fno-pie -fno-PIC
ASFLAGS=-32

OBJS=init.o cache.o print.o
rom: rlib $(OBJS) link.lds
	ld -T link.lds $(OBJS) r/target/i686-unknown-linux-musl/release/libr.a -o $@ -Map rom.map

rlib:
	cd r/; RUSTFLAGS="-C target-feature=-sse -C relocation-model=static" cargo build --release --target=i686-unknown-linux-musl
.POHNY: rlib

clean:
	rm -f *.o rom *.map
