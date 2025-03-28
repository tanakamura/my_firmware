import optparse

def show(val):
    bit_table = [
        (56, 8, 'base', False),
        (55, 1, 'g(1=4K)', True),
        (54, 1, 'd/b(1=32bit mode)', True),
        (53, 1, 'l', True),
        (48, 4, 'limit', False),
        (47, 1, 'p', True),
        (45, 2, 'dpl', True),
        (44, 1, 's', True),
        (43, 1, 'e (0=data, 1=code)', True),
        (42, 1, 'dc', True),
        (41, 1, 'rw', True),
        (40, 1, 'a', True),
        (32, 8, 'base', False),
        (16, 16, 'base', False),
        (0, 16, 'limit', False)
    ]

    print('\n\n===val===       : {:x}'.format(val))

    for i in range(0, len(bit_table)):
        if bit_table[i][3]:
            mask = (1 << bit_table[i][1]) - 1
            print('{:16s}: {:x}'.format(bit_table[i][2], (val >> bit_table[i][0]) & mask))

    g = (val >> 55) & 1

    base = (val >> 16) & 0xff_ffff | (((val >> 56) & 0xf)<<28)
    if g:
        print('base            : {:x}'.format(base*4096))
    else:
        print('base            : {:x}'.format(base))

    limit = (val >> 0) & 0xffff | (((val >> 48) & 0xf) << 16)
    if g:
        print('limit           : {:x}'.format(limit*4096+4095))
    else:
        print('limit           : {:x}'.format(limit))



def main():
    show(((0x0)<<52) | (0x0<<48) | (0x9b<<40) | (0xf0000<<16) | (0xffff<<0))
    show(((0x4)<<52) | (0x0<<48) | (0x9b<<40) | (0xf0000<<16) | (0xffff<<0))
    show(((0xc)<<52) | (0xf<<48) | (0x9b<<40) | (0xffff<<0))




if __name__ == '__main__':
    main()
