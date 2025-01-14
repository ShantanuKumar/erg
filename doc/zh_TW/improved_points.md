# Python 的改進

[![badge](https://img.shields.io/endpoint.svg?url=https%3A%2F%2Fgezf7g7pd5.execute-api.ap-northeast-1.amazonaws.com%2Fdefault%2Fsource_up_to_date%3Fowner%3Derg-lang%26repos%3Derg%26ref%3Dmain%26path%3Ddoc/EN/improved_points.md%26commit_hash%3Deccd113c1512076c367fb87ea73406f91ff83ba7)](https://gezf7g7pd5.execute-api.ap-northeast-1.amazonaws.com/default/source_up_to_date?owner=erg-lang&repos=erg&ref=main&path=doc/EN/improved_points.md&commit_hash=eccd113c1512076c367fb87ea73406f91ff83ba7)

## 執行靜態分析(靜態類型檢查、變量和屬性檢查)

靜態類型檢查的好處現在怎么強調都不為過，但是檢查變量和屬性的存在也是相當重要的一部分。

## 嚴格范圍處理

在 Python 中，語句沒有范圍。
因此，在 `for` 或 `if` 中定義的變量具有外部影響。 不能隨便命名變量。

```python
for i in range(10):
    x = 1
    print(i + x)
print(x) # 1
```

在 Erg 中，所有塊都有范圍并且是完全隔離的。

## 明確區分可變對象和不可變對象

Python并不清楚可變和不可變/堆和值對象之間的區別，因此您必須記住元組是不可變的但列表是可變的......您需要記住元組是不可變的，但列表是可變的 ... 等等。
另外，如果你想讓你自己的類不可變，你必須經歷一個乏味的過程。

```python
# 你能相信這段代碼對過去的 Python 版本有效嗎?
i = 256
assert i is 256
i = 257
assert i is not 257
```

## Traits

就像 Java 的接口一樣，你可以進行基于契約的編程。

Python 也有 ABC(抽象基類)，但這種結構最適合靜態類型。

## 靜態解決依賴關系

這可以防止長時間運行程序然后由于缺少模塊而出現錯誤的煩人體驗。

## 內置包管理器

具有標準化目錄結構和構建文件的可重現構建。
當然提供了鎖定文件生成和版本控制。
無需為每個項目選擇或混合anaconda、pyenv、詩歌等。
