# README #

[source](https://leetcode.com/problems/find-all-k-distant-indices-in-an-array/)

You are given a **0-indexed** integer array `nums` and two integers `key` and `k`. A **k-distant index** is an index `i` of `nums` for which there exists at least one index `j` such that `|i - j| &lt;= k` and `nums[j] == key`.

Return *a list of all k-distant indices sorted in **increasing order***.


&lt;strong class=&quot;example&quot;&gt;Example 1:&lt;/strong&gt;

```
**Input:** nums = [3,4,9,1,3,9,5], key = 9, k = 1
**Output:** [1,2,3,4,5,6]
**Explanation:** Here, `nums[2] == key` and &lt;code&gt;nums[5] == key.
- For index 0, |0 - 2| &amp;gt; k and |0 - 5| &amp;gt; k, so there is no j&lt;/code&gt; where `|0 - j| &lt;= k` and &lt;code&gt;nums[j] == key. Thus, 0 is not a k-distant index.
- For index 1, |1 - 2| &lt;= k and nums[2] == key, so 1 is a k-distant index.
- For index 2, |2 - 2| &lt;= k and nums[2] == key, so 2 is a k-distant index.
- For index 3, |3 - 2| &lt;= k and nums[2] == key, so 3 is a k-distant index.
- For index 4, |4 - 5| &lt;= k and nums[5] == key, so 4 is a k-distant index.
- For index 5, |5 - 5| &lt;= k and nums[5] == key, so 5 is a k-distant index.
- For index 6, |6 - 5| &lt;= k and nums[5] == key, so 6 is a k-distant index.
&lt;/code&gt;Thus, we return [1,2,3,4,5,6] which is sorted in increasing order. 

```


&lt;strong class=&quot;example&quot;&gt;Example 2:&lt;/strong&gt;

```
**Input:** nums = [2,2,2,2,2], key = 2, k = 2
**Output:** [0,1,2,3,4]
**Explanation:** For all indices i in nums, there exists some index j such that |i - j| &lt;= k and nums[j] == key, so every index is a k-distant index. 
Hence, we return [0,1,2,3,4].

```



**Constraints:**


	+ `1 &lt;= nums.length &lt;= 1000`
	+ `1 &lt;= nums[i] &lt;= 1000`
	+ `key` is an integer from the array `nums`.
	+ `1 &lt;= k &lt;= nums.length`



