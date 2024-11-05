# Convolution

Rustで書かれた畳み込み演算プログラムです．

# Requirement

作成時の最新バージョンで作られていたため，その時のバージョンを必要としていますが，おそらくある程度バージョンが低くても動作します．

* rustc 1.82.0
* cargo 1.82.0
 
# Installation

Rustをインストールし，build環境を準備する必要があります．Cargoも同時にインストールされます．

```bash
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

Rustのアップデートは以下のコマンドで行えます．

```bash
$ rustup update
```

# Usage

このプロジェクトのフォルダに入れたら，cargoでビルドし，実行することができます．

この際，任意の数値列を2行入力する必要があります．

```bash
$ git clone https://github.com/Kawa0x5F/Convolution
$ cd Convolution
$ cargo run
$ （任意の数値列を入力）
$ （任意の数値列を入力）
```
