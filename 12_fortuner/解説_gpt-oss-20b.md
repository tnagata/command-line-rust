## このプログラムの目的
「source ファイルに含まれるテキスト（フォーチュン）を検索／取得する」

sources に渡したディレクトリ/ファイルを再帰的に走査し、
「%」で区切られた各ブロックを 1 件の Fortune として読み込みます。
その後、オプションで正規表現 pattern を指定すれば、
「このテキストにマッチするフォーチュンだけを表示」
という検索機能が実装されています。
パターンが省略されると ランダム に 1 件だけを返します（シード指定で再現性もあり）。

## 1️⃣ このプログラムの全体像  
```
fortune.rs
├─ main()          ← エントリーポイント（コマンドライン引数を解析し `run()` を呼ぶ）
└─ run(args)       ← 実際に「fortunes」を探して読み込み、表示ロジックを実行する
   ├─ find_files()    → 与えられたパスからすべてのファイル（.dat 拡張子は除外）を取得
   ├─ read_fortunes()→ それぞれのファイルを読み込み、`%` を区切り文字にして複数のフォーチュンへ分割
   └─ pick_fortune() → ランダム（またはシード指定）で1つだけ選択
```

### コマンドライン引数  
`clap::Parser` で自動生成される構造体 `Args` が、次のオプションを持ちます。

| フィールド | 意味 | 備考 |
|------------|------|------|
| `sources: Vec<String>` | ファイル・ディレクトリのパス（必須） | ここに検索対象が入る |
| `pattern: Option<String>` | 正規表現で絞り込みたい文字列 | 省略するとランダム表示 |
| `insensitive: bool` | 大文字小文字を区別しないか | `-i/--insensitive` で付けられる |
| `seed: Option<u64>` | ランダムシード | 同じ出力を再現したいときに使う |

## 2️⃣ `run()` の詳細な流れ  

```rust
fn run(args: Args) -> Result<()> {
    // --------------------------------------------------
    // 1. pattern を Optional<Regex> に変換
    let pattern = args.pattern.map(|val| {
        RegexBuilder::new(val.as_str())
            .case_insensitive(args.insensitive)
            .build()
            .map_err(|_| anyhow!(r#"Invalid --pattern "{val}""#))
    }).transpose()?;

    // --------------------------------------------------
    // 2. 指定されたパスからすべてのファイルを探す
    let files = find_files(&args.sources)?;

    // --------------------------------------------------
    // 3. ファイルごとにフォーチュンを読み込む
    let fortunes = read_fortunes(&files)?;

    // --------------------------------------------------
    // 4. pattern があるかないかで分岐
    match pattern {
        Some(pattern) => {   // フィルタリング + 表示
            let mut prev_source = None;
            for fortune in fortunes.iter().filter(|f| pattern.is_match(&f.text)) {
                if prev_source.as_ref().map_or(true, |s| s != &fortune.source) {
                    eprintln!("({})\n%", fortune.source);  // ファイル名を表示
                    prev_source = Some(fortune.source.clone());
                }
                println!("{}\n%", fortune.text);
            }
        }
        None => {           // ランダムに1つだけ選択して表示
            println!(
                "{}",
                pick_fortune(&fortunes, args.seed)
                    .or_else(|| Some("No fortunes found".to_string()))
                    .unwrap()
            );
        }
    }

    Ok(())
}
```

### 重要ポイント

| ステップ | なぜ必要か |
|----------|------------|
| **1. pattern のコンパイル** | 正規表現が無効だった場合は即時にエラーを返す。`anyhow!()` でわかりやすいメッセージを生成。 |
| **2. find_files()** | `WalkDir` を使って再帰検索し、`.dat` ファイルだけ除外。結果をソート・重複除去して一意に保つ。 |
| **3. read_fortunes()** | 各行を読み込み、`%` で区切られたブロックを `Fortune { source, text }` に変換。バッファリングでメモリ効率も良い。 |
| **4. 表示ロジック** | ① パターンが指定されているときは一致したフォーチュンだけを、② 指定なしならランダムに1つだけを出力。`pick_fortune()` はシードあり/無しで同じ