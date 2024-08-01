# README #
[source](https://leetcode.com/problems/number-of-bit-changes-to-make-two-integers-equal/)

You are given two positive integers `n` and `k`.

You can choose **any** bit in the **binary representation** of `n` that is equal to 1 and change it to 0.

Return the number of changes needed to make `n` equal to `k`. If it is impossible, return -1.


**Example 1:**

```
**Input:** <span class="example-io">n = 13, k = 4</span>

**Output:** <span class="example-io">2</span>

**Explanation:****
Initially, the binary representations of `n` and `k` are `n = (1101)<sub>2</sub>` and `k = (0100)<sub>2</sub>`.**
We can change the first and fourth bits of `n`. The resulting integer is `n = (<u>**0**</u>10<u>**0**</u>)<sub>2</sub> = k`.
```

**Example 2:**

```
**Input:** <span class="example-io">n = 21, k = 21</span>

**Output:** <span class="example-io">0</span>

**Explanation:****
`n` and `k` are already equal, so no changes are needed.
```

**Example 3:**

```
**Input:** <span class="example-io">n = 14, k = 13</span>

**Output:** <span class="example-io">-1</span>

**Explanation:****
It is not possible to make `n` equal to `k`.
```


**Constraints:**


+ `1 <= n, k <= 10^6`


