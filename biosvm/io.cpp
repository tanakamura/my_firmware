#include "io.hpp"

void Connection::open_tty(const char *path) {
    int fd = open(path, O_RDWR);
    if (fd < 0) {
        perror("open_port: Unable to open /dev/ttyUSB0 - ");
        exit(1);
    }

    this->to_mon = dup(fd);
    this->from_mon = fd;
}
