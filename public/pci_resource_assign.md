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

    $ qemu-system-i386 -bios ./target/i686-firmware/release/test_vga -m 2g -M pc -serial mon:stdio

などすれば、このビルドしたオブジェクトをファームウェアとして動かして確認できるはずだ。


