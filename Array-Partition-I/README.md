# README #

[source](https://leetcode.com/problems/array-partition-i/)

Given an integer array `nums` of `2n` integers, group these integers into `n` pairs `(a&lt;sub&gt;1&lt;/sub&gt;, b&lt;sub&gt;1&lt;/sub&gt;), (a&lt;sub&gt;2&lt;/sub&gt;, b&lt;sub&gt;2&lt;/sub&gt;), ..., (a&lt;sub&gt;n&lt;/sub&gt;, b&lt;sub&gt;n&lt;/sub&gt;)` such that the sum of `min(a&lt;sub&gt;i&lt;/sub&gt;, b&lt;sub&gt;i&lt;/sub&gt;)` for all `i` is **maximized**. Return* the maximized sum*.


**Example 1:**

```
**Input:** nums = [1,4,3,2]
**Output:** 4
**Explanation:** All possible pairings (ignoring the ordering of elements) are:
1. (1, 4), (2, 3) -&amp;gt; min(1, 4) + min(2, 3) = 1 + 2 = 3
2. (1, 3), (2, 4) -&amp;gt; min(1, 3) + min(2, 4) = 1 + 2 = 3
3. (1, 2), (3, 4) -&amp;gt; min(1, 2) + min(3, 4) = 1 + 3 = 4
So the maximum possible sum is 4.
```


**Example 2:**

```
**Input:** nums = [6,2,6,5,1,2]
**Output:** 9
**Explanation:** The optimal pairing is (2, 1), (2, 5), (6, 6). min(2, 1) + min(2, 5) + min(6, 6) = 1 + 2 + 6 = 9.

```



**Constraints:**


	+ `1 &lt;= n &lt;= 104`
	+ `nums.length == 2 * n`
	+ `-104 &lt;= nums[i] &lt;= 104`



