# README #
[source](https://leetcode.com/problems/peak-index-in-a-mountain-array/)

You are given an integer **mountain** array `arr` of length `n` where the values increase to a **peak element** and then decrease.

Return the index of the peak element.

Your task is to solve it in `O(log(n))` time complexity.


**Example 1:**

```
**Input:** <span class="example-io">arr = [0,1,0]</span>

**Output:** <span class="example-io">1</span>
```

**Example 2:**

```
**Input:** <span class="example-io">arr = [0,2,1,0]</span>

**Output:** <span class="example-io">1</span>
```

**Example 3:**

```
**Input:** <span class="example-io">arr = [0,10,5,2]</span>

**Output:** <span class="example-io">1</span>
```


**Constraints:**


+ `3 <= arr.length <= 10^5`
+ `0 <= arr[i] <= 10^6`
+ `arr` is **guaranteed** to be a mountain array.


