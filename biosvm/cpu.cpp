#include "io.hpp"
#include "vm.hpp"

static void handle_exit_hlt(CPU *cpu) {
    dump_regs(cpu);
    puts("hlt");
    exit(1);
}

void CPU::setup() {}

void run(VM *vm, Connection *conn) {
    vm->cpu->restore_regs_to_vm();
    if (0) {  // single step
        struct kvm_guest_debug single_step = {};
        single_step.control = KVM_GUESTDBG_ENABLE | KVM_GUESTDBG_SINGLESTEP;
        ioctl(vm->cpu->vcpu_fd, KVM_SET_GUEST_DEBUG, &single_step);
        disasm(vm, vm->mode);
        // dump_regs(vm->cpu.get());
    }
    asm volatile("" ::: "memory");
    int r = ioctl(vm->cpu->vcpu_fd, KVM_RUN, NULL);
    asm volatile("" ::: "memory");
    if (r < 0) {
        perror("kvm run");
        exit(1);
    }

    vm->cpu->load_regs_from_vm();

    auto run_data = vm->cpu->run_data;
    bool record = true;
    bool show = true;

    switch (run_data->exit_reason) {
        case KVM_EXIT_HLT:
            handle_exit_hlt(vm->cpu.get());
            break;

        case KVM_EXIT_INTERNAL_ERROR:
            dump_regs(vm->cpu.get());
            printf("exit internal error : %d [",
                   (int)run_data->internal.suberror);
            exit(1);
            break;
        case KVM_EXIT_MMIO: {
            uintptr_t dp = ((uintptr_t)run_data->mmio.data);

            if (run_data->mmio.is_write) {
                switch (run_data->mmio.len) {
                    case 4:

                        if (record) {
                            fprintf(stderr, "WL,%08x,%08x\n",
                                    ((int)run_data->mmio.phys_addr),
                                    *(uint32_t *)dp);
                        }
                        if (show) {
                            printf(
                                "mmio_out32 pc=%08x, sp=%08x, addr=0x%08x, "
                                "val=0x%08x\n",
                                (int)vm->cpu->regs.rip, (int)vm->cpu->regs.rsp,
                                (int)run_data->mmio.phys_addr, *(uint32_t *)dp);
                        }

                        conn->write32(run_data->mmio.phys_addr,
                                      *(uint32_t *)dp);
                        break;

                    case 2:
                        if (record) {
                            fprintf(stderr, "WH,%08x,%08x\n",
                                    ((int)run_data->mmio.phys_addr),
                                    *(uint16_t *)dp);
                        }
                        if (show) {
                            printf("mmio_out16 addr=0x%08x val=0x%08x\n",
                                   (int)run_data->mmio.phys_addr,
                                   *(uint16_t *)dp);
                        }
                        conn->write16(run_data->mmio.phys_addr,
                                      *(uint16_t *)dp);
                        break;

                    case 1:
                        if (record) {
                            fprintf(stderr, "WB,%08x,%08x\n",
                                    ((int)run_data->mmio.phys_addr),
                                    *(uint8_t *)dp);
                        }

                        if (show) {
                            printf(
                                "mmio_out8 pc=%08x, sp=%08x, addr=0x%08x, "
                                "val=0x%08x\n",
                                (int)vm->cpu->regs.rip, (int)vm->cpu->regs.rsp,
                                (int)run_data->mmio.phys_addr, *(uint8_t *)dp);
                        }

                        conn->write8(run_data->mmio.phys_addr, *(uint8_t *)dp);
                        break;

                    default:
                        puts("unknown mmio");
                        exit(1);
                }
            } else {
                uint32_t inv = 0;
                if (show) {
                    printf("mmio_in%d pc=%08x, sp=%08x, addr=0x%08x, val=",
                           run_data->mmio.len * 8, (int)vm->cpu->regs.rip,
                           (int)vm->cpu->regs.rsp,
                           (int)run_data->mmio.phys_addr);

                    fflush(stdout);
                }
                switch (run_data->mmio.len) {
                    case 4:
                        inv = *(uint32_t *)dp =
                            conn->read32(run_data->mmio.phys_addr);
                        if (record) {
                            fprintf(stderr, "RL,%08x,%08x\n",
                                    ((int)run_data->mmio.phys_addr), inv);
                        }
                        break;
                    case 2:
                        inv = *(uint16_t *)dp =
                            conn->read16(run_data->mmio.phys_addr);
                        if (record) {
                            fprintf(stderr, "RH,%08x,%08x\n",
                                    ((int)run_data->mmio.phys_addr), inv);
                        }
                        break;
                    case 1:
                        inv = *(uint8_t *)dp =
                            conn->read8(run_data->mmio.phys_addr);
                        if (record) {
                            fprintf(stderr, "RB,%08x,%08x\n",
                                    ((int)run_data->mmio.phys_addr), inv);
                        }
                        break;

                    default:
                        puts("unknown mmio");
                        exit(1);
                }
                if (show) {
                    printf("0x%08x\n", (int)inv);
                }
                // dump_regs(vm->cpu.get());
            }
        } break;
        case KVM_EXIT_SHUTDOWN:
            printf("kvm exit shutdown\n");
            exit(1);
            break;
        case KVM_EXIT_FAIL_ENTRY:
            printf(
                "fail entry : reason=%llx\n",
                (long long)run_data->fail_entry.hardware_entry_failure_reason);
            exit(1);
            break;
        case KVM_EXIT_DEBUG:
            break;
        case KVM_EXIT_IO: {
            if (vm->mode == MODE_SPIFLASH) {
                uintptr_t dp = ((uintptr_t)run_data) + run_data->io.data_offset;
                if (run_data->io.direction == KVM_EXIT_IO_IN) {
                    uint32_t inv = 0;
                    printf("io_in%d port=0x%04x, val=", run_data->io.size * 8,
                           run_data->io.port);
                    fflush(stdout);
                    switch (run_data->io.size) {
                        case 4:
                            inv = *(uint32_t *)dp =
                                conn->in32(run_data->io.port);
                            if (record) {
                                fprintf(stderr, "IL,%08x,%08x\n",
                                        ((int)run_data->io.port), inv);
                            }
                            break;
                        case 2:
                            inv = *(uint16_t *)dp =
                                conn->in16(run_data->io.port);
                            if (record) {
                                fprintf(stderr, "IH,%08x,%08x\n",
                                        ((int)run_data->io.port), inv);
                            }
                            break;
                        case 1:
                            inv = *(uint8_t *)dp = conn->in8(run_data->io.port);
                            if (record) {
                                fprintf(stderr, "IB,%08x,%08x\n",
                                        ((int)run_data->io.port), inv);
                            }
                            break;

                        default:
                            puts("unknown io");
                            exit(1);
                    }
                    printf("0x%08x\n", (int)inv);
                } else {
                    switch (run_data->io.size) {
                        case 4:
                            if (show) {
                                printf("io_out32 port=0x%04x val=0x%08x\n",
                                       run_data->io.port, *(uint32_t *)dp);
                            }
                            if (record) {
                                fprintf(stderr, "OL,%08x,%08x\n",
                                        ((int)run_data->io.port),
                                        *(uint32_t *)dp);
                            }
                            conn->out32(run_data->io.port, *(uint32_t *)dp);
                            break;

                        case 2:
                            if (show) {
                                printf("io_out16 port=0x%04x val=0x%08x\n",
                                       run_data->io.port, *(uint16_t *)dp);
                            }
                            if (record) {
                                fprintf(stderr, "OH,%08x,%08x\n",
                                        ((int)run_data->io.port),
                                        *(uint16_t *)dp);
                            }

                            conn->out16(run_data->io.port, *(uint16_t *)dp);
                            break;

                        case 1:
                            if (show) {
                                printf(
                                    "io_out8 pc=%08x, sp=%08x, port=0x%04x "
                                    "val=0x%08x\n",
                                    (int)vm->cpu->regs.rip,
                                    (int)vm->cpu->regs.rsp, run_data->io.port,
                                    *(uint8_t *)dp);
                            }
                            if (record) {
                                fprintf(stderr, "OB,%08x,%08x\n",
                                        ((int)run_data->io.port),
                                        *(uint8_t *)dp);
                            }
                            if (run_data->io.port == 0x3f8) {
                                putchar(*(uint8_t *)dp);
                            } else if (run_data->io.port == 0x2e ||
                                       run_data->io.port == 0x2f ||
                                       ((run_data->io.port & 0x3f0) == 0x3f0)) {
                                /* skip superio */
                            } else {
                                conn->out8(run_data->io.port, *(uint8_t *)dp);
                            }
                            break;

                        default:
                            puts("unknown io");
                            exit(1);
                    }
                }
            } else {
                uintptr_t dp = ((uintptr_t)run_data) + run_data->io.data_offset;
                if (run_data->io.direction == KVM_EXIT_IO_IN) {
                    if (run_data->io.port == 0x3fd) {
                        *(uint8_t *)dp = (1<<5);
                    } else {
                        fprintf(stderr, "unknown io_in port=0x%04x\n",
                                run_data->io.port);
                    }
                } else {
                    if (run_data->io.port == 0x3f8) {
                        putchar(*(uint8_t *)dp);
                    } else {
                        fprintf(stderr, "unknown io_out port=0x%04x\n",
                                run_data->io.port);
                    }
                }
            }
        } break;
        default:
            printf("unknown exit %d\n", run_data->exit_reason);
            exit(1);
            break;
    }
}
