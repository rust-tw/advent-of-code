# Advent of Code 2022 / Pan93412's Solution

## Original Question

<https://adventofcode.com/2021/day/1>

## 解析資料

```plain
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
```

可以先解析成這樣的區塊：

```python
[
    [1000, 2000, 3000],
    [4000],
    [5000, 6000],
    [7000, 8000, 9000],
    [10000]
]
```

然後把這些區塊進行加總：

```python
[
    6000,
    4000,
    11000,
    24000,
    10000
]
```

Rust 上可以利用 `.split()` 將資料進行分割。
我們可以看到一個 block 剛好有兩個換行 (`\n\n`)，
所以可以先將資料切成區塊：

```rs
data.split("\n\n")
```

`.trim()` 之後發現每個數字剛好為一列，所以切 `\n`：

```rs
blocks.map(|blk| {
    let numbers = blk.split('\n');
    // ...
})
```

然後 `.parse()` 成 `u64`，之後加總這些數字。

## Question 1, Part one

Part one 是求 blocks 中的最大值。考慮到我們的
Question 2 會需要求出最大三值，所以我們用 sort
先對 `Vec::<u64>` 的 blocks 做排序：

```rs
blocks.sort_unstable();
```

`sort_unstable()` 為升冪排序，我們用
iterator 取出其最後一個值：

```rs
let answer_0 = *blocks.iter().last().unwrap();
```

注意兩點：

- 這裡用 `unwrap()` 是確定沒有「無 blocks」的情形。
  您也可以根據情況自行抉擇要不要進行錯誤處理。
- `last()` 回傳 `&u64`。考慮到 `u64` 是 `Copy` 的，
  這裡直接 _dereference_ 即可。

## Question 1, Part two

同 **Part 1**，但這裡取出倒數 3 個值並對其做加總：

```rs
let answer_1 = blocks.iter().rev().take(3).sum();
```

回傳值為 `u64`，可直接回傳之。

## License

`AGPL-3.0-only`
