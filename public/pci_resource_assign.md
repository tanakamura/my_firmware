---
title: がんばろう、PCIの bus assign 〜 lspic -v で見れる情報ってなんやねん 〜
tags:
  - PC
  - BIOS
  - x86
  - PCI
private: false
organization_url_name: null
ignorePublish: false
---

前回 : https://qiita.com/tanakmura/items/57f1e39f090299354ccf

前回までで、ようやく SDRAM が使えるようになって、パソコンらしくなってきた。

最終目標としては、 https://qiita.com/tanakmura/items/c22981aafc9ed25734a0 に書いたとおり、なんらかのOSを動かすというのがあるので、次は画面表示を頑張ってみよう。

みんなが MS-DOS を使っていたような古い時代の PC では、ISA というアドレスとデータの信号がそのまま出てるだけみたいな野蛮なバスに周辺I/Oを付けていた。(らしい、私も触ったことない時代なのでよく知らない https://atmarkit.itmedia.co.jp/fsys/pcencyclopedia/006pc_history04/pc_hist07.html こういうのを読んで空気を感じてほしい)

しかし、Windows3.1やWindows95 が使える頃には野蛮なバスでは安定したコンピュータを作れないということで、実行時に使うリソースを決められる PCI が標準になった。

実行時に使うリソースが決められる、ということは逆に言えば、実行時にデバイスが使うリソースを誰かが決めないといけないということでもある。
これはハードウェアを使う前に決まってなければならない、BIOSはハードウェアを使う、つまり BIOS が PCI デバイスが使うリソースを割り当ててやらないといけない。

(このリソース割り当てはそんなに自明な問題ではないので、バグってるBIOSもある。バグってるBIOS対策でOSがリソース割り当てを行うこともある。Linuxの場合は、 `pci=assign-busses` をカーネルパラメータに渡してやると、この対策が入る)

というわけで、画面表示デバイスを有効にするために、PCI のリソースアサインをやってみよう。

# QEMU での動作確認

superIO と SDRAM の初期化が終われば QEMU でも 実機でも大体同じ状態になるので、ここからは QEMU も使ってみなさんの手元でも確認できるはずだ。

https://github.com/tanakamura/my_firmware/tree/qiita20250504/rust

このディレクトリで、

    $ cargo build --release

とする、すると、 `target/i686-firmware/release` 以下にオブジェクトができる。これに対して、

    $ qemu-system-i386 -bios ./target/i686-firmware/release/test_vga -m 2g -M pc -serial stdio

などすれば、このビルドしたオブジェクトをファームウェアとして動かして確認できるはずだ。画面に「Hello from My Firmware!!」と表示されてれば成功である。

(これの話は Binary Hacks Rebooted ( https://www.oreilly.co.jp/books/9784814400850/ )の 「#35　QEMU上で動くファームウェアを作る」にも書いた。興味があればそちらも参照)

もちろん、これは D945GCLF2 実機でも動作確認している。いないと思うが、持ってる人がいたらこれも焼いて試してほしい。


QEMUの画面に文字が出てるが、それの話は次回に回すとして、今日は、

ログに出てる↓
```
00 bridge_start=0x8000, bridge_end=0x8fff, *addr=0x9000, valid=true
00 bridge_start=0x90000000, bridge_end=0x900fffff, *addr=0x90100000, valid=true
00 bridge_start=0xb0000000, bridge_end=0xb0ffffff, *addr=0xb1000000, valid=true
VGA Option ROM copied to 0xc0000-0xc8e00, size=0x8e00 from=0x90010000
enable bridge VGACTL : 00:00:00
Bus 00:00:00 00-01:
00:00:00 membase:0x000000-0x0fffff, pref_membase:0x000000-0x0fffff, iobase:0000-0fff, brctl:0000, command:0007
00:00:00, vendor=8086, dev=1237, command=0007
00:01:00, vendor=8086, dev=7000, command=0007
00:01:01, vendor=8086, dev=7010, command=0007
BAR[4] : addr=00008000 size=00000010, is_io=true, prefetchable=false
00:01:03, vendor=8086, dev=7113, command=0007
00:02:00, vendor=1234, dev=1111, command=0007
BAR[0] : addr=b0000000 size=01000000, is_io=false, prefetchable=true
BAR[2] : addr=90000000 size=00001000, is_io=false, prefetchable=false
EXP_ROM: addr=90010000 size=00010000
00:03:00, vendor=8086, dev=100e, command=0007
BAR[0] : addr=90020000 size=00020000, is_io=false, prefetchable=false
BAR[1] : addr=00008040 size=00000040, is_io=true, prefetchable=false
EXP_ROM: addr=90040000 size=00040000
VGA 00:02:00: 1234:1111 03:00:00
VGA option rom found. invoke vga option rom
```

この部分の話をする。

# PCI の config 空間

PCIでアクセスできる領域には、MMIO空間、I/O空間、Configuration空間の3種類がある。

MMIOやI/Oを有効にするには、Configuration空間を介して設定を行う必要があり、そのためにはまずConfiguration空間へのアクセスが可能な状態でなければならない。このConfiguration空間にアクセスする際には、**バス番号（bus）、デバイス番号（device）、ファンクション番号（function）**を指定してアクセスする。


Linux を使ってる人なら、起動中のメッセージとして、
`0000:00:02:00` 
のような数字列を見たことがあるだろう。

最初の4桁は、PCI のドメインになる。これは、私もよく知らないので各自で調べて…(NUMA環境とかでPCI host bridgeが複数あると変わるかもしれない)

次の 3つの2桁の数字が、それぞれ bus, dev, function になる。これにプラスして、デバイス内でバイト単位でのオフセットを指定すると、configuration 空間の各レジスタにアクセスできる。


:バス番号: PCI のバスはツリー上になっていて、デバイスと同じように、複数のデバイスをさらにネストして繋げられるブリッジを接続できる。どのブリッジにあるデバイスにアクセスするかを指定するのが、バス番号になる。PCI express では、バスは存在しないので、バス番号という名前は実態とはあってないが、PCI express は PCI と互換性を維持していて、ソフトウェアからはバス番号を同じように使える。

:デバイス番号: 各ブリッジには、デバイスを最大31個まで接続できる。この複数のデバイスの中から一個を識別するために使うのがデバイス番号になる。

:ファンクション番号: 各デバイスは、中に最大8個までファンクションを持つことができる。この複数のファンクションの中から一個を識別するために使うのがファンクション番号になる。

SoC やノースブリッジ、サウスブリッジに含まれているデバイスは、実際にはPCI で接続されていなくても、PCIの仕様を借用してPCI と同様にアクセスできることがある。

例えば、D945GCLF2 の PCI にディスプレイアダプタを付けたときの `lspci -t` を見てみると、

```
(TBD)
```

こんな風になっている。

00.0

# qemu と PCI バスツリー

qemu は、PCI バスツリーを自由に組んでテストできるようになっている。

マザーボードを用意するより色々な意味で楽なので、使いかたを覚えておこう。

まず、 qemu を使っている人なら、 `-M q35` や `-M pc` というオプションを使ったことがあるかもしれない。今回の点で重要なのは、 `-M q35` すると、PCI ホストブリッジが PCI express 相当のものになるのに対して、`-M pc` とすると、PCI ホストブリッジが (express でない) PCI 相当になるという点である。

PCI express は物理的にはバスが存在していないため、直接ブリッジの下に複数デバイスを繋げることができない。間にスイッチを挟む必要があり、設定としては少し面倒である。

上で動くソフトウェアから見ると、どちらでも大差ないので、`-M q35`を使わないようにしよう。

     $ qemu-system-i386 -M pc -serial mon:stdio
