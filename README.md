# rust-image-resizer
Rust の image クレートを利用して作った簡単な画像のリサイズアプリ

## 使い方
```rust-image-resizer/input```に変換したい画像ファイルを入れる（inputディレクトリがない場合は作成）。

```rust-image-resizer```内で
```
cargo run
```
を打つ（Rustのインストールをしていない場合はまずそれをする）。

「何分の一にしたいか（整数値）」と、変換したい画像ファイルのファイル名を、改行区切りで入力する。

```rust-image-resizer/output```内に変換後の画像ファイル（```ファイル名.after_resize.拡張子```）が保存される。
