function distributeCandies(nums::Array{Int64})
    a = Set(nums)
    if length(a) > length(nums)/2
        length(nums) / 2
    else
        length(a)
    end
end

using Test
@test distributeCandies([1,1,2,2,3,3]) == 3
@test distributeCandies([1,1,2,3]) == 2
