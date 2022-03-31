# README #

[source](https://leetcode.com/problems/matrix-block-sum/)

Given a `m x n` matrix `mat` and an integer `k`, return *a matrix* `answer` *where each* `answer[i][j]` *is the sum of all elements* `mat[r][c]` *for*:


	+ `i - k &lt;= r &lt;= i + k,`
	+ `j - k &lt;= c &lt;= j + k`, and
	+ `(r, c)` is a valid position in the matrix.



**Example 1:**

```
**Input:** mat = [[1,2,3],[4,5,6],[7,8,9]], k = 1
**Output:** [[12,21,16],[27,45,33],[24,39,28]]

```


**Example 2:**

```
**Input:** mat = [[1,2,3],[4,5,6],[7,8,9]], k = 2
**Output:** [[45,45,45],[45,45,45],[45,45,45]]

```



**Constraints:**


	+ `m ==mat.length`
	+ `n ==mat[i].length`
	+ `1 &lt;= m, n, k &lt;= 100`
	+ `1 &lt;= mat[i][j] &lt;= 100`



