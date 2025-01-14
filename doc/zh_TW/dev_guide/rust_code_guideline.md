# Rust 代碼指南

[![badge](https://img.shields.io/endpoint.svg?url=https%3A%2F%2Fgezf7g7pd5.execute-api.ap-northeast-1.amazonaws.com%2Fdefault%2Fsource_up_to_date%3Fowner%3Derg-lang%26repos%3Derg%26ref%3Dmain%26path%3Ddoc/EN/dev_guide/rust_code_guideline.md%26commit_hash%3Dd15cbbf7b33df0f78a575cff9679d84c36ea3ab1)](https://gezf7g7pd5.execute-api.ap-northeast-1.amazonaws.com/default/source_up_to_date?owner=erg-lang&repos=erg&ref=main&path=doc/EN/dev_guide/rust_code_guideline.md&commit_hash=d15cbbf7b33df0f78a575cff9679d84c36ea3ab1)

## 本地規則

* 使用 `log!` 進行調試輸出(使用 `println!` 等進行輸出處理，這也是發布所必需的)。
* 未使用或內部變量/方法(私有且僅用于特定功能)必須以 `_` 為前綴。如果要避免與保留字沖突，請在末尾添加一個`_`。

## 推薦代碼

* 定義和使用特定領域的枚舉而不是數字枚舉或布爾值。
* 將訪問修飾符保持在最低限度。即使在發布時也要優先使用 `pub(mod)` 或 `pub(crate)`。
* 將 for 表達式中的可迭代對象顯式轉換為迭代器(`for i in x.iter()` 而不是 `for i in x`)。
* 懶惰的評價。例如，如果 `default` 不是文字，請使用 `unwrap_or_else` 而不是 `unwrap_or`。

## 不鼓勵使用代碼

* 大量使用返回類型重載。特別是使用大量非顯而易見的 `.into` 的代碼。這是因為類型推斷結果可能違反直覺。在這種情況下，建議使用 `from` 代替。
* 大量使用 `Deref`。這有效地提出了與繼承相同的問題。

## 根據上下文做出決策的代碼

* 定義未使用的輔助方法。
* 大量使用 `unwrap` 和 `clone`。在某些情況下，沒有什么比這樣做更好的了。