#include <iostream>

#include "io.hpp"
#include "vm.hpp"

int main(int argc, char **argv) {
    if (argc < 2) {
        std::cerr << "Usage: " << argv[0] << " <filename>" << std::endl;
        return 1;
    }

    VM vm(argv[1]);
    Connection conn;
    conn.open_tty("/dev/ttyS0");

    conn.init();

    while (1) {
        run(&vm, &conn);
    }
}
