# Day 15 雜談

1. 由於從 sensor 到 beacon 之間的距離是用 Manhattan distance 計算的，因此我們知道兩者的座標後，就可以算出距離 = dx + dy。有了距離，就可以得知 sensor 掃描的範圍，然後可以算出「從 sensor 往上/下 n 行時，掃描範圍的 x 軸極左與極右值」。

2. Part 1 就只是在 y = 200000 那條線上、所有的 sensor 掃描範圍的聯集。但要注意的是，這個範圍中可能真的會有 beacon 存在，因此也要額外扣掉 beacon 的數量。

3. Part 2 我目前是基於 part 1 的演算法，用暴力法去解：從 y = 0 到 y = 4_000_000，去看看是否每一條線都完整被掃描到。但這樣做其實速度有點慢。在我的 NB 上（CPU: AMD Ryzen 9 6900HS 3.3GHz），dev build 要跑 27 秒左右；而 release build 也需要 2.7 秒。
   不過後來我用 [rayon](https://crates.io/crates/rayon) 把逐行掃描平行化，這樣跑下來就只需要 200+ ms。雖然有點偷吃步，但也能展示平行處理的威力，以及 rayon 多麼容易使用（事實上我只在 iterator chain 中多加了一行而已）。
