# `ergc` 的架構

[![badge](https://img.shields.io/endpoint.svg?url=https%3A%2F%2Fgezf7g7pd5.execute-api.ap-northeast-1.amazonaws.com%2Fdefault%2Fsource_up_to_date%3Fowner%3Derg-lang%26repos%3Derg%26ref%3Dmain%26path%3Ddoc/EN/compiler/architecture.md%26commit_hash%3Da711efa99b325ba1012f6897e7b0e2bdb947d8a1)](https://gezf7g7pd5.execute-api.ap-northeast-1.amazonaws.com/default/source_up_to_date?owner=erg-lang&repos=erg&ref=main&path=doc/EN/compiler/architecture.md&commit_hash=a711efa99b325ba1012f6897e7b0e2bdb947d8a1)

## 1. 掃描 Erg 腳本 (.er) 并生成 `TokenStream` (parser/lex.rs)

* parser/lexer/Lexer 生成`TokenStream`(這是一個Token的迭代器，TokenStream可以通過lexer.collect()生成)
  * `Lexer` 由 `Lexer::new` 或 `Lexer::from_str` 構造，其中 `Lexer::new` 從文件或命令選項中讀取代碼。
  * `Lexer` 可以作為迭代器按順序生成令牌；如果您想一次獲得 `TokenStream`，請使用 `Lexer::lex`。
  * `Lexer` 輸出 `LexError` 為錯誤，但 `LexError` 沒有足夠的信息顯示自己。如果要顯示錯誤，請使用 `LexerRunner` 轉換錯誤。
  * 如果你想單獨使用 `Lexer`，也可以使用 `LexerRunner`；`Lexer` 只是一個迭代器，并沒有實現 `Runnable` 特性。
    * `Runnable` 由 `LexerRunner`、`ParserRunner`、`Compiler` 和 `VirtualMachine` 實現。

## 2. 轉換 `TokenStream` -> `AST` (parser/parse.rs)

* `Parser` 和 `Lexer` 一樣，有兩個構造函數，`Parser::new` 和 `Parser::from_str`，而 `Parser::parse` 會給出 `AST`。
* `AST` 是 `Vec<Expr>` 的包裝器類型。

### 2.5 脫糖 `AST`

* 擴展嵌套變量 (`Desugarer::desugar_nest_vars_pattern`)
* desugar 多模式定義語法(`Desugarer::desugar_multiple_pattern_def`)

## 3. 類型檢查和推斷，轉換 `AST` -> `HIR` (compiler/lower.rs)

* `HIR` 有每個變量的類型信息。它是用于"高級中間表示"的。
* `HIR` 只保存變量的類型，但這已經足夠了。在極端情況下，這是因為 Erg 只有轉換(或運算符)應用程序的參數對象。
* `ASTLower` 可以用與`Parser` 和`Lexer` 相同的方式構造。
* 如果沒有錯誤發生，`ASTLowerer::lower` 將輸出 `HIR` 和 `CompileWarnings` 的元組。
* `ASTLowerer`歸`Compiler`所有。與傳統結構不同，`ASTLowerer`處理代碼上下文并且不是一次性的。
* 如果類型推斷的結果不完整(如果存在未知類型變量)，名稱解析時會出錯。

## 4. 檢查副作用(compiler/effectcheck.rs)

## 4. 檢查所有權(compiler/memcheck.rs)

## 5. 從`HIR`(compiler/codegen.rs)生成字節碼(`CodeObj`)

* 根據表達式的類型信息，將執行量化子程序的名稱解析。

##(6.(未來計劃)轉換字節碼 -> LLVM IR)

* 字節碼是基于堆棧的，而 LLVM IR 是基于寄存器的。
  這個轉換過程會多出幾層中間過程。