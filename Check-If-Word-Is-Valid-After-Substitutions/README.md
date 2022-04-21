# README #

[source](https://leetcode.com/problems/check-if-word-is-valid-after-substitutions/)

Given a string `s`, determine if it is **valid**.

A string `s` is **valid** if, starting with an empty string `t = &quot;&quot;`, you can **transform **`t`** into **`s` after performing the following operation **any number of times**:


	+ Insert string `&quot;abc&quot;` into any position in `t`. More formally, `t` becomes `t&lt;sub&gt;left&lt;/sub&gt; + &quot;abc&quot; + t&lt;sub&gt;right&lt;/sub&gt;`, where `t == t&lt;sub&gt;left&lt;/sub&gt; + t&lt;sub&gt;right&lt;/sub&gt;`. Note that `t&lt;sub&gt;left&lt;/sub&gt;` and `t&lt;sub&gt;right&lt;/sub&gt;` may be **empty**.


Return `true` *if *`s`* is a **valid** string, otherwise, return* `false`.


**Example 1:**

```
**Input:** s = &quot;aabcbc&quot;
**Output:** true
**Explanation:**
&quot;&quot; -&amp;gt; &quot;&lt;u&gt;abc&lt;/u&gt;&quot; -&amp;gt; &quot;a&lt;u&gt;abc&lt;/u&gt;bc&quot;
Thus, &quot;aabcbc&quot; is valid.
```


**Example 2:**

```
**Input:** s = &quot;abcabcababcc&quot;
**Output:** true
**Explanation:**
&quot;&quot; -&amp;gt; &quot;&lt;u&gt;abc&lt;/u&gt;&quot; -&amp;gt; &quot;abc&lt;u&gt;abc&lt;/u&gt;&quot; -&amp;gt; &quot;abcabc&lt;u&gt;abc&lt;/u&gt;&quot; -&amp;gt; &quot;abcabcab&lt;u&gt;abc&lt;/u&gt;c&quot;
Thus, &quot;abcabcababcc&quot; is valid.

```


**Example 3:**

```
**Input:** s = &quot;abccba&quot;
**Output:** false
**Explanation:** It is impossible to get &quot;abccba&quot; using the operation.

```



**Constraints:**


	+ `1 &lt;= s.length &lt;= 2 * 104`
	+ `s` consists of letters `&#39;a&#39;`, `&#39;b&#39;`, and `&#39;c&#39;`



