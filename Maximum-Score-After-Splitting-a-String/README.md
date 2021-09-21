# README #

[source](https://leetcode.com/problems/maximum-score-after-splitting-a-string/)

Given astring `s`of zeros and ones, *return the maximum score after splitting the string into two **non-empty** substrings* (i.e. **left** substring and **right** substring).

The score after splitting a string is the number of **zeros** in the **left** substring plus the number of **ones** in the **right** substring.


**Example 1:**

```
**Input:** s = &quot;011101&quot;
**Output:** 5 
**Explanation:** 
All possible ways of splitting s into two non-empty substrings are:
left = &quot;0&quot; and right = &quot;11101&quot;, score = 1 + 4 = 5 
left = &quot;01&quot; and right = &quot;1101&quot;, score = 1 + 3 = 4 
left = &quot;011&quot; and right = &quot;101&quot;, score = 1 + 2 = 3 
left = &quot;0111&quot; and right = &quot;01&quot;, score = 1 + 1 = 2 
left = &quot;01110&quot; and right = &quot;1&quot;, score = 2 + 1 = 3

```


**Example 2:**

```
**Input:** s = &quot;00111&quot;
**Output:** 5
**Explanation:** When left = &quot;00&quot; and right = &quot;111&quot;, we get the maximum score = 2 + 3 = 5

```


**Example 3:**

```
**Input:** s = &quot;1111&quot;
**Output:** 3

```



**Constraints:**


	+ `2 &lt;= s.length &lt;= 500`
	+ The string `s` consists of characters &#39;0&#39; and &#39;1&#39; only.



