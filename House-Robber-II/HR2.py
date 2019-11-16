def rob(nums):
    if len(nums) == 0:
        return 0
    if len(nums) <= 3:
        return max(nums)

    return max([max_inner(nums[1:]), max_inner(nums[:-1])])


def max_inner(nums):
    temp, before_max, now_max = 0, 0, 0
    for (ind, tempv) in enumerate(nums):
        temp = now_max
        now_max = max([before_max+tempv, now_max])
        before_max = temp

    return now_max
