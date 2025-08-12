# README #
[source](https://leetcode.com/problems/flip-square-submatrix-vertically/)

You are given an `m x n` integer matrix `grid`, and three integers `x`, `y`, and `k`.

The integers `x` and `y` represent the row and column indices of the **top-left** corner of a **square** submatrix and the integer `k` represents the size (side length) of the square submatrix.

Your task is to flip the submatrix by reversing the order of its rows vertically.

Return the updated matrix.


**Example 1:**
<img alt="" src="https://assets.leetcode.com/uploads/2025/07/20/gridexmdrawio.png" style="width: 300px; height: 116px;" />
```
**Input:** <span class="example-io">grid = </span>[[1,2,3,4],[5,6,7,8],[9,10,11,12],[13,14,15,16]]<span class="example-io">, x = 1, y = 0, k = 3</span>

**Output:** <span class="example-io">[[1,2,3,4],[13,14,15,8],[9,10,11,12],[5,6,7,16]]</span>

**Explanation:**

The diagram above shows the grid before and after the transformation.
```

**Example 2:**
<img alt="" src="https://assets.leetcode.com/uploads/2025/07/20/gridexm2drawio.png" style="width: 350px; height: 68px;" />​​​​​​​
```
**Input:** <span class="example-io">grid = [[3,4,2,3],[2,3,4,2]], x = 0, y = 2, k = 2</span>

**Output:** <span class="example-io">[[3,4,4,2],[2,3,2,3]]</span>

**Explanation:**

The diagram above shows the grid before and after the transformation.
```


**Constraints:**


+ `m == grid.length`
+ `n == grid[i].length`
+ `1 <= m, n <= 50`
+ `1 <= grid[i][j] <= 100`
+ `0 <= x < m`
+ `0 <= y < n`
+ `1 <= k <= min(m - x, n - y)`


