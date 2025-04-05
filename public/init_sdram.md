---
title: D945GCLF2 で SDRAM を初期化する
tags:
  - PC
  - BIOS
  - x86
private: false
updated_at: '2025-04-06T01:14:47+09:00'
id: 57f1e39f090299354ccf
organization_url_name: null
slide: false
ignorePublish: false
---
(なんか忙しくて間があきましたすいません)

前回 : https://qiita.com/tanakmura/items/1951f3f6ff84fd27fe31

前回の話で、DRAM コントローラ初期化のためのパラメータを取れるようになった。

これを使って DRAM コントローラ初期化をしよう…と言いたいところだが、残念ながら、DRAM コントローラの仕様は公開されておらず、何をやったら初期化できたと入れるのか筆者もよくわかってない。

coreboot にはソースあるものの、これが何してるのかは正直わかりませんでした…

https://github.com/coreboot/coreboot/blob/584f9bcc3f96565c92bb4a6513a90da4fb56fb66/src/northbridge/intel/i945/raminit.c


なんかDRAMコントローラの詳細が分かるものがあれば良かったのだけど、筆者の力では見つけられなかった…

Renesas 社のCPUであれば、SDR SDRAM のコントーラの詳細であれば書いてある、気がしますね。

https://www.renesas.com/ja/products/microcontrollers-microprocessors/other-mcus-mpus/superh-risc-engine-family-mcus/sh7727-32-bit-microcontrollers

でもこれを見てSDR SDRAM が使えるようになるかは試したことないので知らないです。

これに加えて、DDR SDRAM 以降は信号タイミングを調整したりする、いわゆる「トレーニング」が入っていて初期化手順がさらに難しくなっているので、正直説明できることはなにもないです。

まあでもなんも分からずにcorebootのコードコピペして使うのも納得感が低いので、「正規のファームウェアからDRAM初期化シーケンスを抜いてそれを使う」という方法を試すことにしよう。


# モニタプログラムの準備

まず「モニタプログラム」というのを作ろう。「モニタプログラム」というと定義は色々状況によって違うけど、ここでは、「シリアルから受け取ったコマンドをもとに、マシン内の状態を読み書きするプログラム」とする。


```
+----------------+
|host PC         |             +--------------+
|                |             |target PC     |
|    +---------+ |             | +--------+   |
|    |test     ----- UART -------|monitor |   |
|    |program  | |             | +--------+   |
|    +---------+ |             +--------------+
|                |
+----------------+
```

https://github.com/tanakamura/my_firmware/blob/qiita20250405/d945gclf_monitor/r/src/console.rs

ソースはこんな感じ、コマンドを送って、そのコマンドと対応するペイロードを送ると、対応するリプライが返ってくる。


このモニタはこれまでも使っていて、前回書いた SPD アクセスのテストとかは、実際は↓これで試していた。

https://github.com/tanakamura/my_firmware/blob/qiita20250405/d945gclf_monitor/spd.py


ファームウェアは、実機で開発しようとすると、毎回フラッシュ書き込みが必要になり、かなりTATが長くなってしまう。開発用ではない一般用機械で開発してると、ケーブル抜き差しや、電源ON/OFFのたびに実際に物理的に手を動かす必要があり、かなり手間である。
特に、SPI Flash のクリップは貧弱でアクセスも不安定なので、ちょっと手が触れるだけですぐ接続できなくなるし、クリップもすぐ壊れる(筆者はもう2個破壊している)。

なので、SPI Flash 書き換えしないで開発できる方法はそこそこ重要である。

筆者は、

1. モニタプログラムを準備する
2. モニタ経由で実験して、手順を確率する
3. その手順をCやRustで再実装してSPI Flash に焼く

という手順でやっている。

ここで準備したモニタプログラムは、

- メモリアクセス
- IOアクセス
- RDMSR, WRMSR

ができるようにしてある。


# モニタプログラムを使って、正規のファームウェアが行っている初期化シーケンスを取り出す

で、続いて、この用意したモニタプログラムを使って、うまいことやってみよう。

正規のファームウェアは、SMBus 経由で SPD を読んで、それをもとにDRAM初期化する、というようなことをやっているはずである。

この初期化シーケンスを、モニタプログラムとKVMを使って抜き出してみる。


手順は、

1. KVM で作った自作 VM 上で正規ファームウェアを動かす
2. 正規ファームウェアが行う I/O および MMIO アクセスをモニタに流す
3. そのMMIOアクセスシーケンスを抜き出して、自分のファームウェアにハードコードする

とする。

```
+------------------+
|host PC           |                 +--------------+
|                  |                 |target PC     |
|    +-----------+ |                 | +--------+   |
|    |biosvm   --------- UART ----------monitor |   |
|    |  on  KVM  | |                 | +--------+   |
|    | +--------+| |                 +--------------+
|    | |orig    || |
|    | |firmware|| |
|    | +--------+| |
|    |           | |
|    +-----------+ |
|                  |
+------------------+
```
図としてはこんな感じ。


MMIO シーケンスのダンプは https://github.com/tanakamura/my_firmware/blob/5ab05853df7660770939228884abe759e4910685/biosvm/cpu.cpp#L45 こんな感じにする。


実際には、目視 & 手作業がそれなりに発生する。

特に CAR 領域 (https://qiita.com/tanakmura/items/ca0aaf4402ea0d399e0e を読もう )  のアドレスはファームウェア毎に違うので、なんとなくそれっぽい箇所をよく見てさがす。 D945GCLF2 のファームウェアは、 `0xfefc0000` を使っていたようだ https://github.com/tanakamura/my_firmware/blob/5ab05853df7660770939228884abe759e4910685/biosvm/vm.hpp#L93


この biosvm を動かす手順は以下のとおり。

1. オリジナルのファームウェアを抜き出す ( https://qiita.com/tanakmura/items/918cc1e80da3324367c0 で書いた original.rom です )
2. 実機で、上で書いたモニタプログラムを動かし、ttyS0 経由でアクセスできる状態にする。(ttyS0 のパスが違う場合は、main.cpp の/dev/ttyS0 をなおして…)
3. biosvm に original.rom を渡して起動する。biosvm 内でオリジナルファームウェアが動き、MMIO アクセスだけが抜き出されて ttyS0 の先のモニタにフォワードされる。その結果が、stderr に出る

これで、stderr に、
```
OL,00000cf8,8000f8d8
OH,00000cfc,0000ffcf
OL,00000cf8,80000048
OL,00000cfc,f0000003
WL,f0000044,fed14001
RB,fed14c00,00000001
OL,00000cf8,80000090
OB,00000cfc,00000000
OL,00000cf8,80000094
OB,00000cfd,00000000
OB,00000cfe,00000000
WB,f00000df,00000000
```

https://github.com/tanakamura/my_firmware/blob/qiita20250405/d945gclf_monitor/record.txt

こんなのが表示されるはずだ。`OL,addr,val` は、 32bit out 命令、でaddrにvalを書くというステップ、`WL,addr,val` は 32bit memory write 命令で、addr に val を書く、といったような記録だ。

これで、正規ファームウェアが実行している DRAM コントローラ初期化シーケンスが採取できた

# 採取した DRAM コントローラ初期化シーケンスを流す

これをそのまま流してもいいのだが(確か動作確認したはず)、これには MMCFG (MMIO 経由の pci config アクセス用レジスタ) の設定などが含まれているし、SPD を読むための SMBus アクセスなどが含まれていて、自分のファームウェアとくっつけるのは少し不都合が出るかもしれない。

もう少しこれを解読して、まともにアクセスするために変換するのが、https://github.com/tanakamura/my_firmware/blob/qiita20250405/d945gclf_monitor/replay.py これになる。

これは、先程の生出力を解析して、DRAM 初期化に関連する MMIO アクセスだけを抜き出すスクリプトになる。

coreboot の raminit ソースから、DRAM 初期化に必要なのは、i945 ノースブリッジ上にある、MCHBAR である。
MCHBAR へのアクセス方法は、公式資料にも書かれており、

https://www.intel.com/Assets/PDF/datasheet/307502.pdf

Bus=0, Dev=0, Fn=0 (以下 00:0.0) の PCI デバイス上のconfig空間0x44 に 32bit値を書くと、そこへマップされる。

正規ファームウェアでは、 00:0.0 のconfig空間へのアクセスは、PCIEXBAR 経由で行われており、PCIEXBAR は、00:0.0 の 48H にある。

先程抜き出した、シーケンスを見ると、

```
OL,00000cf8,8000f8d8
OH,00000cfc,0000ffcf
OL,00000cf8,80000048
OL,00000cfc,f0000003
```



ふたつめのPCIアクセスで、00:0.0 の 48H へ、0xf000_0003 を設定しているのがわかる。これより、PCI config 空間は0xf000_000 から 128MB にマップされていることが分かる。

下 の 3の意味は↓で、

![image.png](https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/12877/c2a11421-f289-45a0-b45d-2ff9175cd4a1.png)

(https://www.intel.com/Assets/PDF/datasheet/307502.pdf より)

最初のPCIアクセスの意味は…分かりませんでした…D31:f0 のd8 = GPIOBASE にアクセスしてる？

```
WL,f0000044,fed14001
```

次に、これ、これは PCIEXBAR 経由で、bus=0,dev=0,fn=0 の 44h にアクセスしてるということが分かりこれから MCHBAR が 0xfed1_4000 にマップされていることがわかる。

```
RL,fed1f400,00000000
WL,fed1f400,00000004
WL,fed1f100,00042210
WL,fed1f104,00002100
WL,fed1f108,10004321
WL,fed1f10c,00214321
```

あとはなんか、こういう MCHBAR 経由っぽいアクセスだけ抜き出せば、メモリコントローラへのアクセスだけ抜き出せる、というわけである。

これで処理したものが、

https://github.com/tanakamura/my_firmware/blob/qiita20250405/d945gclf_monitor/commands.txt

これになる。DDR2 SDRAM 初期化のためにはDRAMへの初期化も必要なようなので、それも入れてある。


これを実機に流してみる。

https://github.com/tanakamura/my_firmware/blob/qiita20250405/d945gclf_monitor/replay.py

これには、実機に流す機能も入れてあって、`actual_io = True` にすると、このシーケンスをモニタに流せるようになっている。

これを流したあと、 SDRAM の 0番地にアクセスする

https://github.com/tanakamura/my_firmware/blob/qiita20250405/d945gclf_monitor/rw0.py

を実行すると、実際に SDRAM にデータを保持できたことが確認できるはずだ！ここまで長かったが、無事SDRAMを使える状態になった。もう 1MB もない CAR 領域で苦労する必要はない。

これを試行錯誤していて気付いたが、D945GCLF2 では CAR はデータしか置けないらしく、キャッシュがうまくいかないのか命令を置いても実行することができなかった。これからは、命令をSDRAM上に置いて書き換えることができる。


これで動作を確認できたら、これを Rust に変換して、自分のファームウェアに組み込もう。

https://github.com/tanakamura/my_firmware/blob/qiita20250405/d945gclf_monitor/r/src/raminit.rs

これは、厳密にいうとうまく動かない可能性がある、というのはDDR SDRAM初期化はタイミングが必要になる場面はおそらくあって、シリアル経由のモニタからシーケンスを実行した場合はOKでも、ネイティブ機械語として流すとタイミングあわなくなって初期化に失敗するという懸念がある。が、まあとりあえず D945GCLF2 では問題なさそうなので、これでヨシとしよう。


次回、PCI bus assign 編に続く

super IO と DRAM 初期化が終わればあとは QEMU で実験できるので、次回以降しばらくQEMUで書きます。

