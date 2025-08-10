# README #
[source](https://leetcode.com/problems/count-unreachable-pairs-of-nodes-in-an-undirected-graph/)

You are given an integer `n`. There is an **undirected** graph with `n` nodes, numbered from `0` to `n - 1`. You are given a 2D integer array `edges` where `edges[i] = [a<sub>i</sub>, b<sub>i</sub>]` denotes that there exists an **undirected** edge connecting nodes `a<sub>i</sub>` and `b<sub>i</sub>`.

Return the **number of pairs** of different nodes that are **unreachable** from each other.


**Example 1:**
<img alt="" src="https://assets.leetcode.com/uploads/2022/05/05/tc-3.png" style="width: 267px; height: 169px;" />
```
**Input:** n = 3, edges = [[0,1],[0,2],[1,2]]
**Output:** 0
**Explanation:** There are no pairs of nodes that are unreachable from each other. Therefore, we return 0.
```

**Example 2:**
<img alt="" src="https://assets.leetcode.com/uploads/2022/05/05/tc-2.png" style="width: 295px; height: 269px;" />
```
**Input:** n = 7, edges = [[0,2],[0,5],[2,4],[1,6],[5,4]]
**Output:** 14
**Explanation:** There are 14 pairs of nodes that are unreachable from each other:
[[0,1],[0,3],[0,6],[1,2],[1,3],[1,4],[1,5],[2,3],[2,6],[3,4],[3,5],[3,6],[4,6],[5,6]].
Therefore, we return 14.
```


**Constraints:**


+ `1 <= n <= 10^5`
+ `0 <= edges.length <= 2 * 10^5`
+ `edges[i].length == 2`
+ `0 <= a<sub>i</sub>, b<sub>i</sub> < n`
+ `a<sub>i</sub> != b<sub>i</sub>`
+ There are no repeated edges.


