# Day 7

- 利用 `HashMap<K,V>` 模擬樹狀結構。
- 因為需要不斷更新 tree 的內容，因此使用 mutable 變數來管理。

## 可改進方向

1. 目錄的 size 是否可以使用 lazy 方式 update？
2. 為了避開 mutable borrow 可能發生的問題、以及為了實作「回到 tree 上一層」的功能，path 和 current dir 分開儲存。是否有更好的方法，不用存 path，直接使用 reference to node 追蹤就好？
3. 有沒有辦法更 functional 處理 tree construction？
4. `FsNode` 很直觀地分成 `File` 和 `Dir` 兩種 variants。但處理起來有些不太方便（例如就是想要取得 `size`，還是要依 variant 的不同用不同方法取用。有沒有更好的方法？