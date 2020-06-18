function maximumGap(nums)
    length(nums) < 2 && return 0

    sort!(nums)
    largest = 0;
    last = nums[1]
    for c in nums
        (c - last) > largest && begin largest = (c - last) end
        last = c
    end

    largest
end

using Test
@test maximumGap([3,6,9,1]) == 3
@test maximumGap([10]) == 0
