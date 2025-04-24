# README #
[source](https://leetcode.com/problems/make-array-non-decreasing/)

You are given an integer array `nums`. In one operation, you can select a <span data-keyword="subarray-nonempty">subarray</span> and replace it with a single element equal to its **maximum** value.

Return the **maximum possible size** of the array after performing zero or more operations such that the resulting array is **non-decreasing**.


**Example 1:**

```
**Input:** <span class="example-io">nums = [4,2,5,3,5]</span>

**Output:** <span class="example-io">3</span>

**Explanation:**

One way to achieve the maximum size is:


+ Replace subarray `nums[1..2] = [2, 5]` with `5` &rarr; `[4, 5, 3, 5]`.
+ Replace subarray `nums[2..3] = [3, 5]` with `5` &rarr; `[4, 5, 5]`.


The final array `[4, 5, 5]` is non-decreasing with size <font face="monospace">3.</font>
```

**Example 2:**

```
**Input:** <span class="example-io">nums = [1,2,3]</span>

**Output:** <span class="example-io">3</span>

**Explanation:**

No operation is needed as the array `[1,2,3]` is already non-decreasing.
```


**Constraints:**


+ `1 <= nums.length <= 2 * 10^5`
+ `1 <= nums[i] <= 2 * 10^5`


