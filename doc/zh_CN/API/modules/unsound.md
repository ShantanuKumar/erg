# 模块 `unsound`

[![badge](https://img.shields.io/endpoint.svg?url=https%3A%2F%2Fgezf7g7pd5.execute-api.ap-northeast-1.amazonaws.com%2Fdefault%2Fsource_up_to_date%3Fowner%3Derg-lang%26repos%3Derg%26ref%3Dmain%26path%3Ddoc/EN/API/modules/unsound.md%26commit_hash%3D06f8edc9e2c0cee34f6396fd7c64ec834ffb5352)](https://gezf7g7pd5.execute-api.ap-northeast-1.amazonaws.com/default/source_up_to_date?owner=erg-lang&repos=erg&ref=main&path=doc/EN/API/modules/unsound.md&commit_hash=06f8edc9e2c0cee34f6396fd7c64ec834ffb5352)

让 API 执行在 Erg 的类型系统中无法保证的不健全和不安全的操作。

## `unsafe!`

执行"不安全"过程。 就像 Rust 一样，`Unsafe` API 不能直接调用，而是作为高阶函数传递给这个过程。

```python
unsound = import "unsound"

i = unsound. unsafe! do!:
     # 将 `Result Int` 转换为 `Int`
     unsound.transmute input!().try_into(Int), Int
```

## transmit

将第一个参数的对象转换为第二个参数的类型。没有进行类型检查。
这个函数破坏了类型系统的类型安全。请在使用前进行验证。

## 隐式转换

与 `transmute` 不同，它会自动转换为预期的类型。与 Ocaml 的 `Obj.magic` 工作方式相同。