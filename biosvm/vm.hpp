#include <fcntl.h>
#include <linux/kvm.h>
#include <stdio.h>
#include <stdlib.h>
#include <functional>
#include <string.h>
#include <sys/ioctl.h>
#include <sys/mman.h>
#include <sys/stat.h>
#include <unistd.h>

#include <memory>

struct VM;
struct CPU;

void install_int_handler_readlmode(VM *vm, std::function<void(VM *vm, CPU *cpu)> f, int num);

static constexpr int MODE_SPIFLASH = 0;
static constexpr int MODE_SDRAM = 1;
static constexpr int MODE_OPTIONROM = 2;
static constexpr size_t SDRAM_SIZE = 2 * 1024 * 1024 * 1024ULL;
void dump_regs(const CPU *cpu);

struct CPU {
    int vcpu_fd;
    struct kvm_sregs sregs;
    struct kvm_regs regs;
    struct kvm_run *run_data;
    int mode;

    CPU(int kvm_fd, int vm_fd, int mode) : mode(mode) {
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

        if (mode == MODE_SDRAM) {
            sregs.cs.base = 0;
            sregs.cs.selector = 0;
            sregs.cs.limit = 0xffffffff;
            sregs.cs.type = (1 << 3) | 3;  // code, readable, accessed
            sregs.cs.present = 1;
            sregs.cs.dpl = 0;
            sregs.cs.db = 1;  // 32bit
            sregs.cs.s = 1;   // code or data
            sregs.cs.l = 0;   // 32bit
            sregs.cs.g = 1;   // 4k
            sregs.cs.avl = 0;
            sregs.ss = sregs.cs;
            sregs.ss.type = 3;
            sregs.ds = sregs.ss;

            /* PE : cache enabled, protected mode */
            sregs.cr0 = 0x00000010;

            regs.rip = 0x10000000;
            regs.rsp = 0x10000000 + SDRAM_SIZE - 8;

        } else if (mode == MODE_OPTIONROM) {
            sregs.cs.base = 0xc0000;
            sregs.cs.selector = 0xc000;

            sregs.ss.base = 0xf0000;
            sregs.ss.selector = 0xf000;
            sregs.ss.limit = 0xffff;

            regs.rsp = 0;
            regs.rip = 3;
        }
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
    unsigned char *rom = nullptr, *car = nullptr, *sdram = nullptr;
    std::unique_ptr<CPU> cpu;
    size_t rom_size;
    int mode;
    bool forward_to_uart = true;
    std::function<void(VM*,CPU*)> int_handlers[256];

    VM(const char *rom_path, int mode, bool forward_to_uart) : mode(mode), forward_to_uart(forward_to_uart) {
        kvm_fd = open("/dev/kvm", O_RDWR);
        if (kvm_fd < 0) {
            perror("/dev/kvm");
            exit(1);
        }
        struct stat st;
        int r = stat(rom_path, &st);
        if (r < 0) {
            perror(rom_path);
            exit(1);
        }

        rom_size = st.st_size;

        int rom_image = open(rom_path, O_RDONLY);
        if (rom_image < 0) {
            perror(rom_path);
            exit(1);
        }
        rom = (unsigned char *)mmap(0, rom_size, PROT_READ | PROT_WRITE,
                                    MAP_PRIVATE, rom_image, 0);
        if (rom == MAP_FAILED) {
            perror("mmap");
            exit(1);
        }

        vm_fd = ioctl(kvm_fd, KVM_CREATE_VM, NULL);

        if (mode == MODE_SPIFLASH) {
            struct kvm_userspace_memory_region mem = {0};
            mem.slot = 0;
            mem.flags = 0;
            mem.guest_phys_addr = (1ULL << 32) - rom_size;
            mem.memory_size = rom_size;
            mem.userspace_addr = (__u64)rom;
            r = ioctl(vm_fd, KVM_SET_USER_MEMORY_REGION, &mem, NULL);
            if (r < 0) {
                perror("kvm set user memory region");
                exit(1);
            }

            void *p;
            posix_memalign(&p, 4096, 32 * 1024);
            car = (unsigned char *)p;

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

            struct kvm_userspace_memory_region mem_rom16 = {0};
            mem.slot = 3;
            mem.flags = 0;
            mem.guest_phys_addr = 1 * 1024 * 1024 - 16 * 1024;
            mem.memory_size = 16 * 1024;
            mem.userspace_addr = (__u64)(rom + rom_size - 16 * 1024);
            r = ioctl(vm_fd, KVM_SET_USER_MEMORY_REGION, &mem, NULL);
            if (r < 0) {
                perror("kvm set user memory region");
                exit(1);
            }
        } else if (mode == MODE_SDRAM) {
            void *p;
            posix_memalign(&p, 4096, SDRAM_SIZE);
            sdram = (uint8_t *)p;

            struct kvm_userspace_memory_region mem = {0};
            mem.slot = 0;
            mem.flags = 0;
            mem.guest_phys_addr = 0x10000000;
            mem.memory_size = SDRAM_SIZE;
            mem.userspace_addr = (__u64)p;
            r = ioctl(vm_fd, KVM_SET_USER_MEMORY_REGION, &mem, NULL);
            if (r < 0) {
                perror("kvm set user memory region");
                exit(1);
            }
            memset(sdram, 0xf4, SDRAM_SIZE);  // fill halt
            memcpy(sdram, rom, rom_size);
            *(uint32_t *)(&sdram[0xfffff8]) =
                SDRAM_SIZE - 4;  // ret to halt
        } else if (mode == MODE_OPTIONROM) {
            void *p;
            posix_memalign(&p, 4096, 1024*1024);
            sdram = (uint8_t *)p;

            struct kvm_userspace_memory_region mem = {0};
            mem.slot = 0;
            mem.flags = 0;
            mem.guest_phys_addr = 0x0;
            mem.memory_size = 1024*1024;
            mem.userspace_addr = (__u64)p;
            r = ioctl(vm_fd, KVM_SET_USER_MEMORY_REGION, &mem, NULL);
            if (r < 0) {
                perror("kvm set user memory region");
                exit(1);
            }
            memset(sdram, 0xf4, 1024*1024);  // fill halt
            memcpy(sdram+0xc0000, rom, rom_size);

            install_int_handler_readlmode(this, [](VM*vm, CPU*cpu){ puts("int10");}, 0x10);
        }

        cpu = std::make_unique<CPU>(kvm_fd, vm_fd, mode);
    }

    ~VM() {
        close(vm_fd);
        close(kvm_fd);
        munmap(rom, rom_size);
        free(car);
    }

    void emu_reti();
    void emu_push16(uint16_t val);
    void emu_far_ret();
    void emu_far_call(uintptr_t cs, uintptr_t ip);
};

void disasm(const VM *vm, int mode);
struct Connection;
void run(VM *vm, Connection *conn);
void dump_regs(const CPU *cpu);

