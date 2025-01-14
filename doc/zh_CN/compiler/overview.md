# `erg` 概览

[![badge](https://img.shields.io/endpoint.svg?url=https%3A%2F%2Fgezf7g7pd5.execute-api.ap-northeast-1.amazonaws.com%2Fdefault%2Fsource_up_to_date%3Fowner%3Derg-lang%26repos%3Derg%26ref%3Dmain%26path%3Ddoc/EN/compiler/overview.md%26commit_hash%3Dd15cbbf7b33df0f78a575cff9679d84c36ea3ab1)](https://gezf7g7pd5.execute-api.ap-northeast-1.amazonaws.com/default/source_up_to_date?owner=erg-lang&repos=erg&ref=main&path=doc/EN/compiler/overview.md&commit_hash=d15cbbf7b33df0f78a575cff9679d84c36ea3ab1)

我们将介绍每一层的功能以及特别重要的功能和方法。

## 1. 词法分析

* `Lexer` 进行词法分析。 `Lexer::next`(`Lexer`被实现为一个迭代器)负责词法分析的主要逻辑。 `Token` 作为解析的结果输出。

## 2. 解析

* `Parser` 进行解析。特别重要的是`Parser::parse_expr`。作为解析的结果，输出作为`ast::Expr`的集合的`AST`。

## 3. 脱糖

* 脱糖由 `Desugarer` 完成。 `AST` 将被输出。

## 4. 类型检查/类型推断

* `ASTLowerer` 进行打字。类型检查主要由 `Context` 完成。尤其重要的是 `Context::supertype_of`(确定子类型关系)、`Context::unify/sub_unify`(统一/半统一类型变量)、`Context::init_builtin_*`(定义内置 API)。 `HIR` 作为分析结果输出。

## 5. 副作用检查

* `SideEffectChecker` 可以。

## 6. 所有权检查

* `OwnershipChecker` 可以。

## 7. 字节码生成

* `CodeGenerator` 将 `HIR` 转换为 `CodeObj`。 `CodeObj` 保存字节码和执行配置。特别重要的是`CodeGenerator::compile_expr`。

---

* 以上所有的处理都是由`Compiler`作为一个门面组合起来的。
* 当然 Python 会执行生成的字节码，称为 `DummyVM`。