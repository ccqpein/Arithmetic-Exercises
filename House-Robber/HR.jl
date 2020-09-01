function rob(nums::Array{T,1}) where T
    table = Dict{T,UInt64}()
    function inner(nums, start, len, table)
        try
            return table[start]
        catch end

        if start == len
            table[start] = nums[start]
            return nums[start]
        elseif start == len + 1
            table[start] = 0
            return 0
        end

        a = max(
            maximum([inner(nums, x, len, table) for x in start+1:len]),
            nums[start] + inner(nums, start+2, len, table)
        )

        table[start] = a
        a
    end
    inner(nums, 1, length(nums), table)
end

using Test
@test rob([183, 219, 57, 193, 94, 233, 202, 154, 65, 240, 97, 234, 100, 249, 186, 66, 90, 238, 168, 128, 177, 235, 50, 81, 185, 165, 217, 207, 88, 80, 112, 78, 135, 62, 228, 247, 211]) == UInt64(3365)
