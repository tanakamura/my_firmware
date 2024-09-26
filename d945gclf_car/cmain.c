#define UART_BASE 0x3f8
#define UART_DATA (UART_BASE)
#define UART_LSR (UART_BASE + 5)

#include <stdint.h>
#include <immintrin.h>

static inline void outb(uint8_t v, uint16_t port) {
    __asm__ __volatile__("outb %b[val], %w[port]"
                         :
                         : [val] "a"(v), [port] "Nd"(port));
}

static inline uint8_t inb(uint16_t port) {
    uint8_t ret;
    __asm__ __volatile__("inb %w[port], %b[val]"
                         : [val] "=a"(ret)
                         : [port] "Nd"(port));
    return ret;
}

int putchar(int c) {
    while (1) {
        if (inb(UART_LSR) & (1 << 5)) { /* (1<<5) : THRE */
            outb(c, UART_DATA);
            return 0;
        }
    }
}

int puts(const char *str) {
    for (int i=0; str[i]; i++) {
        putchar(str[i]);
    }
    putchar('\r');
    putchar('\n');
    return 0;
}

void print_hex(uint32_t v) {
    const char *table = "0123456789ABCDEF";
    for (int i=7; i>=0; i--) {
        putchar(table[(v>>(i*4))&0xf]);
    }
    putchar('\r');
    putchar('\n');
}

void cmain() {
    const int STACK_SIZE = 16*1024;
    puts("Hello World from C!!!!!! Hello!!!!!!!!");
    volatile uint32_t *ptr = (uint32_t*)(0xc0000);
    for (int i=0; i<(256*1024-STACK_SIZE)/4; i++) {
        ptr[i] = i;
    }
    for (int i=0; i<(256*1024-STACK_SIZE)/4; i++) {
        if (ptr[i] != i) {
            print_hex(i);
            print_hex(ptr[i]);
        }
    }
    puts("DONE");
}
