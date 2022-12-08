# Day 8

- Part 1 使用 Dynamic Programming 的方式解。
- Part 2 如果用暴力解法，時間複雜度會是 $O(W^2 \cdot H^2)$，其中 W 為寬度，H 為高度。

  後來使用的方法是利用一個 array 記錄最後一次遇到高度為 n 的樹是在什麼位置。利用這個 array 就可以在 constant time 算出最遠能看到多少顆樹。最後的複雜度變成 $O(W \cdot H)$。

## 待改進方向

1. 4 個方向的演算法是否可以找到更通用的？其中 Part 1 已經使用 trait `Fn` 盡可能通用化了。但 Part 2 感覺還能找到更好的方法。（目前想到的是，`values` 在進去計算前，也許可以先 rearrange 成更適合演算法的格式？）

2. 現在 2D arrays 都是用 `Vec<Vec<T>>` 來儲存。也許可以考慮使用 1D array 來模擬。這樣還以使用 [transpose](https://docs.rs/transpose/latest/transpose/) 這樣的 crate 來幫忙處理。

3. 也許 Part 1 也可以不要用 DP，改用 Part 2 的 iteration 方式來做。

4. Part 2 中的 count tables，是否可以用 iterator 直接產生，少用 mutable variables？
