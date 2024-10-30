#include <iostream>
#include <string.h>
#include "io.hpp"
#include "vm.hpp"
#include <getopt.h>

int main(int argc, char **argv) {
    int opt;
    int mode = MODE_SPIFLASH;
    bool connect_to_realmachine = false;

    while ((opt = getopt(argc, argv, "m:c")) != -1) {
        switch (opt) {
            case 'm':
                if (strcmp(optarg, "sdram") == 0) {
                    mode = MODE_SDRAM;
                } else {
                    printf("Invalid mode: %s\n", optarg);
                    return 1;
                }
                break;
            case 'c':
                connect_to_realmachine = true;
                break;
        }
    }

    if (optind >= argc) {
        std::cerr << "Usage: " << argv[0] << " <filename>" << std::endl;
        return 1;
    }

    VM vm(argv[optind], mode);
    Connection conn;

    if (connect_to_realmachine) {
        conn.open_tty("/dev/ttyS0");
        conn.init();
    }

    while (1) {
        run(&vm, &conn);
    }
}
