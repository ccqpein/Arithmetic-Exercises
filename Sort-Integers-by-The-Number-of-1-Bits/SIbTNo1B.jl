function sort_by_bits(nums)
    #result = repeat([Array{Integer,1}()],16)
    result = [Array{Integer,1}() for _=1:16]
    for num in nums
        let temp = result[count_ones(num) + 1]
            result[count_ones(num) + 1] = push!(temp, num)
        end
    end
    collect(Iterators.flatten(map(x -> sort(x), filter(!isempty, result))))
end

using Test
@test sort_by_bits([0,1,2,3,4,5,6,7,8]) == [0,1,2,4,8,3,5,6,7]
@test sort_by_bits([2,3,5,7,11,13,17,19]) == [2,3,5,17,7,11,13,19]
@test sort_by_bits([10,100,1000,10000]) == [10,100,10000,1000]
@test sort_by_bits([1024,512,256,128,64,32,16,8,4,2,1]) == [1,2,4,8,16,32,64,128,256,512,1024]
