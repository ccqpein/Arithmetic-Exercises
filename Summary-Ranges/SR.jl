function summaryRanges(nums::Array{Number})
    length(nums) == 0 && return []

    start, endding = nums[1], nums[1]
    result::Array{String} = []

    for i in nums
        i > endding+1 &&
            begin
                start != endding ?
                    push!(result, string(start, "->", endding)) :
                    push!(result, string(start))
                start = i
            end
        endding = i
    end

    start != endding ?
        push!(result, string(start, "->", endding)) :
        push!(result, string(start))

    result
end

using Test
@test summaryRanges([0, 1, 2, 4, 5, 7]) == ["0->2", "4->5", "7"]
@test summaryRanges([0, 2, 3, 4, 6, 8, 9]) == ["0", "2->4", "6", "8->9"]
