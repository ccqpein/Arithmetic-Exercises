# README #
[source](https://leetcode.com/problems/count-binary-substrings/)

Given a binary string `s`, return the number of non-empty substrings that have the same number of `0`'s and `1`'s, and all the `0`'s and all the `1`'s in these substrings are grouped consecutively.

Substrings that occur multiple times are counted the number of times they occur.


**Example 1:**

```
**Input:** s = &quot;00110011&quot;
**Output:** 6
**Explanation:** There are 6 substrings that have equal number of consecutive 1's and 0's: &quot;0011&quot;, &quot;01&quot;, &quot;1100&quot;, &quot;10&quot;, &quot;0011&quot;, and &quot;01&quot;.
Notice that some of these substrings repeat and are counted the number of times they occur.
Also, &quot;00110011&quot; is not a valid substring because all the 0's (and 1's) are not grouped together.
```

**Example 2:**

```
**Input:** s = &quot;10101&quot;
**Output:** 4
**Explanation:** There are 4 substrings: &quot;10&quot;, &quot;01&quot;, &quot;10&quot;, &quot;01&quot; that have equal number of consecutive 1's and 0's.
```


**Constraints:**


+ `1 <= s.length <= 10^5`
+ `s[i]` is either `'0'` or `'1'`.


