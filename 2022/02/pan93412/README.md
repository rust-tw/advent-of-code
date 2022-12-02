# AOC 2022 Day 2 Answer

程式碼有完整註釋和測試，這裡只說明一些比較有趣的地方。

## 題目

<https://adventofcode.com/2022/day/2>

## 執行

改 `inputs.txt` 之後 `cargo run --release`。

## 思路

### 如何拆解題目邏輯？

這個題目可以簡單分成四個部分：

- “Their” Response: 對手出了什麼拳？
- “Our” Response: 我們出了什麼拳？
- “Our” Strategy: 我們的策略是什麼？
- Outcome: 結果是輸、贏還是平手？

Response 可以先轉為統一的 `Response` 方便進行輸贏判斷；Outcome 和 Strategy 也可以用 `enum` 表示，而且有著極其相似的 fields。

### Tokenize: 將輸入內容變成好讀的架構

輸入內容大概長這樣：

```plain
A Y
B X
C Z
```

其實可以很直覺想到「噢，A 跟 Y 可以表示成 Tuple，而每一列可以表示成 Vec。」確定好資料結構之後，我們便可以寫一個 `Tokenizer` 將資料轉成想要的架構。

考慮到一列的長度是等長的 (`A|B|C X|Y|Z`)，所以我們直接跑 `chars()` 然後用 `.next()` 將每個字元取出即可。也可以使用 `as_bytes()` 獲得更高速的效能，但考慮到整個程式階段只會做這麼一次 deserialization，所以就沒有這個必要。

完整程式碼見 [`src/tokenizer.rs`](src/tokenizer.rs)。

### Response: 讓 A、B、C 變成一個人類也讀得懂的策略

我們當然可以直接判斷 $A, B, C$ vs $X, Y, Z$，但這樣有幾個問題：

- 不怕你在寫這些判斷程式碼的時候，把這些字母的涵義搞混嗎？
- 即便您一下子就能看出來——下一個讀程式碼的人，知道這些字母是什麼涵義嗎？
- 平局和 win/lost 判斷轉成同一的結構會不會比較好算？

所以我自己的做法，是把這些看不懂的字母轉換成 Enumeration。這樣不僅讓程式碼更好讀、增強類型安全，也讓判斷上更為直覺容易：

```rs
pub enum Response {
    /// 石頭 (1pt)
    Rock,
    /// 布 (2pt)
    Paper,
    /// 剪刀 (3pt)
    Scissors,
}
```

接下來就是把這些字母轉成 `Response`，然後算點數。詳細資訊請見 [`src/response.rs`](./src/response.rs)。

### Strategy: 把策略變成 Enumeration

跟〈Response〉一段的理由相似，我們也把 $X, Y, Z$ 轉成 Enumeration：

```rs
pub enum Strategy {
    /// 嘗試輸掉遊戲
    DoLose,
    /// 嘗試平局
    DoDraw,
    /// 嘗試贏得遊戲
    DoWin,
}
```

剩下的就是做出相應的判斷（策略只會有一個解而已 XD）比較有趣的是，這裡我做了一件有點「工程」的事情：我把計算策略的函數，透過 `trait` 將其當作 Response 的擴充方法來寫。這樣子就不需要再包一個 `Strategy` 的 struct，同時也把功能之間的關係寫得很清楚。

詳細資訊請見 [`src/strategy.rs`](./src/strategy.rs)。

### *Lists: 將 Tokenized 的內容轉換成對應的結構

存取的資料有兩種可能：

    對方出法    我方出法

和：

    對方出法    我方策略

因此我寫了兩個 Lists 將輸入的原始資料特化，並在其中寫上 `point()` 和 `find_best_solution()` 的封裝。

詳細資訊請見以下兩個檔案：

- [`src/response/list.rs`](./src/response/list.rs)
- [`src/strategy/list.rs`](./src/strategy/list.rs)

### Solution: 封裝輸入的內容

這裡基本上只是呼叫資料結構提供的方法而已，可以想像成 MVC 架構中的 View（或者說 Service-Controller 架構中的 Controller）。但裡面有個小巧思：裡面存放的資料結構是 `Vec<(char, char)>` 而不是上述任何一個特化過的結構。

為何要這麼做？因為我方 (our) 的 $X, Y, Z$ 有兩種意思：「出法」和「策略」。如果把它轉換成上述任何一個結構，必定會弱化其中一方的意思。因此就乾脆維持原樣，讓 `Solution` 這個結構只存放原始的 deserialized 結果。

詳細程式碼見 [`src/solution.rs`](./src/solution.rs)。

## 授權條款

AGPL-3.0-only
