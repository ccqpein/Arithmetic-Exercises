# README #
[source](https://leetcode.com/problems/minimum-operations-to-make-array-equal-to-target/)

You are given two positive integer arrays `nums` and `target`, of the same length.

In a single operation, you can select any <span data-keyword="subarray">subarray</span> of `nums` and increment or decrement each element within that subarray by 1.

Return the **minimum** number of operations required to make `nums` equal to the array `target`.


**Example 1:**

```
**Input:** <span class="example-io">nums = [3,5,1,2], target = [4,6,2,4]</span>

**Output:** <span class="example-io">2</span>

**Explanation:**

We will perform the following operations to make `nums` equal to `target`:**
- Increment`nums[0..3]` by 1, `nums = [4,6,2,3]`.**
- Increment`nums[3..3]` by 1, `nums = [4,6,2,4]`.
```

**Example 2:**

```
**Input:** <span class="example-io">nums = [1,3,2], target = [2,1,4]</span>

**Output:** <span class="example-io">5</span>

**Explanation:**

We will perform the following operations to make `nums` equal to `target`:**
- Increment`nums[0..0]` by 1, `nums = [2,3,2]`.**
- Decrement`nums[1..1]` by 1, `nums = [2,2,2]`.**
- Decrement`nums[1..1]` by 1, `nums = [2,1,2]`.**
- Increment`nums[2..2]` by 1, `nums = [2,1,3]`.**
- Increment`nums[2..2]` by 1, `nums = [2,1,4]`.
```


**Constraints:**


+ `1 <= nums.length == target.length <= 10^5`
+ `1 <= nums[i], target[i] <= 10^8`


