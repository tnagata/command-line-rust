findの使い方例(bash):
-typeで使えるオプションは、d: directory, f: file, l: link
tests/inputs で実行した結果
$ find . -type d
.
./a
./a/b
./a/b/c
./d
./d/e
./f


$ find . -type f
./a/a.txt
./a/b/b.csv
./a/b/c/c.mp3
./d/b.csv
./d/d.tsv
./d/d.txt
./d/e/e.mp3
./f/f.txt
./g.csv


$ find . -name \*.csv　bashでは\*とする必要があるので注意
./a/b/b.csv
./d/b.csv
./g.csv
あるいは、
$ find . -name "*.csv"
./a/b/b.csv
./d/b.csv
./g.csv

cd ../../
$ find ./tests -name cli.rs
./tests/cli.rs


◇　tests/cli.rsの修正前のワーニング例：

warning: use of deprecated associated function `assert_cmd::Command::cargo_bin`: incompatible with a custom cargo build-dir, see instead `cargo::cargo_bin_cmd!`
  --> tests\cli.rs:30:14
   |
30 |     Command::cargo_bin(PRG)?
   |              ^^^^^^^^^
   |
   = note: `#[warn(deprecated)]` on by default
この類が全部で４カ所

warning: hiding a lifetime that's elided elsewhere is confusing
  --> tests\cli.rs:63:36
   |
63 | fn format_file_name(expected_file: &str) -> Cow<str> {
   |                                    ^^^^     ^^^^^^^^ the same lifetime is hidden here
   |                                    |
   |                                    the lifetime is elided here

help: use `'_` for type paths
   |
63 | fn format_file_name(expected_file: &str) -> Cow<'_, str> {
   |                                                 +++
これに従って修正した