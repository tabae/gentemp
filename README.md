# gentemp
プログラミングコンテスト用テンプレファイル＆ディレクトリ生成ツール

## インストール方法
このツールはRustで書かれています．
作者はRust初心者なのでインストール方法をよく分かっていません．
とりあえずRustの環境を構築して，このリポジトリをクローンして`cd gentemp`して，`cargo install --path .`をするとインストールできるのではないでしょうか．
本当によく分かっていません．ごめんなさい．

## 使い方
- 基本的な使い方は，`gentemp <ディレクトリ名>`でディレクトリを生成できます．
- ディレクトリ名が指定されない場合は`dir_generated_by_gentemp`というディレクトリができます．
```
$ gentemp abc200
$ tree abc200
abc200/
├── a
│   └── main.cpp
├── b
│   └── main.cpp
├── c
│   └── main.cpp
├── d
│   └── main.cpp
├── e
│   └── main.cpp
├── f
│   └── main.cpp
├── g
│   └── main.cpp
└── h
    └── main.cpp
```
- `-n`オプションでサブディレクトリの数を指定できます．MAX26です．（アルファベットが足りなくなるので．）
```
$ gentemp abc201 -n 10
$ ls abc201
a  b  c  d  e  f  g  h  i  j
```
- `-s, --source`オプションでソースファイルを指定できます．
- 特にソースファイルを指定しない場合，ソースコード中の`path_to_template_file`のファイルがコピーされます．
- `--cf`オプションを指定した場合は，ソースコード中の`path_to_template_file_cf`のファイルがコピーされます．こどふぉ用のテンプレートファイルを割り当てておくことを想定しています．
- `--mini`オプションを指定した場合は，ソースコード中の`path_to_template_file_mini`のファイルがコピーされます．作者はテンプレを最小限に減らしたテンプレートファイルを割り当てていますが，好きに使ってください．
- `main.cpp`の名前を変えたい場合，ソースコード中の`generated_file_name`を変更してください．
