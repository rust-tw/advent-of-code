# 2023 Day 16

題目本身其實沒有什麼特別好講的，就是直接在 2D 地圖中照著地型移動，最後算走過的格子數。

之所以寫 README.txt，只是想記錄一下有用到 `std::ops::{Index, IndexMut}`。用起來其實有點麻煩（需要另外宣告 container type，還要窮舉 `enum` 對應到的欄位），不像 C++ 可以直接把 `enum` 的 variant cast 成 integer。不過這也算是為了安全性的必要之惡吧！
