# README #
[source](https://leetcode.com/problems/count-partitions-with-even-sum-difference/)

You are given an integer array `nums` of length `n`.

A **partition** is defined as an index `i` where `0 <= i < n - 1`, splitting the array into two **non-empty** subarrays such that:


+ Left subarray contains indices `[0, i]`.
+ Right subarray contains indices `[i + 1, n - 1]`.


Return the number of **partitions** where the **difference** between the **sum** of the left and right subarrays is **even**.


**Example 1:**

```
**Input:** <span class="example-io">nums = [10,10,3,7,6]</span>

**Output:** <span class="example-io">4</span>

**Explanation:**

The 4 partitions are:


+ `[10]`, `[10, 3, 7, 6]` with a sum difference of `10 - 26 = -16`, which is even.
+ `[10, 10]`, `[3, 7, 6]` with a sum difference of `20 - 16 = 4`, which is even.
+ `[10, 10, 3]`, `[7, 6]` with a sum difference of `23 - 13 = 10`, which is even.
+ `[10, 10, 3, 7]`, `[6]` with a sum difference of `30 - 6 = 24`, which is even.

```

**Example 2:**

```
**Input:** <span class="example-io">nums = [1,2,2]</span>

**Output:** <span class="example-io">0</span>

**Explanation:**

No partition results in an even sum difference.
```

**Example 3:**

```
**Input:** <span class="example-io">nums = [2,4,6,8]</span>

**Output:** <span class="example-io">3</span>

**Explanation:**

All partitions result in an even sum difference.
```


**Constraints:**


+ `2 <= n == nums.length <= 100`
+ `1 <= nums[i] <= 100`


