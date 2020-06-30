maxSizeSlices(slice::Array{Int64,1}) = maxInner(slice, 1, length(slice), div(length(slice), 3), 1)

function maxInner(slice::Array{Int64}, i, j, left, cycle=0)
    left == 1 && return maximum(slice[i:j])
    left == 0 && return 0
    (j - i + 1) < (2 * left - 1) && return -2147483648

    max(
        (slice[j] + maxInner(slice, i + cycle, j - 2, left - 1)),
        maxInner(slice, i, j - 1, left)
    )
end

using Test
@test maxSizeSlices([3, 1, 2]) == 3
@test maxSizeSlices([4, 1, 2, 5, 8, 3, 1, 9, 7]) == 21

@test maxSizeSlices([8, 9, 8, 6, 1, 1]) == 16
@test maxSizeSlices([1, 2, 3, 4, 5, 6]) == 10

@test maxSizeSlices([10, 9, 1, 10, 8, 5, 10, 2, 2]) == 30
@test maxSizeSlices([9, 8, 1, 7, 7, 9, 5, 10, 7, 9, 3, 8, 3, 4, 8]) == 45

@test maxSizeSlices([9, 5, 1, 7, 8, 4, 4, 5, 5, 8, 7, 7]) == 30

@test maxSizeSlices([
    2, 4, 3, 1, 10, 6, 1, 2, 10, 2, 8, 9, 4, 8, 8, 8, 10, 3, 9, 10, 7, 9, 4, 5, 4, 3, 1,
    10, 6, 1
]) == 85

@test maxSizeSlices([
    6, 3, 1, 2, 6, 2, 4, 3, 10, 4, 1, 4, 6, 5, 5, 3, 4, 7, 6, 5, 8, 7, 3, 8, 8, 1, 7, 1, 7,
    8
]) == 70

# @test maxSizeSlices([
#     7, 8, 5, 6, 9, 10, 1, 6, 5, 10, 8, 10, 5, 4, 7, 2, 8, 5, 9, 7, 5, 9, 3, 7, 7, 2, 2, 10,
#     7, 6, 4, 6, 5, 7, 7, 9, 6, 8, 10, 7, 5, 7, 2, 5, 4, 9, 6, 10, 10, 2, 10
# ]) == 150
