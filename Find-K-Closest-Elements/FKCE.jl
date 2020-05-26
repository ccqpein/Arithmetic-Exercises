function find_closest_element(arr::Array{Int}, k::UInt, x::Int)
    larger = findfirst(v -> v>=x, arr)
    smaller = larger

    #
    len = lastindex(arr)
    larger isa Nothing && begin
        return arr[(len - k + 1):len]
        end 
    smaller == 1 && return arr[1:k]

    #
    smaller -= 1
    count = 0

    while true
        smaller < 1 && return arr[1:k]
        larger > len && return arr[(len - k + 1):len]

        count == k && break

        if x - arr[smaller] <= arr[larger] - x
            smaller -= 1
        else
            larger += 1
        end
        count += 1
    end

    arr[smaller+1:larger-1]
end
