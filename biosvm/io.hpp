/* -*- c++ -*- */

#include <assert.h>
#include <fcntl.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/fcntl.h>
#include <unistd.h>

struct Connection {
    int to_mon;
    int from_mon;

    static int constexpr INIT = 127;

    static int constexpr READ8 = 2;
    static int constexpr READ16 = 3;
    static int constexpr READ32 = 4;

    static int constexpr WRITE8 = 5;
    static int constexpr WRITE16 = 6;
    static int constexpr WRITE32 = 7;

    static int constexpr IN8 = 8;
    static int constexpr IN16 = 9;
    static int constexpr IN32 = 10;

    static int constexpr OUT8 = 11;
    static int constexpr OUT16 = 12;
    static int constexpr OUT32 = 13;

    static int constexpr RDMSR = 14;
    static int constexpr WRMSR = 15;

    void o1(uint8_t v) { write(to_mon, &v, 1); }
    void o2(uint16_t v) { write(to_mon, &v, 2); }
    void o4(uint32_t v) { write(to_mon, &v, 4); }

    void read_all(int fd, void *p, ssize_t rem) {
        uint8_t *pp = (uint8_t*)p;
        while (rem) {
            fd_set fds;
            FD_ZERO(&fds);
            FD_SET(fd, &fds);

            select(fd+1, &fds, nullptr, nullptr, nullptr);

            ssize_t rdsz = read(fd, pp, rem);
            if (rdsz <= 0) {
                perror("read");
                exit(1);
            }
            pp += rdsz;
            rem -= rdsz;
        }
    }

    uint8_t i1() {
        uint8_t ret;
        read_all(from_mon, &ret, 1);
        return ret;
    }
    uint16_t i2() {
        uint16_t ret;
        read_all(from_mon, &ret, 2);
        return ret;
    }
    uint32_t i4() {
        uint32_t ret;
        read_all(from_mon, &ret, 4);
        return ret;
    }

    void open_tty(const char *path);
    uint8_t read8(uint32_t addr) {
        this->o1(READ8);
        this->o4(addr);
        return this->i1();
    }
    uint16_t read16(uint32_t addr) {
        this->o1(READ16);
        this->o4(addr);
        return this->i2();
    }
    uint32_t read32(uint32_t addr) {
        this->o1(READ32);
        this->o4(addr);
        return this->i4();
    }
    void write8(uint32_t addr, uint8_t v) {
        this->o1(WRITE8);
        this->o4(addr);
        this->o1(v);
        auto ack = this->i1();
        assert(ack == 0xfe);
    }
    void write16(uint32_t addr, uint16_t v) {
        this->o1(WRITE16);
        this->o4(addr);
        this->o2(v);
        auto ack = this->i1();
        assert(ack == 0xfe);
    }
    void write32(uint32_t addr, uint32_t v) {
        this->o1(WRITE32);
        this->o4(addr);
        this->o4(v);
        auto ack = this->i1();
        assert(ack == 0xfe);
    }


    uint8_t in8(uint32_t addr) {
        this->o1(IN8);
        this->o2(addr);
        return this->i1();
    }
    uint16_t in16(uint32_t addr) {
        this->o1(IN16);
        this->o2(addr);
        return this->i2();
    }
    uint32_t in32(uint32_t addr) {
        this->o1(IN32);
        this->o2(addr);
        return this->i4();
    }
    void out8(uint32_t addr, uint8_t v) {
        this->o1(OUT8);
        this->o2(addr);
        this->o1(v);
        auto ack = this->i1();
        assert(ack == 0xfe);
    }
    void out16(uint32_t addr, uint16_t v) {
        this->o1(OUT16);
        this->o2(addr);
        this->o2(v);
        auto ack = this->i1();
        assert(ack == 0xfe);
    }
    void out32(uint32_t addr, uint32_t v) {
        this->o1(OUT32);
        this->o2(addr);
        this->o4(v);
        auto ack = this->i1();
        assert(ack == 0xfe);
    }

    void init() {
        this->o1(INIT);
        auto v = this->i1();
        if (v != 1) {
            printf("init fail %d\n", v);
            exit(1);
        }
    }

    ~Connection() {
        close(to_mon);
        close(from_mon);
    }
};
