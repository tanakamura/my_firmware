scriptdir=$(dirname $0)
if [ $# -ne 1 ]; then
    echo "Usage: $0 <bios>"
    exit 1
fi

#SERIAL=-chardev socket,id=monitor,path=/tmp/qemu.socket,server=on -serial chardev:monitor
SERIAL="-serial mon:stdio"
# -d trace:'pci_cfg_*' \



# -M q35 \

qemu-system-i386  \
-d trace:'pci_cfg_write' \
 -display none \
 -nodefaults \
 -readconfig $scriptdir/machine.cfg \
 -m 2G -bios $1 $SERIAL \
