# README #
[source](https://leetcode.com/problems/find-the-xor-of-numbers-which-appear-twice/)

You are given an array `nums`, where each number in the array appears **either** once or twice.

Return the bitwise `XOR` of all the numbers that appear twice in the array, or 0 if no number appears twice.


**Example 1:**

```
**Input:** <span class="example-io">nums = [1,2,1,3]</span>

**Output:** <span class="example-io">1</span>

**Explanation:**

The only number that appears twice in`nums`is 1.
```

**Example 2:**

```
**Input:** <span class="example-io">nums = [1,2,3]</span>

**Output:** <span class="example-io">0</span>

**Explanation:**

No number appears twice in`nums`.
```

**Example 3:**

```
**Input:** <span class="example-io">nums = [1,2,2,1]</span>

**Output:** <span class="example-io">3</span>

**Explanation:**

Numbers 1 and 2 appeared twice. `1 XOR 2 == 3`.
```


**Constraints:**


+ `1 <= nums.length <= 50`
+ `1 <= nums[i] <= 50`
+ Each number in `nums` appears either once or twice.


