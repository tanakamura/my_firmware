#include <linux/kvm.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/mman.h>
#include <fcntl.h>
#include <memory>
#include <sys/ioctl.h>

struct VM;

struct CPU {
    int vcpu_fd;
    struct kvm_sregs sregs;
    struct kvm_regs regs;
    struct kvm_run *run_data;

    CPU(int kvm_fd, int vm_fd) {
        vcpu_fd = ioctl(vm_fd, KVM_CREATE_VCPU, (void *)0);
        if (vcpu_fd < 0) {
            perror("vcpu create");
            exit(1);
        }
        load_regs_from_vm();

        size_t vcpu_region_size = ioctl(kvm_fd, KVM_GET_VCPU_MMAP_SIZE, NULL);

        run_data =
            (struct kvm_run *)mmap(0, vcpu_region_size, PROT_READ | PROT_WRITE,
                                   MAP_SHARED, vcpu_fd, 0);
    }

    void load_regs_from_vm() {
        ioctl(vcpu_fd, KVM_GET_SREGS, &sregs);
        ioctl(vcpu_fd, KVM_GET_REGS, &regs);
    }
    void restore_regs_to_vm() {
        ioctl(vcpu_fd, KVM_SET_SREGS, &sregs);
        ioctl(vcpu_fd, KVM_SET_REGS, &regs);
    }

    ~CPU() { close(vcpu_fd); }

    void setup();
};

struct VM {
    int kvm_fd;
    int vm_fd;
    unsigned char *rom, *car;
    std::unique_ptr<CPU> cpu;

    VM(const char *rom_path) {
        kvm_fd = open("/dev/kvm", O_RDWR);
        if (kvm_fd < 0) {
            perror("/dev/kvm");
            exit(1);
        }

        int rom_image = open(rom_path, O_RDONLY);
        if (rom_image < 0) {
            perror(rom_path);
            exit(1);
        }
        rom = (unsigned char *)mmap(0, 512 * 1024, PROT_READ | PROT_WRITE,
                                    MAP_PRIVATE, rom_image, 0);
        if (rom == MAP_FAILED) {
            perror("mmap");
            exit(1);
        }

        vm_fd = ioctl(kvm_fd, KVM_CREATE_VM, NULL);

        struct kvm_userspace_memory_region mem = {0};
        mem.slot = 0;
        mem.flags = 0;
        mem.guest_phys_addr = (1ULL<<32)-(512*1024);
        mem.memory_size = 512 * 1024;
        mem.userspace_addr = (__u64)rom;
        int r = ioctl(vm_fd, KVM_SET_USER_MEMORY_REGION, &mem, NULL);
        if (r < 0) {
            perror("kvm set user memory region");
            exit(1);
        }

        void *p;
        posix_memalign(&p, 4096, 32 * 1024);
        car = (unsigned char*)p;

        struct kvm_userspace_memory_region mem_car = {0};
        mem.slot = 1;
        mem.flags = 0;
        mem.guest_phys_addr = 0xfefc0000;
        mem.memory_size = 32 * 1024;
        mem.userspace_addr = (__u64)car;
        r = ioctl(vm_fd, KVM_SET_USER_MEMORY_REGION, &mem, NULL);
        if (r < 0) {
            perror("kvm set user memory region");
            exit(1);
        }

        struct kvm_userspace_memory_region mem_car2 = {0};
        mem.slot = 2;
        mem.flags = 0;
        mem.guest_phys_addr = 0xfef00000;
        mem.memory_size = 32 * 1024;
        mem.userspace_addr = (__u64)car;
        r = ioctl(vm_fd, KVM_SET_USER_MEMORY_REGION, &mem, NULL);
        if (r < 0) {
            perror("kvm set user memory region");
            exit(1);
        }

        cpu = std::make_unique<CPU>(kvm_fd, vm_fd);
    }

    ~VM() {
        close(vm_fd);
        close(kvm_fd);
        munmap(rom, 512 * 1024);
        free(car);
    }
};

void disasm(const VM *vm);
struct Connection;
void run(VM *vm, Connection *conn);
void dump_regs(const CPU *cpu);
