使用例：uniq は 連続した行の重複をまとめる。

$ cargo run -- -c tests/inputs/three.txt
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `target\debug\uniqr.exe -c tests/inputs/three.txt`
   2 a
   2 b
   1 a
   3 c
   1 a
   4 d

$ sort tests/inputs/three.txt | cargo run -- -c
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `target\debug\uniqr.exe -c`
   4 a
   2 b
   3 c
   4 d

日本語でもOK
$ cargo run -- -c tests/inputs/日本語例.txt
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `target\debug\uniqr.exe -c 'tests/inputs/日本語例.txt'`
   2 猫
   1 犬
   1 豚
   1 猫
   3 鬼太郎
   1 猫娘
   1 鬼太郎
   1 猫娘
nagat@HugeLeopard MINGW64 /d/study/rust/command-line-rust/06_uniqr (main)
$ sort tests/inputs/日本語例.txt | cargo run -- -c
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `target\debug\uniqr.exe -c`
   1 犬
   3 猫
   2 猫娘
   1 豚
   4 鬼太郎
