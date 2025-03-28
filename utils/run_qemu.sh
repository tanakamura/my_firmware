if [ $# -ne 1 ]; then
    echo "Usage: $0 <bios>"
    exit 1
fi

qemu-system-i386 -M q35 \
 -chardev socket,id=monitor,path=/tmp/qemu.socket,server=on \
 -m 2G -bios $1 -serial chardev:monitor -vga std 
