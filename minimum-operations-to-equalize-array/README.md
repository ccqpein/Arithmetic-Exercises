# README #
[source](https://leetcode.com/problems/minimum-operations-to-equalize-array/)

You are given an integer array `nums` of length `n`.

In one operation, choose any subarray `nums[l...r]` (`0 <= l <= r < n`) and **replace** each element in that subarray with the **bitwise AND** of all elements.

Return the **minimum** number of operations required to make all elements of `nums` equal.
A **subarray** is a contiguous **non-empty** sequence of elements within an array.

**Example 1:**

```
**Input:** <span class="example-io">nums = [1,2]</span>

**Output:** <span class="example-io">1</span>

**Explanation:**

Choose `nums[0...1]`: `(1 AND 2) = 0`, so the array becomes `[0, 0]` and all elements are equal in 1 operation.
```

**Example 2:**

```
**Input:** <span class="example-io">nums = [5,5,5]</span>

**Output:** <span class="example-io">0</span>

**Explanation:**

`nums` is `[5, 5, 5]` which already has all elements equal, so 0 operations are required.
```


**Constraints:**


+ `1 <= n == nums.length <= 100`
+ `1 <= nums[i] <= 10^5`


