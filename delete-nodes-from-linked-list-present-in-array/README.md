# README #
[source](https://leetcode.com/problems/delete-nodes-from-linked-list-present-in-array/)

You are given an array of integers `nums` and the `head` of a linked list. Return the `head` of the modified linked list after **removing** all nodes from the linked list that have a value that exists in `nums`.


**Example 1:**

```
**Input:** <span class="example-io">nums = [1,2,3], head = [1,2,3,4,5]</span>

**Output:** <span class="example-io">[4,5]</span>

**Explanation:**

**<img alt="" src="https://assets.leetcode.com/uploads/2024/06/11/linkedlistexample0.png" style="width: 400px; height: 66px;" />**

Remove the nodes with values 1, 2, and 3.
```

**Example 2:**

```
**Input:** <span class="example-io">nums = [1], head = [1,2,1,2,1,2]</span>

**Output:** <span class="example-io">[2,2,2]</span>

**Explanation:**

<img alt="" src="https://assets.leetcode.com/uploads/2024/06/11/linkedlistexample1.png" style="height: 62px; width: 450px;" />

Remove the nodes with value 1.
```

**Example 3:**

```
**Input:** <span class="example-io">nums = [5], head = [1,2,3,4]</span>

**Output:** <span class="example-io">[1,2,3,4]</span>

**Explanation:**

**<img alt="" src="https://assets.leetcode.com/uploads/2024/06/11/linkedlistexample2.png" style="width: 400px; height: 83px;" />**

No node has value 5.
```


**Constraints:**


+ `1 <= nums.length <= 10^5`
+ `1 <= nums[i] <= 10^5`
+ All elements in `nums` are unique.
+ The number of nodes in the given list is in the range `[1, 10^5]`.
+ `1 <= Node.val <= 10^5`
+ The input is generated such that there is at least one node in the linked list that has a value not present in `nums`.


