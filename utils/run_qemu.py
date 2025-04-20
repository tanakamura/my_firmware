import argparse
import subprocess

def main():
    a = argparse.ArgumentParser()

    a.add_argument('-g', action='store_true', help='gdb')
    a.add_argument('-u', action='store_true', help='use uds serial')
    a.add_argument('-m', type=str, help='machine config path')
    a.add_argument('-t', type=str, help='trace pattern')
    a.add_argument('path')

    args = a.parse_args()

    exec_args = ["qemu-system-i386", "-vga", "std", "-M", "q35", "-accel", "kvm", "-m", "2G"]

    if args.g:
        exec_args += ["-s", "-S"]

    if args.u:
        exec_args += ["-chardev","socket,id=monitor,path=/tmp/qemu.socket,server=on,wait=off", "-serial", "chardev:monitor", "-monitor stdio"]

    if args.m:
        exec_args += ["-nodefaults", "-readconfig", args.m]

    if args.t:
        exec_args += ["-d", f"trace:{args.t}"]

    exec_args += ["-bios", args.path]

    print(exec_args)
    subprocess.call(exec_args)


if __name__ == '__main__':
    main()
