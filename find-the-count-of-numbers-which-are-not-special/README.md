# README #
[source](https://leetcode.com/problems/find-the-count-of-numbers-which-are-not-special/)

You are given 2 **positive** integers `l` and `r`. For any number `x`, all positive divisors of `x` except `x` are called the **proper divisors** of `x`.

A number is called **special** if it has exactly 2 **proper divisors**. For example:


+ The number 4 is special because it has proper divisors 1 and 2.
+ The number 6 is not special because it has proper divisors 1, 2, and 3.


Return the count of numbers in the range `[l, r]` that are **not** **special**.


**Example 1:**

```
**Input:** <span class="example-io">l = 5, r = 7</span>

**Output:** <span class="example-io">3</span>

**Explanation:**

There are no special numbers in the range `[5, 7]`.
```

**Example 2:**

```
**Input:** <span class="example-io">l = 4, r = 16</span>

**Output:** <span class="example-io">11</span>

**Explanation:**

The special numbers in the range `[4, 16]` are 4 and 9.
```


**Constraints:**


+ `1 <= l <= r <= 10^9`


