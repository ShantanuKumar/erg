# Python 的改进

[![badge](https://img.shields.io/endpoint.svg?url=https%3A%2F%2Fgezf7g7pd5.execute-api.ap-northeast-1.amazonaws.com%2Fdefault%2Fsource_up_to_date%3Fowner%3Derg-lang%26repos%3Derg%26ref%3Dmain%26path%3Ddoc/EN/improved_points.md%26commit_hash%3Deccd113c1512076c367fb87ea73406f91ff83ba7)](https://gezf7g7pd5.execute-api.ap-northeast-1.amazonaws.com/default/source_up_to_date?owner=erg-lang&repos=erg&ref=main&path=doc/EN/improved_points.md&commit_hash=eccd113c1512076c367fb87ea73406f91ff83ba7)

## 执行静态分析(静态类型检查、变量和属性检查)

静态类型检查的好处现在怎么强调都不为过，但是检查变量和属性的存在也是相当重要的一部分。

## 严格范围处理

在 Python 中，语句没有范围。
因此，在 `for` 或 `if` 中定义的变量具有外部影响。 不能随便命名变量。

```python
for i in range(10):
    x = 1
    print(i + x)
print(x) # 1
```

在 Erg 中，所有块都有范围并且是完全隔离的。

## 明确区分可变对象和不可变对象

Python并不清楚可变和不可变/堆和值对象之间的区别，因此您必须记住元组是不可变的但列表是可变的......您需要记住元组是不可变的，但列表是可变的 ... 等等。
另外，如果你想让你自己的类不可变，你必须经历一个乏味的过程。

```python
# 你能相信这段代码对过去的 Python 版本有效吗?
i = 256
assert i is 256
i = 257
assert i is not 257
```

## Traits

就像 Java 的接口一样，你可以进行基于契约的编程。

Python 也有 ABC(抽象基类)，但这种结构最适合静态类型。

## 静态解决依赖关系

这可以防止长时间运行程序然后由于缺少模块而出现错误的烦人体验。

## 内置包管理器

具有标准化目录结构和构建文件的可重现构建。
当然提供了锁定文件生成和版本控制。
无需为每个项目选择或混合anaconda、pyenv、诗歌等。
