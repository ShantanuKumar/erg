# Rust 代码指南

[![badge](https://img.shields.io/endpoint.svg?url=https%3A%2F%2Fgezf7g7pd5.execute-api.ap-northeast-1.amazonaws.com%2Fdefault%2Fsource_up_to_date%3Fowner%3Derg-lang%26repos%3Derg%26ref%3Dmain%26path%3Ddoc/EN/dev_guide/rust_code_guideline.md%26commit_hash%3Dd15cbbf7b33df0f78a575cff9679d84c36ea3ab1)](https://gezf7g7pd5.execute-api.ap-northeast-1.amazonaws.com/default/source_up_to_date?owner=erg-lang&repos=erg&ref=main&path=doc/EN/dev_guide/rust_code_guideline.md&commit_hash=d15cbbf7b33df0f78a575cff9679d84c36ea3ab1)

## 本地规则

* 使用 `log!` 进行调试输出(使用 `println!` 等进行输出处理，这也是发布所必需的)。
* 未使用或内部变量/方法(私有且仅用于特定功能)必须以 `_` 为前缀。如果要避免与保留字冲突，请在末尾添加一个`_`。

## 推荐代码

* 定义和使用特定领域的枚举而不是数字枚举或布尔值。
* 将访问修饰符保持在最低限度。即使在发布时也要优先使用 `pub(mod)` 或 `pub(crate)`。
* 将 for 表达式中的可迭代对象显式转换为迭代器(`for i in x.iter()` 而不是 `for i in x`)。
* 懒惰的评价。例如，如果 `default` 不是文字，请使用 `unwrap_or_else` 而不是 `unwrap_or`。

## 不鼓励使用代码

* 大量使用返回类型重载。特别是使用大量非显而易见的 `.into` 的代码。这是因为类型推断结果可能违反直觉。在这种情况下，建议使用 `from` 代替。
* 大量使用 `Deref`。这有效地提出了与继承相同的问题。

## 根据上下文做出决策的代码

* 定义未使用的辅助方法。
* 大量使用 `unwrap` 和 `clone`。在某些情况下，没有什么比这样做更好的了。