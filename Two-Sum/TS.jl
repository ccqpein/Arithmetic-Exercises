function twoSum(l::Vector{T}, target::T) where T
    len = length(l)
    for i in 1:(len - 1)
        for j in (i+1):len
            l[i] + l[j] == target && return [i,j]
        end
    end
    return []
end
