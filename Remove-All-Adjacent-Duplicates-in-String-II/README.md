# README #

[source](https://leetcode.com/problems/remove-all-adjacent-duplicates-in-string-ii/)

You are given a string `s` and an integer `k`, a `k` **duplicate removal** consists of choosing `k` adjacent and equal letters from `s` and removing them, causing the left and the right side of the deleted substring to concatenate together.

We repeatedly make `k` **duplicate removals** on `s` until we no longer can.

Return the final string after all such duplicate removals have been made. It is guaranteed that the answer is unique.


**Example 1:**

```
**Input:** s = &quot;abcd&quot;, k = 2
**Output:** &quot;abcd&quot;
**Explanation: **There&#39;s nothing to delete.
```


**Example 2:**

```
**Input:** s = &quot;deeedbbcccbdaa&quot;, k = 3
**Output:** &quot;aa&quot;
&lt;strong&gt;Explanation: 
&lt;/strong&gt;First delete &quot;eee&quot; and &quot;ccc&quot;, get &quot;ddbbbdaa&quot;
Then delete &quot;bbb&quot;, get &quot;dddaa&quot;
Finally delete &quot;ddd&quot;, get &quot;aa&quot;
```


**Example 3:**

```
**Input:** s = &quot;pbbcggttciiippooaais&quot;, k = 2
**Output:** &quot;ps&quot;

```



**Constraints:**


	+ `1 &lt;= s.length &lt;= 105`
	+ `2 &lt;= k &lt;= 104`
	+ `s` only contains lower case English letters.



