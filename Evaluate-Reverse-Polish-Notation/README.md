# README #

[source](https://leetcode.com/problems/evaluate-reverse-polish-notation/)

Evaluate the value of an arithmetic expression in &lt;a href=&quot;http://en.wikipedia.org/wiki/Reverse_Polish_notation&quot; target=&quot;_blank&quot;&gt;Reverse Polish Notation&lt;/a&gt;.

Valid operators are `+`, `-`, `*`, and `/`. Each operand may be an integer or another expression.

**Note** that division between two integers should truncate toward zero.

It is guaranteed that the given RPN expression is always valid. That means the expression would always evaluate to a result, and there will not be any division by zero operation.


**Example 1:**

```
**Input:** tokens = [&quot;2&quot;,&quot;1&quot;,&quot;+&quot;,&quot;3&quot;,&quot;*&quot;]
**Output:** 9
**Explanation:** ((2 + 1) * 3) = 9

```


**Example 2:**

```
**Input:** tokens = [&quot;4&quot;,&quot;13&quot;,&quot;5&quot;,&quot;/&quot;,&quot;+&quot;]
**Output:** 6
**Explanation:** (4 + (13 / 5)) = 6

```


**Example 3:**

```
**Input:** tokens = [&quot;10&quot;,&quot;6&quot;,&quot;9&quot;,&quot;3&quot;,&quot;+&quot;,&quot;-11&quot;,&quot;*&quot;,&quot;/&quot;,&quot;*&quot;,&quot;17&quot;,&quot;+&quot;,&quot;5&quot;,&quot;+&quot;]
**Output:** 22
**Explanation:** ((10 * (6 / ((9 + 3) * -11))) + 17) + 5
= ((10 * (6 / (12 * -11))) + 17) + 5
= ((10 * (6 / -132)) + 17) + 5
= ((10 * 0) + 17) + 5
= (0 + 17) + 5
= 17 + 5
= 22

```



**Constraints:**


	+ `1 &lt;= tokens.length &lt;= 104`
	+ `tokens[i]` is either an operator: `&quot;+&quot;`, `&quot;-&quot;`, `&quot;*&quot;`, or `&quot;/&quot;`, or an integer in the range `[-200, 200]`.



