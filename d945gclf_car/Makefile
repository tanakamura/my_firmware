all: rom

CFLAGS=-Os -Wall -fstack-check=no -fno-stack-protector -m32 -fno-pic -fno-pie -fno-PIC
ASFLAGS=-32

OBJS=init.o cache.o print.o cmain.o
rom: $(OBJS) link.lds
	ld -T link.lds $(OBJS) -o $@ -Map rom.map


clean:
	rm -f *.o rom *.map
