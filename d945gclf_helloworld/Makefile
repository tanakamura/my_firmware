all: rom

rom: init.o link.lds
	ld -T link.lds $< -o $@ -Map rom.map


clean:
	rm -f *.o rom *.map
