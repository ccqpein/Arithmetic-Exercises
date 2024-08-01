# README #
[source](https://leetcode.com/problems/find-if-digit-game-can-be-won/)

You are given an array of **positive** integers `nums`.

Alice and Bob are playing a game. In the game, Alice can choose **either** all single-digit numbers or all double-digit numbers from `nums`, and the rest of the numbers are given to Bob. Alice wins if the sum of her numbers is **strictly greater** than the sum of Bob's numbers.

Return `true` if Alice can win this game, otherwise, return `false`.


**Example 1:**

```
**Input:** <span class="example-io">nums = [1,2,3,4,10]</span>

**Output:** <span class="example-io">false</span>

**Explanation:**

Alice cannot win by choosing either single-digit or double-digit numbers.
```

**Example 2:**

```
**Input:** <span class="example-io">nums = [1,2,3,4,5,14]</span>

**Output:** <span class="example-io">true</span>

**Explanation:**

Alice can win by choosing single-digit numbers which have a sum equal to 15.
```

**Example 3:**

```
**Input:** <span class="example-io">nums = [5,5,5,25]</span>

**Output:** <span class="example-io">true</span>

**Explanation:**

Alice can win by choosing double-digit numbers which have a sum equal to 25.
```


**Constraints:**


+ `1 <= nums.length <= 100`
+ `1 <= nums[i] <= 99`


