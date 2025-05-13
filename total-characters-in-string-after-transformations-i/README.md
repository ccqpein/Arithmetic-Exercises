# README #
[source](https://leetcode.com/problems/total-characters-in-string-after-transformations-i/)

You are given a string `s` and an integer `t`, representing the number of **transformations** to perform. In one **transformation**, every character in `s` is replaced according to the following rules:


+ If the character is `'z'`, replace it with the string `"ab"`.
+ Otherwise, replace it with the **next** character in the alphabet. For example, `'a'` is replaced with `'b'`, `'b'` is replaced with `'c'`, and so on.


Return the **length** of the resulting string after **exactly** `t` transformations.

Since the answer may be very large, return it **modulo**<!-- notionvc: eb142f2b-b818-4064-8be5-e5a36b07557a --> `10^9 + 7`.


**Example 1:**

```
**Input:** <span class="example-io">s = "abcyy", t = 2</span>

**Output:** <span class="example-io">7</span>

**Explanation:**


+ **First Transformation (t = 1)**:

	
+ `'a'` becomes `'b'`
+ `'b'` becomes `'c'`
+ `'c'` becomes `'d'`
+ `'y'` becomes `'z'`
+ `'y'` becomes `'z'`
+ String after the first transformation: `"bcdzz"`
	
	
+ **Second Transformation (t = 2)**:
	
+ `'b'` becomes `'c'`
+ `'c'` becomes `'d'`
+ `'d'` becomes `'e'`
+ `'z'` becomes `"ab"`
+ `'z'` becomes `"ab"`
+ String after the second transformation: `"cdeabab"`
	
	
+ **Final Length of the string**: The string is `"cdeabab"`, which has 7 characters.

```

**Example 2:**

```
**Input:** <span class="example-io">s = "azbk", t = 1</span>

**Output:** <span class="example-io">5</span>

**Explanation:**


+ **First Transformation (t = 1)**:

	
+ `'a'` becomes `'b'`
+ `'z'` becomes `"ab"`
+ `'b'` becomes `'c'`
+ `'k'` becomes `'l'`
+ String after the first transformation: `"babcl"`
	
	
+ **Final Length of the string**: The string is `"babcl"`, which has 5 characters.

```


**Constraints:**


+ `1 <= s.length <= 10^5`
+ `s` consists only of lowercase English letters.
+ `1 <= t <= 10^5`


