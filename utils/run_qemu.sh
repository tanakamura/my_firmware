scriptdir=$(dirname $0)
if [ $# -ne 1 ]; then
    echo "Usage: $0 <bios>"
    exit 1
fi

#SERIAL=-chardev socket,id=monitor,path=/tmp/qemu.socket,server=on -serial chardev:monitor
SERIAL="-serial mon:stdio"
# -d trace:'pci_cfg_*' \



# -M q35 \
# -nodefaults \
# -readconfig $scriptdir/machine.cfg \


qemu-system-i386  \
 -vga std \
 -M q35 \
  -s -S \
  -m 2G -bios $1 $SERIAL \

