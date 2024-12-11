# 2024/02 hms5232

https://adventofcode.com/2024/day/2

## 如何使用

在此專案目錄（和本檔案同層）新增一個 `input.txt` 或其他喜歡的檔案名稱並更改 `src/main.rs` 程式碼讀取的檔案。

把題目給的 input 直接放入剛剛的檔案，然後直接 `cargo run` 就會算出答案。

## 題目說明

題目的 input 會有多行，稱為 `report`；每個 report 由多個以單一空白分隔的數字組成，稱為 `level`。

### part 1

一個 _safe_ 的 report 有兩個條件:

1. level 必須遞增或遞減排列
2. 1 <= `相鄰的 level 差` <= 3

### part 2

承 part 1，新增一條規則：

3. 可以容忍一個 level 不合格（tolerate a single bad level），此 level 將被移除後才檢查

> Now, the same rules apply as before, except if removing a single level from an unsafe report would make it safe, the report instead counts as safe.
 
## 解法說明

### part 1

這邊拆成兩道關卡，第一道檢查排序，第二道檢查距離：

1. 由於 Rust 向量的 `is_sorted()` 只能檢查遞增，如果要檢查遞減，直接反轉向量後再比較即可。

2. 而相鄰 level 差距可以當成距離，直接用兩者差的絕對值比較。

### part 2

由於可以容忍一個 bad 的 level，一開始因此可以先移除這個 level，然後再檢查是否為 safe。

剛開始為了減少運算時間，在排序的階段我一找到移除後即完成排序的 level 就立刻回傳，可是隨後發現這樣行不通，因為可能出現一個 report 有多個可以被移除的選擇，例如：

```txt
24 27 30 31 33 35 33
```

此例子中，移除最後的 35 或 33 都可以達成排序，但如果移除 35，隨後的絕對值檢查就會失敗；若選擇移除最後的 33，就是 safe 的 report。

最後採用另外一招──迴圈逐一檢查，只要在任一階段兩道關卡都通過就是 safe。雖然我認為很沒效率，但他能動。

看之後其他人有沒有更好的解法，這邊附上卡關時想的各種測試資料：

```txt
5 4 3 2 9
15 8 5 4 3
3 4 5 9 10
17 17 15 12 11 9 7 5
37 41 42 44 45 46 49 51
24 27 30 31 33 35 33
```
