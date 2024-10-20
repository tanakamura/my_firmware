use x86::io::{outb};

//const UART_BASE: u16 = 0x3f8;
//const UART_DATA: u16 = UART_BASE;
//const UART_LSR: u16 = UART_BASE + 5;
//
//fn uart_put8(c: u8) {
//    loop {
//        unsafe {
//            if (inb(UART_LSR) & (1 << 5)) != 0 {
//                /* (1<<5) : THRE */
//                outb(UART_DATA, c);
//                return;
//            }
//        }
//    }
//}

unsafe fn test_error() {
    const PMCON2: u32 = 0xa2u32;
    const PMCON3: u32 = 0xa4u32;

    const D31F0_BASE:u32 = (0<<20) | (31<<15) | (0<<12);
    const PMCON2_ADDR:u32 = 0xF000_0000 + D31F0_BASE + PMCON2;
    const PMCON3_ADDR:u32 = 0xF000_0000 + D31F0_BASE + PMCON3;

    let pmcon2 = core::ptr::read_volatile(PMCON2_ADDR as *const u8);

    if (pmcon2 & (1 << 2)) != 0 {
        core::ptr::write_volatile(PMCON2_ADDR as *mut u8, pmcon2);

        let mut pmcon3 = core::ptr::read_volatile(PMCON3_ADDR as *const u8);
        pmcon3 |= 1<<3;
        core::ptr::write_volatile(PMCON3_ADDR as *mut u8, pmcon3);

        outb(0xcf9, 0xa);
        outb(0xcf9, 0xe);
        loop {}
    }
}

pub fn raminit() {
    unsafe {
        test_error();
// set MCH_BASE to 0xfed10000
core::ptr::write_volatile(0xf0000044 as *mut u32, 0xfed10001);
//RB,MCH,c00,1
core::ptr::read_volatile(0xfed10c00 as *const u8);
//WL,PCI,0,0,54,b8000009
core::ptr::write_volatile(0xf0000054 as *mut u32, 0xb8000009);
//RB,PCI,1f,0,a4,1
core::ptr::read_volatile(0xf00f80a4 as *const u8);
//WB,PCI,1f,0,a4,f5
core::ptr::write_volatile(0xf00f80a4 as *mut u8, 0x000000f5);
//RB,PCI,1f,0,a2,f5
core::ptr::read_volatile(0xf00f80a2 as *const u8);
//RB,PCI,1f,0,a4,5
core::ptr::read_volatile(0xf00f80a4 as *const u8);
//RL,MCH,40,100
core::ptr::read_volatile(0xfed10040 as *const u32);
//WL,MCH,40,100
core::ptr::write_volatile(0xfed10040 as *mut u32, 0x00000100);
//RL,MCH,40,100
core::ptr::read_volatile(0xfed10040 as *const u32);
//WL,MCH,40,100
core::ptr::write_volatile(0xfed10040 as *mut u32, 0x00000100);
//RL,MCH,40,100
core::ptr::read_volatile(0xfed10040 as *const u32);
//WL,MCH,bd4,cc
core::ptr::write_volatile(0xfed10bd4 as *mut u32, 0x000000cc);
//WL,MCH,bd0,ccffff00
core::ptr::write_volatile(0xfed10bd0 as *mut u32, 0xccffff00);
//RB,PCI,1f,0,a4,2
core::ptr::read_volatile(0xf00f80a4 as *const u8);
//WB,PCI,1f,0,a4,f5
core::ptr::write_volatile(0xf00f80a4 as *mut u8, 0x000000f5);
//RB,MCH,c00,1
core::ptr::read_volatile(0xfed10c00 as *const u8);
//RL,MCH,f14,101
core::ptr::read_volatile(0xfed10f14 as *const u32);
//RL,MCH,f14,101
core::ptr::read_volatile(0xfed10f14 as *const u32);
//WL,MCH,f14,103
core::ptr::write_volatile(0xfed10f14 as *mut u32, 0x00000103);
//RL,MCH,80,0
core::ptr::read_volatile(0xfed10080 as *const u32);
//WL,MCH,80,10000000
core::ptr::write_volatile(0xfed10080 as *mut u32, 0x10000000);
//RL,MCH,200,3
core::ptr::read_volatile(0xfed10200 as *const u32);
//WL,MCH,200,0
core::ptr::write_volatile(0xfed10200 as *mut u32, 0x00000000);
//RL,MCH,c00,20000001
core::ptr::read_volatile(0xfed10c00 as *const u32);
//WL,MCH,c00,20000021
core::ptr::write_volatile(0xfed10c00 as *mut u32, 0x20000021);
//RL,MCH,c00,20000021
core::ptr::read_volatile(0xfed10c00 as *const u32);
//RB,MCH,c09,0
core::ptr::read_volatile(0xfed10c09 as *const u8);
//WB,MCH,c09,4
core::ptr::write_volatile(0xfed10c09 as *mut u8, 0x00000004);
//WL,MCH,208,1000400
core::ptr::write_volatile(0xfed10208 as *mut u32, 0x01000400);
//WL,MCH,20c,200
core::ptr::write_volatile(0xfed1020c as *mut u32, 0x00000200);
//WL,MCH,138,100401
core::ptr::write_volatile(0xfed10138 as *mut u32, 0x00100401);
//WL,MCH,1b8,100401
core::ptr::write_volatile(0xfed101b8 as *mut u32, 0x00100401);
//WL,MCH,13c,0
core::ptr::write_volatile(0xfed1013c as *mut u32, 0x00000000);
//WL,MCH,1bc,0
core::ptr::write_volatile(0xfed101bc as *mut u32, 0x00000000);
//RB,MCH,c09,4
core::ptr::read_volatile(0xfed10c09 as *const u8);
//WB,MCH,c09,6
core::ptr::write_volatile(0xfed10c09 as *mut u8, 0x00000006);
//RB,MCH,c09,4
core::ptr::read_volatile(0xfed10c09 as *const u8);
//RB,MCH,c09,4
core::ptr::read_volatile(0xfed10c09 as *const u8);
//WB,MCH,c09,0
core::ptr::write_volatile(0xfed10c09 as *mut u8, 0x00000000);
//RH,MCH,10e,0
core::ptr::read_volatile(0xfed1010e as *const u16);
//WH,MCH,10e,0
core::ptr::write_volatile(0xfed1010e as *mut u16, 0x00000000);
//RH,MCH,10e,0
core::ptr::read_volatile(0xfed1010e as *const u16);
//WH,MCH,10e,5
core::ptr::write_volatile(0xfed1010e as *mut u16, 0x00000005);
//RH,MCH,18e,0
core::ptr::read_volatile(0xfed1018e as *const u16);
//WH,MCH,18e,0
core::ptr::write_volatile(0xfed1018e as *mut u16, 0x00000000);
//RL,MCH,114,2483d22
core::ptr::read_volatile(0xfed10114 as *const u32);
//WL,MCH,114,2493d22
core::ptr::write_volatile(0xfed10114 as *mut u32, 0x02493d22);
//RL,MCH,194,2483d22
core::ptr::read_volatile(0xfed10194 as *const u32);
//WL,MCH,194,2493d22
core::ptr::write_volatile(0xfed10194 as *mut u32, 0x02493d22);
//RL,MCH,120,40002802
core::ptr::read_volatile(0xfed10120 as *const u32);
//WL,MCH,120,40000806
core::ptr::write_volatile(0xfed10120 as *mut u32, 0x40000806);
//RL,MCH,1a0,40002802
core::ptr::read_volatile(0xfed101a0 as *const u32);
//WL,MCH,1a0,40000806
core::ptr::write_volatile(0xfed101a0 as *mut u32, 0x40000806);
//RL,MCH,110,a96038e8
core::ptr::read_volatile(0xfed10110 as *const u32);
//WL,MCH,110,b95020e8
core::ptr::write_volatile(0xfed10110 as *mut u32, 0xb95020e8);
//RL,MCH,190,a96038e8
core::ptr::read_volatile(0xfed10190 as *const u32);
//WL,MCH,190,b95020e8
core::ptr::write_volatile(0xfed10190 as *mut u32, 0xb95020e8);
//RL,MCH,114,2493d22
core::ptr::read_volatile(0xfed10114 as *const u32);
//WL,MCH,114,2618922
core::ptr::write_volatile(0xfed10114 as *mut u32, 0x02618922);
//RL,MCH,194,2493d22
core::ptr::read_volatile(0xfed10194 as *const u32);
//WL,MCH,194,2618922
core::ptr::write_volatile(0xfed10194 as *mut u32, 0x02618922);
//RL,MCH,118,800003ff
core::ptr::read_volatile(0xfed10118 as *const u32);
//WL,MCH,118,8000025f
core::ptr::write_volatile(0xfed10118 as *mut u32, 0x8000025f);
//RL,MCH,198,800003ff
core::ptr::read_volatile(0xfed10198 as *const u32);
//WL,MCH,198,8000025f
core::ptr::write_volatile(0xfed10198 as *mut u32, 0x8000025f);
//RL,MCH,220,264
core::ptr::read_volatile(0xfed10220 as *const u32);
//WL,MCH,220,3000364
core::ptr::write_volatile(0xfed10220 as *mut u32, 0x03000364);
//RL,MCH,224,0
core::ptr::read_volatile(0xfed10224 as *const u32);
//WL,MCH,224,43000
core::ptr::write_volatile(0xfed10224 as *mut u32, 0x00043000);
//RL,MCH,224,43000
core::ptr::read_volatile(0xfed10224 as *const u32);
//WL,MCH,224,3000
core::ptr::write_volatile(0xfed10224 as *mut u32, 0x00003000);
//RL,MCH,124,0
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,2
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x00000002);
//RL,MCH,1a4,0
core::ptr::read_volatile(0xfed101a4 as *const u32);
//WL,MCH,1a4,2
core::ptr::write_volatile(0xfed101a4 as *mut u32, 0x00000002);
//RL,MCH,228,100
core::ptr::read_volatile(0xfed10228 as *const u32);
//WL,MCH,228,3100
core::ptr::write_volatile(0xfed10228 as *mut u32, 0x00003100);
//RL,MCH,10c,50000
core::ptr::read_volatile(0xfed1010c as *const u32);
//WL,MCH,10c,50007
core::ptr::write_volatile(0xfed1010c as *mut u32, 0x00050007);
//WB,MCH,410,44
core::ptr::write_volatile(0xfed10410 as *mut u8, 0x00000044);
//WB,MCH,490,44
core::ptr::write_volatile(0xfed10490 as *mut u8, 0x00000044);
//WB,MCH,420,0
core::ptr::write_volatile(0xfed10420 as *mut u8, 0x00000000);
//WB,MCH,428,0
core::ptr::write_volatile(0xfed10428 as *mut u8, 0x00000000);
//WB,MCH,430,44
core::ptr::write_volatile(0xfed10430 as *mut u8, 0x00000044);
//WB,MCH,438,44
core::ptr::write_volatile(0xfed10438 as *mut u8, 0x00000044);
//WB,MCH,418,33
core::ptr::write_volatile(0xfed10418 as *mut u8, 0x00000033);
//WL,MCH,680,8070706
core::ptr::write_volatile(0xfed10680 as *mut u32, 0x08070706);
//RL,MCH,680,8070706
core::ptr::read_volatile(0xfed10680 as *const u32);
//WL,MCH,500,8070706
core::ptr::write_volatile(0xfed10500 as *mut u32, 0x08070706);
//WL,MCH,684,a090908
core::ptr::write_volatile(0xfed10684 as *mut u32, 0x0a090908);
//RL,MCH,684,a090908
core::ptr::read_volatile(0xfed10684 as *const u32);
//WL,MCH,504,a090908
core::ptr::write_volatile(0xfed10504 as *mut u32, 0x0a090908);
//WL,MCH,688,d0c0b0a
core::ptr::write_volatile(0xfed10688 as *mut u32, 0x0d0c0b0a);
//RL,MCH,688,d0c0b0a
core::ptr::read_volatile(0xfed10688 as *const u32);
//WL,MCH,508,d0c0b0a
core::ptr::write_volatile(0xfed10508 as *mut u32, 0x0d0c0b0a);
//WL,MCH,68c,12100f0e
core::ptr::write_volatile(0xfed1068c as *mut u32, 0x12100f0e);
//RL,MCH,68c,12100f0e
core::ptr::read_volatile(0xfed1068c as *const u32);
//WL,MCH,50c,12100f0e
core::ptr::write_volatile(0xfed1050c as *mut u32, 0x12100f0e);
//WL,MCH,690,1a181614
core::ptr::write_volatile(0xfed10690 as *mut u32, 0x1a181614);
//RL,MCH,690,1a181614
core::ptr::read_volatile(0xfed10690 as *const u32);
//WL,MCH,510,1a181614
core::ptr::write_volatile(0xfed10510 as *mut u32, 0x1a181614);
//WL,MCH,694,22201e1c
core::ptr::write_volatile(0xfed10694 as *mut u32, 0x22201e1c);
//RL,MCH,694,22201e1c
core::ptr::read_volatile(0xfed10694 as *const u32);
//WL,MCH,514,22201e1c
core::ptr::write_volatile(0xfed10514 as *mut u32, 0x22201e1c);
//WL,MCH,698,2a282624
core::ptr::write_volatile(0xfed10698 as *mut u32, 0x2a282624);
//RL,MCH,698,2a282624
core::ptr::read_volatile(0xfed10698 as *const u32);
//WL,MCH,518,2a282624
core::ptr::write_volatile(0xfed10518 as *mut u32, 0x2a282624);
//WL,MCH,69c,3934302d
core::ptr::write_volatile(0xfed1069c as *mut u32, 0x3934302d);
//RL,MCH,69c,3934302d
core::ptr::read_volatile(0xfed1069c as *const u32);
//WL,MCH,51c,3934302d
core::ptr::write_volatile(0xfed1051c as *mut u32, 0x3934302d);
//WL,MCH,6a0,a090908
core::ptr::write_volatile(0xfed106a0 as *mut u32, 0x0a090908);
//RL,MCH,6a0,a090908
core::ptr::read_volatile(0xfed106a0 as *const u32);
//WL,MCH,520,a090908
core::ptr::write_volatile(0xfed10520 as *mut u32, 0x0a090908);
//WL,MCH,6a4,c0b0b0a
core::ptr::write_volatile(0xfed106a4 as *mut u32, 0x0c0b0b0a);
//RL,MCH,6a4,c0b0b0a
core::ptr::read_volatile(0xfed106a4 as *const u32);
//WL,MCH,524,c0b0b0a
core::ptr::write_volatile(0xfed10524 as *mut u32, 0x0c0b0b0a);
//WL,MCH,6a8,e0d0d0c
core::ptr::write_volatile(0xfed106a8 as *mut u32, 0x0e0d0d0c);
//RL,MCH,6a8,e0d0d0c
core::ptr::read_volatile(0xfed106a8 as *const u32);
//WL,MCH,528,e0d0d0c
core::ptr::write_volatile(0xfed10528 as *mut u32, 0x0e0d0d0c);
//WL,MCH,6ac,1211100f
core::ptr::write_volatile(0xfed106ac as *mut u32, 0x1211100f);
//RL,MCH,6ac,1211100f
core::ptr::read_volatile(0xfed106ac as *const u32);
//WL,MCH,52c,1211100f
core::ptr::write_volatile(0xfed1052c as *mut u32, 0x1211100f);
//WL,MCH,6b0,19171513
core::ptr::write_volatile(0xfed106b0 as *mut u32, 0x19171513);
//RL,MCH,6b0,19171513
core::ptr::read_volatile(0xfed106b0 as *const u32);
//WL,MCH,530,19171513
core::ptr::write_volatile(0xfed10530 as *mut u32, 0x19171513);
//WL,MCH,6b4,211f1d1b
core::ptr::write_volatile(0xfed106b4 as *mut u32, 0x211f1d1b);
//RL,MCH,6b4,211f1d1b
core::ptr::read_volatile(0xfed106b4 as *const u32);
//WL,MCH,534,211f1d1b
core::ptr::write_volatile(0xfed10534 as *mut u32, 0x211f1d1b);
//WL,MCH,6b8,2d292623
core::ptr::write_volatile(0xfed106b8 as *mut u32, 0x2d292623);
//RL,MCH,6b8,2d292623
core::ptr::read_volatile(0xfed106b8 as *const u32);
//WL,MCH,538,2d292623
core::ptr::write_volatile(0xfed10538 as *mut u32, 0x2d292623);
//WL,MCH,6bc,3f393531
core::ptr::write_volatile(0xfed106bc as *mut u32, 0x3f393531);
//RL,MCH,6bc,3f393531
core::ptr::read_volatile(0xfed106bc as *const u32);
//WL,MCH,53c,3f393531
core::ptr::write_volatile(0xfed1053c as *mut u32, 0x3f393531);
//WL,MCH,580,7070606
core::ptr::write_volatile(0xfed10580 as *mut u32, 0x07070606);
//WL,MCH,584,e0c0a08
core::ptr::write_volatile(0xfed10584 as *mut u32, 0x0e0c0a08);
//WL,MCH,588,17141210
core::ptr::write_volatile(0xfed10588 as *mut u32, 0x17141210);
//WL,MCH,58c,201e1c1a
core::ptr::write_volatile(0xfed1058c as *mut u32, 0x201e1c1a);
//WL,MCH,590,28262422
core::ptr::write_volatile(0xfed10590 as *mut u32, 0x28262422);
//WL,MCH,594,302e2c2a
core::ptr::write_volatile(0xfed10594 as *mut u32, 0x302e2c2a);
//WL,MCH,598,38363432
core::ptr::write_volatile(0xfed10598 as *mut u32, 0x38363432);
//WL,MCH,59c,3f3e3c3a
core::ptr::write_volatile(0xfed1059c as *mut u32, 0x3f3e3c3a);
//WL,MCH,5a0,13131212
core::ptr::write_volatile(0xfed105a0 as *mut u32, 0x13131212);
//WL,MCH,5a4,16151414
core::ptr::write_volatile(0xfed105a4 as *mut u32, 0x16151414);
//WL,MCH,5a8,211d1a18
core::ptr::write_volatile(0xfed105a8 as *mut u32, 0x211d1a18);
//WL,MCH,5ac,28262422
core::ptr::write_volatile(0xfed105ac as *mut u32, 0x28262422);
//WL,MCH,5b0,302e2c2a
core::ptr::write_volatile(0xfed105b0 as *mut u32, 0x302e2c2a);
//WL,MCH,5b4,38363432
core::ptr::write_volatile(0xfed105b4 as *mut u32, 0x38363432);
//WL,MCH,5b8,3f3e3c3a
core::ptr::write_volatile(0xfed105b8 as *mut u32, 0x3f3e3c3a);
//WL,MCH,5bc,3f3f3f3f
core::ptr::write_volatile(0xfed105bc as *mut u32, 0x3f3f3f3f);
//WL,MCH,5c0,7070606
core::ptr::write_volatile(0xfed105c0 as *mut u32, 0x07070606);
//WL,MCH,5c4,e0c0a08
core::ptr::write_volatile(0xfed105c4 as *mut u32, 0x0e0c0a08);
//WL,MCH,5c8,17141210
core::ptr::write_volatile(0xfed105c8 as *mut u32, 0x17141210);
//WL,MCH,5cc,201e1c1a
core::ptr::write_volatile(0xfed105cc as *mut u32, 0x201e1c1a);
//WL,MCH,5d0,28262422
core::ptr::write_volatile(0xfed105d0 as *mut u32, 0x28262422);
//WL,MCH,5d4,302e2c2a
core::ptr::write_volatile(0xfed105d4 as *mut u32, 0x302e2c2a);
//WL,MCH,5d8,38363432
core::ptr::write_volatile(0xfed105d8 as *mut u32, 0x38363432);
//WL,MCH,5dc,3f3e3c3a
core::ptr::write_volatile(0xfed105dc as *mut u32, 0x3f3e3c3a);
//WL,MCH,5e0,13131212
core::ptr::write_volatile(0xfed105e0 as *mut u32, 0x13131212);
//WL,MCH,5e4,16151414
core::ptr::write_volatile(0xfed105e4 as *mut u32, 0x16151414);
//WL,MCH,5e8,211d1a18
core::ptr::write_volatile(0xfed105e8 as *mut u32, 0x211d1a18);
//WL,MCH,5ec,28262422
core::ptr::write_volatile(0xfed105ec as *mut u32, 0x28262422);
//WL,MCH,5f0,302e2c2a
core::ptr::write_volatile(0xfed105f0 as *mut u32, 0x302e2c2a);
//WL,MCH,5f4,38363432
core::ptr::write_volatile(0xfed105f4 as *mut u32, 0x38363432);
//WL,MCH,5f8,3f3e3c3a
core::ptr::write_volatile(0xfed105f8 as *mut u32, 0x3f3e3c3a);
//WL,MCH,5fc,3f3f3f3f
core::ptr::write_volatile(0xfed105fc as *mut u32, 0x3f3f3f3f);
//WL,MCH,600,c0b0b0b
core::ptr::write_volatile(0xfed10600 as *mut u32, 0x0c0b0b0b);
//WL,MCH,604,d0d0c0c
core::ptr::write_volatile(0xfed10604 as *mut u32, 0x0d0d0c0c);
//WL,MCH,608,100f0e0d
core::ptr::write_volatile(0xfed10608 as *mut u32, 0x100f0e0d);
//WL,MCH,60c,15131211
core::ptr::write_volatile(0xfed1060c as *mut u32, 0x15131211);
//WL,MCH,610,1d1b1917
core::ptr::write_volatile(0xfed10610 as *mut u32, 0x1d1b1917);
//WL,MCH,614,2523211f
core::ptr::write_volatile(0xfed10614 as *mut u32, 0x2523211f);
//WL,MCH,618,2a282927
core::ptr::write_volatile(0xfed10618 as *mut u32, 0x2a282927);
//WL,MCH,61c,32302e2c
core::ptr::write_volatile(0xfed1061c as *mut u32, 0x32302e2c);
//WL,MCH,620,9090808
core::ptr::write_volatile(0xfed10620 as *mut u32, 0x09090808);
//WL,MCH,624,c0b0b0a
core::ptr::write_volatile(0xfed10624 as *mut u32, 0x0c0b0b0a);
//WL,MCH,628,100f0e0d
core::ptr::write_volatile(0xfed10628 as *mut u32, 0x100f0e0d);
//WL,MCH,62c,14131211
core::ptr::write_volatile(0xfed1062c as *mut u32, 0x14131211);
//WL,MCH,630,18171615
core::ptr::write_volatile(0xfed10630 as *mut u32, 0x18171615);
//WL,MCH,634,1e1c1a19
core::ptr::write_volatile(0xfed10634 as *mut u32, 0x1e1c1a19);
//WL,MCH,638,26242220
core::ptr::write_volatile(0xfed10638 as *mut u32, 0x26242220);
//WL,MCH,63c,2e2c2a28
core::ptr::write_volatile(0xfed1063c as *mut u32, 0x2e2c2a28);
//WL,MCH,640,c0b0b0b
core::ptr::write_volatile(0xfed10640 as *mut u32, 0x0c0b0b0b);
//WL,MCH,644,d0d0c0c
core::ptr::write_volatile(0xfed10644 as *mut u32, 0x0d0d0c0c);
//WL,MCH,648,100f0e0d
core::ptr::write_volatile(0xfed10648 as *mut u32, 0x100f0e0d);
//WL,MCH,64c,15131211
core::ptr::write_volatile(0xfed1064c as *mut u32, 0x15131211);
//WL,MCH,650,1d1b1917
core::ptr::write_volatile(0xfed10650 as *mut u32, 0x1d1b1917);
//WL,MCH,654,2523211f
core::ptr::write_volatile(0xfed10654 as *mut u32, 0x2523211f);
//WL,MCH,658,2a282927
core::ptr::write_volatile(0xfed10658 as *mut u32, 0x2a282927);
//WL,MCH,65c,32302e2c
core::ptr::write_volatile(0xfed1065c as *mut u32, 0x32302e2c);
//WL,MCH,660,9090808
core::ptr::write_volatile(0xfed10660 as *mut u32, 0x09090808);
//WL,MCH,664,c0b0b0a
core::ptr::write_volatile(0xfed10664 as *mut u32, 0x0c0b0b0a);
//WL,MCH,668,100f0e0d
core::ptr::write_volatile(0xfed10668 as *mut u32, 0x100f0e0d);
//WL,MCH,66c,14131211
core::ptr::write_volatile(0xfed1066c as *mut u32, 0x14131211);
//WL,MCH,670,18171615
core::ptr::write_volatile(0xfed10670 as *mut u32, 0x18171615);
//WL,MCH,674,1e1c1a19
core::ptr::write_volatile(0xfed10674 as *mut u32, 0x1e1c1a19);
//WL,MCH,678,26242220
core::ptr::write_volatile(0xfed10678 as *mut u32, 0x26242220);
//WL,MCH,67c,2e2c2a28
core::ptr::write_volatile(0xfed1067c as *mut u32, 0x2e2c2a28);
//WL,MCH,540,5050404
core::ptr::write_volatile(0xfed10540 as *mut u32, 0x05050404);
//WL,MCH,544,b090706
core::ptr::write_volatile(0xfed10544 as *mut u32, 0x0b090706);
//WL,MCH,548,13110f0d
core::ptr::write_volatile(0xfed10548 as *mut u32, 0x13110f0d);
//WL,MCH,54c,1d1b1915
core::ptr::write_volatile(0xfed1054c as *mut u32, 0x1d1b1915);
//WL,MCH,550,1f1f1f1f
core::ptr::write_volatile(0xfed10550 as *mut u32, 0x1f1f1f1f);
//WL,MCH,554,1f1f1f1f
core::ptr::write_volatile(0xfed10554 as *mut u32, 0x1f1f1f1f);
//WL,MCH,558,1f1f1f1f
core::ptr::write_volatile(0xfed10558 as *mut u32, 0x1f1f1f1f);
//WL,MCH,55c,1f1f1f1f
core::ptr::write_volatile(0xfed1055c as *mut u32, 0x1f1f1f1f);
//WL,MCH,560,e0e0d0d
core::ptr::write_volatile(0xfed10560 as *mut u32, 0x0e0e0d0d);
//WL,MCH,564,100f0f0f
core::ptr::write_volatile(0xfed10564 as *mut u32, 0x100f0f0f);
//WL,MCH,568,1b191310
core::ptr::write_volatile(0xfed10568 as *mut u32, 0x1b191310);
//WL,MCH,56c,1f1f1f1d
core::ptr::write_volatile(0xfed1056c as *mut u32, 0x1f1f1f1d);
//WL,MCH,570,1f1f1f1f
core::ptr::write_volatile(0xfed10570 as *mut u32, 0x1f1f1f1f);
//WL,MCH,574,1f1f1f1f
core::ptr::write_volatile(0xfed10574 as *mut u32, 0x1f1f1f1f);
//WL,MCH,578,1f1f1f1f
core::ptr::write_volatile(0xfed10578 as *mut u32, 0x1f1f1f1f);
//WL,MCH,57c,1f1f1f1f
core::ptr::write_volatile(0xfed1057c as *mut u32, 0x1f1f1f1f);
//WH,MCH,40c,55
core::ptr::write_volatile(0xfed1040c as *mut u16, 0x00000055);
//WH,MCH,48c,0
core::ptr::write_volatile(0xfed1048c as *mut u16, 0x00000000);
//RL,MCH,400,30800000
core::ptr::read_volatile(0xfed10400 as *const u32);
//WL,MCH,400,18800003
core::ptr::write_volatile(0xfed10400 as *mut u32, 0x18800003);
//RL,MCH,400,18800003
core::ptr::read_volatile(0xfed10400 as *const u32);
//WL,MCH,400,19800103
core::ptr::write_volatile(0xfed10400 as *mut u32, 0x19800103);
//RL,MCH,2a8,2000000
core::ptr::read_volatile(0xfed102a8 as *const u32);
//WL,MCH,2a8,200000c
core::ptr::write_volatile(0xfed102a8 as *mut u32, 0x0200000c);
//RH,MCH,35a,3a00
core::ptr::read_volatile(0xfed1035a as *const u16);
//WH,MCH,35a,3a00
core::ptr::write_volatile(0xfed1035a as *mut u16, 0x00003a00);
//RH,MCH,2f4,7
core::ptr::read_volatile(0xfed102f4 as *const u16);
//WH,MCH,2f4,c
core::ptr::write_volatile(0xfed102f4 as *mut u16, 0x0000000c);
//RL,MCH,2f8,5070f
core::ptr::read_volatile(0xfed102f8 as *const u32);
//WL,MCH,2f8,50a0f
core::ptr::write_volatile(0xfed102f8 as *mut u32, 0x00050a0f);
//RH,MCH,35a,3a00
core::ptr::read_volatile(0xfed1035a as *const u16);
//WH,MCH,35a,3a00
core::ptr::write_volatile(0xfed1035a as *mut u16, 0x00003a00);
//WL,MCH,300,24242424
core::ptr::write_volatile(0xfed10300 as *mut u32, 0x24242424);
//WL,MCH,304,24242424
core::ptr::write_volatile(0xfed10304 as *mut u32, 0x24242424);
//WB,MCH,308,24
core::ptr::write_volatile(0xfed10308 as *mut u8, 0x00000024);
//WL,MCH,380,24242424
core::ptr::write_volatile(0xfed10380 as *mut u32, 0x24242424);
//WL,MCH,384,24242424
core::ptr::write_volatile(0xfed10384 as *mut u32, 0x24242424);
//WB,MCH,388,24
core::ptr::write_volatile(0xfed10388 as *mut u8, 0x00000024);
//WL,MCH,310,24242424
core::ptr::write_volatile(0xfed10310 as *mut u32, 0x24242424);
//WL,MCH,314,24242424
core::ptr::write_volatile(0xfed10314 as *mut u32, 0x24242424);
//WB,MCH,318,24
core::ptr::write_volatile(0xfed10318 as *mut u8, 0x00000024);
//WL,MCH,390,24242424
core::ptr::write_volatile(0xfed10390 as *mut u32, 0x24242424);
//WL,MCH,394,24242424
core::ptr::write_volatile(0xfed10394 as *mut u32, 0x24242424);
//WB,MCH,398,24
core::ptr::write_volatile(0xfed10398 as *mut u8, 0x00000024);
//WL,MCH,320,24242424
core::ptr::write_volatile(0xfed10320 as *mut u32, 0x24242424);
//WL,MCH,324,24242424
core::ptr::write_volatile(0xfed10324 as *mut u32, 0x24242424);
//WB,MCH,328,24
core::ptr::write_volatile(0xfed10328 as *mut u8, 0x00000024);
//WL,MCH,3a0,24242424
core::ptr::write_volatile(0xfed103a0 as *mut u32, 0x24242424);
//WL,MCH,3a4,24242424
core::ptr::write_volatile(0xfed103a4 as *mut u32, 0x24242424);
//WB,MCH,3a8,24
core::ptr::write_volatile(0xfed103a8 as *mut u8, 0x00000024);
//WL,MCH,330,24242424
core::ptr::write_volatile(0xfed10330 as *mut u32, 0x24242424);
//WL,MCH,334,24242424
core::ptr::write_volatile(0xfed10334 as *mut u32, 0x24242424);
//WB,MCH,338,24
core::ptr::write_volatile(0xfed10338 as *mut u8, 0x00000024);
//WL,MCH,3b0,24242424
core::ptr::write_volatile(0xfed103b0 as *mut u32, 0x24242424);
//WL,MCH,3b4,24242424
core::ptr::write_volatile(0xfed103b4 as *mut u32, 0x24242424);
//WB,MCH,3b8,24
core::ptr::write_volatile(0xfed103b8 as *mut u8, 0x00000024);
//RL,MCH,2a8,200000c
core::ptr::read_volatile(0xfed102a8 as *const u32);
//WL,MCH,2a8,200005c
core::ptr::write_volatile(0xfed102a8 as *mut u32, 0x0200005c);
//WB,MCH,37c,1
core::ptr::write_volatile(0xfed1037c as *mut u8, 0x00000001);
//WB,MCH,3fc,1
core::ptr::write_volatile(0xfed103fc as *mut u8, 0x00000001);
//WH,MCH,360,1a5
core::ptr::write_volatile(0xfed10360 as *mut u16, 0x000001a5);
//RL,MCH,2a8,3ff005c
core::ptr::read_volatile(0xfed102a8 as *const u32);
//WL,MCH,2a8,3ff00fc
core::ptr::write_volatile(0xfed102a8 as *mut u32, 0x03ff00fc);
//RL,MCH,2f8,50a0f
core::ptr::read_volatile(0xfed102f8 as *const u32);
//WL,MCH,2f8,50a0f
core::ptr::write_volatile(0xfed102f8 as *mut u32, 0x00050a0f);
//RL,MCH,400,18800103
core::ptr::read_volatile(0xfed10400 as *const u32);
//WL,MCH,400,19800103
core::ptr::write_volatile(0xfed10400 as *mut u32, 0x19800103);
//RL,MCH,124,2
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,102
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x00000102);
//RL,MCH,168,22049200
core::ptr::read_volatile(0xfed10168 as *const u32);
//WL,MCH,168,33624900
core::ptr::write_volatile(0xfed10168 as *mut u32, 0x33624900);
//RL,MCH,16c,28798
core::ptr::read_volatile(0xfed1016c as *const u32);
//WL,MCH,16c,e0218798
core::ptr::write_volatile(0xfed1016c as *mut u32, 0xe0218798);
//RL,MCH,1e8,22049200
core::ptr::read_volatile(0xfed101e8 as *const u32);
//WL,MCH,1e8,33624900
core::ptr::write_volatile(0xfed101e8 as *mut u32, 0x33624900);
//RL,MCH,1ec,28798
core::ptr::read_volatile(0xfed101ec as *const u32);
//WL,MCH,1ec,e0218798
core::ptr::write_volatile(0xfed101ec as *mut u32, 0xe0218798);
//RL,MCH,284,0
core::ptr::read_volatile(0xfed10284 as *const u32);
//WL,MCH,284,10000000
core::ptr::write_volatile(0xfed10284 as *mut u32, 0x10000000);
//RL,MCH,16c,e0218798
core::ptr::read_volatile(0xfed1016c as *const u32);
//WL,MCH,16c,e0218798
core::ptr::write_volatile(0xfed1016c as *mut u32, 0xe0218798);
//RL,MCH,1ec,e0218798
core::ptr::read_volatile(0xfed101ec as *const u32);
//WL,MCH,1ec,e0218798
core::ptr::write_volatile(0xfed101ec as *mut u32, 0xe0218798);
//RL,MCH,124,102
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x00000502);
//RL,MCH,1a4,2
core::ptr::read_volatile(0xfed101a4 as *const u32);
//WL,MCH,1a4,402
core::ptr::write_volatile(0xfed101a4 as *mut u32, 0x00000402);
//RL,MCH,284,10000000
core::ptr::read_volatile(0xfed10284 as *const u32);
//WL,MCH,284,10000040
core::ptr::write_volatile(0xfed10284 as *mut u32, 0x10000040);
//RL,MCH,284,10000040
core::ptr::read_volatile(0xfed10284 as *const u32);
//WL,MCH,284,10004040
core::ptr::write_volatile(0xfed10284 as *mut u32, 0x10004040);
//RL,MCH,40,100
core::ptr::read_volatile(0xfed10040 as *const u32);
//WL,MCH,40,102
core::ptr::write_volatile(0xfed10040 as *mut u32, 0x00000102);
//RL,MCH,230,34020000
core::ptr::read_volatile(0xfed10230 as *const u32);
//WL,MCH,230,34020004
core::ptr::write_volatile(0xfed10230 as *mut u32, 0x34020004);
//RL,MCH,200,0
core::ptr::read_volatile(0xfed10200 as *const u32);
//WL,MCH,200,400
core::ptr::write_volatile(0xfed10200 as *mut u32, 0x00000400);
//RB,MCH,2e0,1
core::ptr::read_volatile(0xfed102e0 as *const u8);
//WB,MCH,2e0,1
core::ptr::write_volatile(0xfed102e0 as *mut u8, 0x00000001);
//RL,MCH,224,3000
core::ptr::read_volatile(0xfed10224 as *const u32);
//WL,MCH,224,1003000
core::ptr::write_volatile(0xfed10224 as *mut u32, 0x01003000);
//RL,MCH,128,1f0
core::ptr::read_volatile(0xfed10128 as *const u32);
//WL,MCH,128,11f0
core::ptr::write_volatile(0xfed10128 as *mut u32, 0x000011f0);
//RL,MCH,1a8,1f0
core::ptr::read_volatile(0xfed101a8 as *const u32);
//WL,MCH,1a8,11f0
core::ptr::write_volatile(0xfed101a8 as *mut u32, 0x000011f0);
//RL,MCH,44,2000
core::ptr::read_volatile(0xfed10044 as *const u32);
//WL,MCH,44,2000
core::ptr::write_volatile(0xfed10044 as *mut u32, 0x00002000);
//WL,MCH,620,11101010
core::ptr::write_volatile(0xfed10620 as *mut u32, 0x11101010);
//WL,MCH,624,12121111
core::ptr::write_volatile(0xfed10624 as *mut u32, 0x12121111);
//WL,MCH,628,15131312
core::ptr::write_volatile(0xfed10628 as *mut u32, 0x15131312);
//WL,MCH,62c,1a181716
core::ptr::write_volatile(0xfed1062c as *mut u32, 0x1a181716);
//WL,MCH,630,22201e1c
core::ptr::write_volatile(0xfed10630 as *mut u32, 0x22201e1c);
//WL,MCH,634,2a282624
core::ptr::write_volatile(0xfed10634 as *mut u32, 0x2a282624);
//WL,MCH,638,2f2e2d2c
core::ptr::write_volatile(0xfed10638 as *mut u32, 0x2f2e2d2c);
//WL,MCH,63c,37353331
core::ptr::write_volatile(0xfed1063c as *mut u32, 0x37353331);
//WB,MCH,100,8
core::ptr::write_volatile(0xfed10100 as *mut u8, 0x00000008);
//WB,MCH,108,33
core::ptr::write_volatile(0xfed10108 as *mut u8, 0x00000033);
//WB,MCH,108,33
core::ptr::write_volatile(0xfed10108 as *mut u8, 0x00000033);
//WB,MCH,101,10
core::ptr::write_volatile(0xfed10101 as *mut u8, 0x00000010);
//WB,MCH,102,10
core::ptr::write_volatile(0xfed10102 as *mut u8, 0x00000010);
//WB,MCH,109,0
core::ptr::write_volatile(0xfed10109 as *mut u8, 0x00000000);
//WB,MCH,103,10
core::ptr::write_volatile(0xfed10103 as *mut u8, 0x00000010);
//WB,MCH,180,10
core::ptr::write_volatile(0xfed10180 as *mut u8, 0x00000010);
//WB,MCH,188,0
core::ptr::write_volatile(0xfed10188 as *mut u8, 0x00000000);
//WB,MCH,181,10
core::ptr::write_volatile(0xfed10181 as *mut u8, 0x00000010);
//WB,MCH,182,10
core::ptr::write_volatile(0xfed10182 as *mut u8, 0x00000010);
//WB,MCH,189,0
core::ptr::write_volatile(0xfed10189 as *mut u8, 0x00000000);
//WB,MCH,183,10
core::ptr::write_volatile(0xfed10183 as *mut u8, 0x00000010);
//WB,PCI,0,0,9c,20
core::ptr::write_volatile(0xf000009c as *mut u8, 0x00000020);
//RL,MCH,200,400
core::ptr::read_volatile(0xfed10200 as *const u32);
//WL,MCH,200,10400
core::ptr::write_volatile(0xfed10200 as *mut u32, 0x00010400);
//RL,RAM,0
core::ptr::read_volatile(0x00000000 as *const u32);
//RL,MCH,200,10400
core::ptr::read_volatile(0xfed10200 as *const u32);
//WL,MCH,200,20400
core::ptr::write_volatile(0xfed10200 as *mut u32, 0x00020400);
//RL,RAM,0
core::ptr::read_volatile(0x00000000 as *const u32);
//RL,MCH,200,20400
core::ptr::read_volatile(0xfed10200 as *const u32);
//WL,MCH,200,240400
core::ptr::write_volatile(0xfed10200 as *mut u32, 0x00240400);
//RL,RAM,0
core::ptr::read_volatile(0x00000000 as *const u32);
//RL,MCH,200,240400
core::ptr::read_volatile(0xfed10200 as *const u32);
//WL,MCH,200,440400
core::ptr::write_volatile(0xfed10200 as *mut u32, 0x00440400);
//RL,RAM,0
core::ptr::read_volatile(0x00000000 as *const u32);
//RL,MCH,200,440400
core::ptr::read_volatile(0xfed10200 as *const u32);
//WL,MCH,200,40400
core::ptr::write_volatile(0xfed10200 as *mut u32, 0x00040400);
//RL,RAM,20
core::ptr::read_volatile(0x00000020 as *const u32);
//RL,MCH,200,40400
core::ptr::read_volatile(0xfed10200 as *const u32);
//WL,MCH,200,30400
core::ptr::write_volatile(0xfed10200 as *mut u32, 0x00030400);
//RL,RAM,3a58
core::ptr::read_volatile(0x00003a58 as *const u32);
//RL,MCH,200,30400
core::ptr::read_volatile(0xfed10200 as *const u32);
//WL,MCH,200,20400
core::ptr::write_volatile(0xfed10200 as *mut u32, 0x00020400);
//RL,RAM,0
core::ptr::read_volatile(0x00000000 as *const u32);
//RL,MCH,200,20400
core::ptr::read_volatile(0xfed10200 as *const u32);
//WL,MCH,200,60400
core::ptr::write_volatile(0xfed10200 as *mut u32, 0x00060400);
//RL,RAM,0
core::ptr::read_volatile(0x00000000 as *const u32);
//RL,MCH,200,60400
core::ptr::read_volatile(0xfed10200 as *const u32);
//WL,MCH,200,30400
core::ptr::write_volatile(0xfed10200 as *mut u32, 0x00030400);
//RL,RAM,3258
core::ptr::read_volatile(0x00003258 as *const u32);
//RL,MCH,200,30400
core::ptr::read_volatile(0xfed10200 as *const u32);
//WL,MCH,200,40400
core::ptr::write_volatile(0xfed10200 as *mut u32, 0x00040400);
//RL,RAM,1c20
core::ptr::read_volatile(0x00001c20 as *const u32);
//RL,MCH,100,10101008
core::ptr::read_volatile(0xfed10100 as *const u32);
//RL,MCH,200,40400
core::ptr::read_volatile(0xfed10200 as *const u32);
//WL,MCH,200,10400
core::ptr::write_volatile(0xfed10200 as *mut u32, 0x00010400);
//RL,RAM,10000000
core::ptr::read_volatile(0x10000000 as *const u32);
//RL,MCH,200,10400
core::ptr::read_volatile(0xfed10200 as *const u32);
//WL,MCH,200,20400
core::ptr::write_volatile(0xfed10200 as *mut u32, 0x00020400);
//RL,RAM,10000000
core::ptr::read_volatile(0x10000000 as *const u32);
//RL,MCH,200,20400
core::ptr::read_volatile(0xfed10200 as *const u32);
//WL,MCH,200,240400
core::ptr::write_volatile(0xfed10200 as *mut u32, 0x00240400);
//RL,RAM,10000000
core::ptr::read_volatile(0x10000000 as *const u32);
//RL,MCH,200,240400
core::ptr::read_volatile(0xfed10200 as *const u32);
//WL,MCH,200,440400
core::ptr::write_volatile(0xfed10200 as *mut u32, 0x00440400);
//RL,RAM,10000000
core::ptr::read_volatile(0x10000000 as *const u32);
//RL,MCH,200,440400
core::ptr::read_volatile(0xfed10200 as *const u32);
//WL,MCH,200,40400
core::ptr::write_volatile(0xfed10200 as *mut u32, 0x00040400);
//RL,RAM,10000020
core::ptr::read_volatile(0x10000020 as *const u32);
//RL,MCH,200,40400
core::ptr::read_volatile(0xfed10200 as *const u32);
//WL,MCH,200,30400
core::ptr::write_volatile(0xfed10200 as *mut u32, 0x00030400);
//RL,RAM,10003a58
core::ptr::read_volatile(0x10003a58 as *const u32);
//RL,MCH,200,30400
core::ptr::read_volatile(0xfed10200 as *const u32);
//WL,MCH,200,20400
core::ptr::write_volatile(0xfed10200 as *mut u32, 0x00020400);
//RL,RAM,10000000
core::ptr::read_volatile(0x10000000 as *const u32);
//RL,MCH,200,20400
core::ptr::read_volatile(0xfed10200 as *const u32);
//WL,MCH,200,60400
core::ptr::write_volatile(0xfed10200 as *mut u32, 0x00060400);
//RL,RAM,10000000
core::ptr::read_volatile(0x10000000 as *const u32);
//RL,MCH,200,60400
core::ptr::read_volatile(0xfed10200 as *const u32);
//WL,MCH,200,30400
core::ptr::write_volatile(0xfed10200 as *mut u32, 0x00030400);
//RL,RAM,10003258
core::ptr::read_volatile(0x10003258 as *const u32);
//RL,MCH,200,30400
core::ptr::read_volatile(0xfed10200 as *const u32);
//WL,MCH,200,40400
core::ptr::write_volatile(0xfed10200 as *mut u32, 0x00040400);
//RL,RAM,10001c20
core::ptr::read_volatile(0x10001c20 as *const u32);
//RL,MCH,101,101010
core::ptr::read_volatile(0xfed10101 as *const u32);
//RL,MCH,102,1010
core::ptr::read_volatile(0xfed10102 as *const u32);
//RL,MCH,103,10
core::ptr::read_volatile(0xfed10103 as *const u32);
//RL,MCH,180,10101010
core::ptr::read_volatile(0xfed10180 as *const u32);
//RL,MCH,181,101010
core::ptr::read_volatile(0xfed10181 as *const u32);
//RL,MCH,182,1010
core::ptr::read_volatile(0xfed10182 as *const u32);
//RL,MCH,183,10
core::ptr::read_volatile(0xfed10183 as *const u32);
//WB,MCH,100,20
core::ptr::write_volatile(0xfed10100 as *mut u8, 0x00000020);
//WB,MCH,108,33
core::ptr::write_volatile(0xfed10108 as *mut u8, 0x00000033);
//WB,MCH,108,33
core::ptr::write_volatile(0xfed10108 as *mut u8, 0x00000033);
//WB,MCH,101,40
core::ptr::write_volatile(0xfed10101 as *mut u8, 0x00000040);
//WB,MCH,102,40
core::ptr::write_volatile(0xfed10102 as *mut u8, 0x00000040);
//WB,MCH,109,0
core::ptr::write_volatile(0xfed10109 as *mut u8, 0x00000000);
//WB,MCH,103,40
core::ptr::write_volatile(0xfed10103 as *mut u8, 0x00000040);
//WB,MCH,180,40
core::ptr::write_volatile(0xfed10180 as *mut u8, 0x00000040);
//WB,MCH,188,0
core::ptr::write_volatile(0xfed10188 as *mut u8, 0x00000000);
//WB,MCH,181,40
core::ptr::write_volatile(0xfed10181 as *mut u8, 0x00000040);
//WB,MCH,182,40
core::ptr::write_volatile(0xfed10182 as *mut u8, 0x00000040);
//WB,MCH,189,0
core::ptr::write_volatile(0xfed10189 as *mut u8, 0x00000000);
//WB,MCH,183,40
core::ptr::write_volatile(0xfed10183 as *mut u8, 0x00000040);
//WB,PCI,0,0,9c,80
core::ptr::write_volatile(0xf000009c as *mut u8, 0x00000080);
//RL,MCH,40,102
core::ptr::read_volatile(0xfed10040 as *const u32);
//WL,MCH,40,100
core::ptr::write_volatile(0xfed10040 as *mut u32, 0x00000100);
//RL,MCH,230,34020004
core::ptr::read_volatile(0xfed10230 as *const u32);
//WL,MCH,230,34000000
core::ptr::write_volatile(0xfed10230 as *mut u32, 0x34000000);
//WL,MCH,130,6c4
core::ptr::write_volatile(0xfed10130 as *mut u32, 0x000006c4);
//WL,MCH,134,871a066d
core::ptr::write_volatile(0xfed10134 as *mut u32, 0x871a066d);
//WL,MCH,1b0,6c4
core::ptr::write_volatile(0xfed101b0 as *mut u32, 0x000006c4);
//WL,MCH,1b4,871a066d
core::ptr::write_volatile(0xfed101b4 as *mut u32, 0x871a066d);
//RL,MCH,218,a4000000
core::ptr::read_volatile(0xfed10218 as *const u32);
//WL,MCH,218,a2c00400
core::ptr::write_volatile(0xfed10218 as *mut u32, 0xa2c00400);
//RL,MCH,220,3000364
core::ptr::read_volatile(0xfed10220 as *const u32);
//WL,MCH,220,3000364
core::ptr::write_volatile(0xfed10220 as *mut u32, 0x03000364);
//RL,MCH,218,a2c00400
core::ptr::read_volatile(0xfed10218 as *const u32);
//WL,MCH,218,a2c00400
core::ptr::write_volatile(0xfed10218 as *mut u32, 0xa2c00400);
//RL,MCH,124,502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x00000502);
//RL,MCH,124,502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,1a4,402
core::ptr::read_volatile(0xfed101a4 as *const u32);
//WL,MCH,1a4,402
core::ptr::write_volatile(0xfed101a4 as *mut u32, 0x00000402);
//RL,MCH,1a4,402
core::ptr::read_volatile(0xfed101a4 as *const u32);
//WL,MCH,1a4,402
core::ptr::write_volatile(0xfed101a4 as *mut u32, 0x00000402);
//RL,MCH,120,40000806
core::ptr::read_volatile(0xfed10120 as *const u32);
//WL,MCH,120,40000a06
core::ptr::write_volatile(0xfed10120 as *mut u32, 0x40000a06);
//RL,MCH,1a0,40000806
core::ptr::read_volatile(0xfed101a0 as *const u32);
//WL,MCH,1a0,40000a06
core::ptr::write_volatile(0xfed101a0 as *mut u32, 0x40000a06);
//RL,MCH,200,40400
core::ptr::read_volatile(0xfed10200 as *const u32);
//WL,MCH,200,f0400
core::ptr::write_volatile(0xfed10200 as *mut u32, 0x000f0400);
//RL,MCH,2f8,50a0f
core::ptr::read_volatile(0xfed102f8 as *const u32);
//WL,MCH,2f8,50acf
core::ptr::write_volatile(0xfed102f8 as *mut u32, 0x00050acf);
//RL,MCH,120,40000a06
core::ptr::read_volatile(0xfed10120 as *const u32);
//WL,MCH,120,40000a06
core::ptr::write_volatile(0xfed10120 as *mut u32, 0x40000a06);
//RL,MCH,1a0,40000a06
core::ptr::read_volatile(0xfed101a0 as *const u32);
//WL,MCH,1a0,40000a06
core::ptr::write_volatile(0xfed101a0 as *mut u32, 0x40000a06);
//WB,MCH,340,0
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000000);
//RL,MCH,114,2618922
core::ptr::read_volatile(0xfed10114 as *const u32);
//WL,MCH,114,4618922
core::ptr::write_volatile(0xfed10114 as *mut u32, 0x04618922);
//RL,MCH,2f8,50acf
core::ptr::read_volatile(0xfed102f8 as *const u32);
//WL,MCH,2f8,50acf
core::ptr::write_volatile(0xfed102f8 as *mut u32, 0x00050acf);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,33f
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,80
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000080);
//RL,MCH,114,4618922
core::ptr::read_volatile(0xfed10114 as *const u32);
//WL,MCH,114,4618922
core::ptr::write_volatile(0xfed10114 as *mut u32, 0x04618922);
//RL,MCH,2f8,60acf
core::ptr::read_volatile(0xfed102f8 as *const u32);
//WL,MCH,2f8,60ac7
core::ptr::write_volatile(0xfed102f8 as *mut u32, 0x00060ac7);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,3
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,0
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000000);
//RL,MCH,114,4618922
core::ptr::read_volatile(0xfed10114 as *const u32);
//WL,MCH,114,4618922
core::ptr::write_volatile(0xfed10114 as *mut u32, 0x04618922);
//RL,MCH,2f8,40ac7
core::ptr::read_volatile(0xfed102f8 as *const u32);
//WL,MCH,2f8,40ac7
core::ptr::write_volatile(0xfed102f8 as *mut u32, 0x00040ac7);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//RL,MCH,114,4618922
core::ptr::read_volatile(0xfed10114 as *const u32);
//WL,MCH,114,4618922
core::ptr::write_volatile(0xfed10114 as *mut u32, 0x04618922);
//RL,MCH,2f8,50ac7
core::ptr::read_volatile(0xfed102f8 as *const u32);
//WL,MCH,2f8,50ac7
core::ptr::write_volatile(0xfed102f8 as *mut u32, 0x00050ac7);
//WB,MCH,340,0
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000000);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,1
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000001);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,2
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000002);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,3
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000003);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,4
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000004);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,5
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000005);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,6
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000006);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,7
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000007);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,10
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000010);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,11
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000011);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,12
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000012);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,13
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000013);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,14
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000014);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,15
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000015);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,16
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000016);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,17
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000017);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,20
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000020);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,21
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000021);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,22
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000022);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,23
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000023);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,24
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000024);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,25
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000025);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,26
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000026);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,27
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000027);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,30
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000030);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,31
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000031);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,32
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000032);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,33
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000033);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,34
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000034);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,35
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000035);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,36
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000036);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,37
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000037);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,40
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000040);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,41
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000041);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,42
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000042);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,43
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000043);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,44
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000044);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,45
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000045);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,46
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000046);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,47
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000047);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,50
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000050);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,2
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,51
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000051);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,2
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,52
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000052);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,2
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,53
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000053);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,54
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000054);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,3
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,55
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000055);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,2
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,56
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000056);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,3
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,57
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000057);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,2
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,60
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000060);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,2
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,61
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000061);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,3
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,62
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000062);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,3
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,63
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000063);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,3
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,64
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000064);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,3
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,65
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000065);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,3
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,66
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000066);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,3
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,67
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000067);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,3
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,70
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000070);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,3
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,61
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000061);
//WB,MCH,340,e1
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x000000e1);
//RL,MCH,114,4618922
core::ptr::read_volatile(0xfed10114 as *const u32);
//WL,MCH,114,3618922
core::ptr::write_volatile(0xfed10114 as *mut u32, 0x03618922);
//RL,MCH,2f8,40ac7
core::ptr::read_volatile(0xfed102f8 as *const u32);
//WL,MCH,2f8,40ac7
core::ptr::write_volatile(0xfed102f8 as *mut u32, 0x00040ac7);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,3
core::ptr::read_volatile(0xfed104ec as *const u32);
//RL,MCH,114,3618922
core::ptr::read_volatile(0xfed10114 as *const u32);
//WL,MCH,114,2618922
core::ptr::write_volatile(0xfed10114 as *mut u32, 0x02618922);
//RL,MCH,2f8,40ac7
core::ptr::read_volatile(0xfed102f8 as *const u32);
//WL,MCH,2f8,40ac7
core::ptr::write_volatile(0xfed102f8 as *mut u32, 0x00040ac7);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,1c
core::ptr::read_volatile(0xfed104ec as *const u32);
//RL,MCH,114,2618922
core::ptr::read_volatile(0xfed10114 as *const u32);
//WL,MCH,114,2618922
core::ptr::write_volatile(0xfed10114 as *mut u32, 0x02618922);
//RL,MCH,2f8,40ac7
core::ptr::read_volatile(0xfed102f8 as *const u32);
//WL,MCH,2f8,40acf
core::ptr::write_volatile(0xfed102f8 as *mut u32, 0x00040acf);
//WB,MCH,340,61
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000061);
//WB,MCH,340,e1
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x000000e1);
//RL,MCH,114,2618922
core::ptr::read_volatile(0xfed10114 as *const u32);
//WL,MCH,114,2618922
core::ptr::write_volatile(0xfed10114 as *mut u32, 0x02618922);
//RL,MCH,2f8,40acf
core::ptr::read_volatile(0xfed102f8 as *const u32);
//WL,MCH,2f8,40acf
core::ptr::write_volatile(0xfed102f8 as *mut u32, 0x00040acf);
//WB,MCH,340,e1
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x000000e1);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,e2
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x000000e2);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,e3
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x000000e3);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,e4
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x000000e4);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,e5
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x000000e5);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,e6
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x000000e6);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,e7
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x000000e7);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,f0
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x000000f0);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,f1
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x000000f1);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,f2
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x000000f2);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,f3
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x000000f3);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,f4
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x000000f4);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,f5
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x000000f5);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,f6
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x000000f6);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,f7
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x000000f7);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//RL,MCH,114,2618922
core::ptr::read_volatile(0xfed10114 as *const u32);
//WL,MCH,114,3618922
core::ptr::write_volatile(0xfed10114 as *mut u32, 0x03618922);
//RL,MCH,2f8,50acf
core::ptr::read_volatile(0xfed102f8 as *const u32);
//WL,MCH,2f8,50ac7
core::ptr::write_volatile(0xfed102f8 as *mut u32, 0x00050ac7);
//RL,MCH,114,3618922
core::ptr::read_volatile(0xfed10114 as *const u32);
//WL,MCH,114,3618922
core::ptr::write_volatile(0xfed10114 as *mut u32, 0x03618922);
//RL,MCH,2f8,50ac7
core::ptr::read_volatile(0xfed102f8 as *const u32);
//WL,MCH,2f8,50ac7
core::ptr::write_volatile(0xfed102f8 as *mut u32, 0x00050ac7);
//WB,MCH,340,0
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000000);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,1
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000001);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,2
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000002);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,3
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000003);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,4
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000004);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,5
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000005);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,6
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000006);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,7
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000007);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,10
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000010);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,11
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000011);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,12
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000012);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,13
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000013);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,14
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000014);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,15
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000015);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,16
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000016);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,17
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000017);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,20
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000020);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,21
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000021);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,22
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000022);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,23
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000023);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,24
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000024);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,25
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000025);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,26
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000026);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,27
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000027);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,30
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000030);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,31
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000031);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,32
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000032);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,33
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000033);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,34
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000034);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,35
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000035);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,36
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000036);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,37
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000037);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,40
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000040);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,41
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000041);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,42
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000042);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,43
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000043);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,44
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000044);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,45
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000045);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,46
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000046);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,47
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000047);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,50
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000050);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,51
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000051);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,52
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000052);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,2
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,53
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000053);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,54
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000054);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,55
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000055);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,0
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,56
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000056);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,2
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,57
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000057);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,2
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,60
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000060);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,2
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,61
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000061);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,2
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,62
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000062);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,2
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,63
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000063);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,3
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,64
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000064);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,3
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,65
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000065);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,3
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,66
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000066);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,3
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,67
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000067);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,3
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,70
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000070);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,3
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,71
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000071);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,3
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,72
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000072);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,3
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,340,63
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000063);
//RL,MCH,114,3618922
core::ptr::read_volatile(0xfed10114 as *const u32);
//WL,MCH,114,3618922
core::ptr::write_volatile(0xfed10114 as *mut u32, 0x03618922);
//RL,MCH,2f8,40ac7
core::ptr::read_volatile(0xfed102f8 as *const u32);
//WL,MCH,2f8,40ac7
core::ptr::write_volatile(0xfed102f8 as *mut u32, 0x00040ac7);
//WB,MCH,341,63
core::ptr::write_volatile(0xfed10341 as *mut u8, 0x00000063);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,e
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,341,64
core::ptr::write_volatile(0xfed10341 as *mut u8, 0x00000064);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,f
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,341,65
core::ptr::write_volatile(0xfed10341 as *mut u8, 0x00000065);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,e
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,341,66
core::ptr::write_volatile(0xfed10341 as *mut u8, 0x00000066);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,e
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,341,67
core::ptr::write_volatile(0xfed10341 as *mut u8, 0x00000067);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,f
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,341,70
core::ptr::write_volatile(0xfed10341 as *mut u8, 0x00000070);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,e
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,341,71
core::ptr::write_volatile(0xfed10341 as *mut u8, 0x00000071);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,f
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,341,72
core::ptr::write_volatile(0xfed10341 as *mut u8, 0x00000072);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,e
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,341,63
core::ptr::write_volatile(0xfed10341 as *mut u8, 0x00000063);
//RL,MCH,114,3618922
core::ptr::read_volatile(0xfed10114 as *const u32);
//WL,MCH,114,3618922
core::ptr::write_volatile(0xfed10114 as *mut u32, 0x03618922);
//RL,MCH,2f8,40ac7
core::ptr::read_volatile(0xfed102f8 as *const u32);
//WL,MCH,2f8,40ac7
core::ptr::write_volatile(0xfed102f8 as *mut u32, 0x00040ac7);
//WB,MCH,342,63
core::ptr::write_volatile(0xfed10342 as *mut u8, 0x00000063);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,3f
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,342,64
core::ptr::write_volatile(0xfed10342 as *mut u8, 0x00000064);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,3f
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,342,65
core::ptr::write_volatile(0xfed10342 as *mut u8, 0x00000065);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,e
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,342,66
core::ptr::write_volatile(0xfed10342 as *mut u8, 0x00000066);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,3e
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,342,67
core::ptr::write_volatile(0xfed10342 as *mut u8, 0x00000067);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,3f
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,342,70
core::ptr::write_volatile(0xfed10342 as *mut u8, 0x00000070);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,3f
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,342,71
core::ptr::write_volatile(0xfed10342 as *mut u8, 0x00000071);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,3e
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,342,72
core::ptr::write_volatile(0xfed10342 as *mut u8, 0x00000072);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,3f
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,342,73
core::ptr::write_volatile(0xfed10342 as *mut u8, 0x00000073);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,3f
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,342,74
core::ptr::write_volatile(0xfed10342 as *mut u8, 0x00000074);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,3f
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,342,75
core::ptr::write_volatile(0xfed10342 as *mut u8, 0x00000075);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,3f
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,342,66
core::ptr::write_volatile(0xfed10342 as *mut u8, 0x00000066);
//RL,MCH,114,3618922
core::ptr::read_volatile(0xfed10114 as *const u32);
//WL,MCH,114,3618922
core::ptr::write_volatile(0xfed10114 as *mut u32, 0x03618922);
//RL,MCH,2f8,40ac7
core::ptr::read_volatile(0xfed102f8 as *const u32);
//WL,MCH,2f8,40ac7
core::ptr::write_volatile(0xfed102f8 as *mut u32, 0x00040ac7);
//WB,MCH,343,66
core::ptr::write_volatile(0xfed10343 as *mut u8, 0x00000066);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,33f
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,343,67
core::ptr::write_volatile(0xfed10343 as *mut u8, 0x00000067);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,33f
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,343,70
core::ptr::write_volatile(0xfed10343 as *mut u8, 0x00000070);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,33f
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,343,71
core::ptr::write_volatile(0xfed10343 as *mut u8, 0x00000071);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,33f
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,343,72
core::ptr::write_volatile(0xfed10343 as *mut u8, 0x00000072);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,33f
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,343,73
core::ptr::write_volatile(0xfed10343 as *mut u8, 0x00000073);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,33f
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,343,74
core::ptr::write_volatile(0xfed10343 as *mut u8, 0x00000074);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,33f
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,343,75
core::ptr::write_volatile(0xfed10343 as *mut u8, 0x00000075);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,4ec,33f
core::ptr::read_volatile(0xfed104ec as *const u32);
//WB,MCH,343,66
core::ptr::write_volatile(0xfed10343 as *mut u8, 0x00000066);
//RL,MCH,114,3618922
core::ptr::read_volatile(0xfed10114 as *const u32);
//WL,MCH,114,2618922
core::ptr::write_volatile(0xfed10114 as *mut u32, 0x02618922);
//RL,MCH,2f8,60ac7
core::ptr::read_volatile(0xfed102f8 as *const u32);
//WL,MCH,2f8,60acf
core::ptr::write_volatile(0xfed102f8 as *mut u32, 0x00060acf);
//WB,MCH,340,76
core::ptr::write_volatile(0xfed10340 as *mut u8, 0x00000076);
//WB,MCH,341,76
core::ptr::write_volatile(0xfed10341 as *mut u8, 0x00000076);
//WB,MCH,342,81
core::ptr::write_volatile(0xfed10342 as *mut u8, 0x00000081);
//WB,MCH,343,81
core::ptr::write_volatile(0xfed10343 as *mut u8, 0x00000081);
//RL,MCH,340,81817676
core::ptr::read_volatile(0xfed10340 as *const u32);
//RL,MCH,124,80000502
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000542
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000542);
//RL,MCH,124,80000542
core::ptr::read_volatile(0xfed10124 as *const u32);
//WL,MCH,124,80000502
core::ptr::write_volatile(0xfed10124 as *mut u32, 0x80000502);
//RL,MCH,1a4,402
core::ptr::read_volatile(0xfed101a4 as *const u32);
//WL,MCH,1a4,442
core::ptr::write_volatile(0xfed101a4 as *mut u32, 0x00000442);
//RL,MCH,1a4,442
core::ptr::read_volatile(0xfed101a4 as *const u32);
//WL,MCH,1a4,402
core::ptr::write_volatile(0xfed101a4 as *mut u32, 0x00000402);
//RL,MCH,400,18800103
core::ptr::read_volatile(0xfed10400 as *const u32);
//WL,MCH,400,18000103
core::ptr::write_volatile(0xfed10400 as *mut u32, 0x18000103);
//RB,MCH,183,40
core::ptr::read_volatile(0xfed10183 as *const u8);
//RB,MCH,183,40
core::ptr::read_volatile(0xfed10183 as *const u8);
//WB,PCI,0,0,9c,80
core::ptr::write_volatile(0xf000009c as *mut u8, 0x00000080);
    }
}
