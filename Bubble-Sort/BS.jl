function bubbleSort(ll)
    flag = true
    len = length(ll)
    while flag
        flag = false
        this = ll[1]
        result = [];
        for i in 2:len
            if ll[i] < this
                push!(result, ll[i])
                flag = true
            else
                push!(result, this)
                this = ll[i];
            end
        end
        push!(result,this)
        ll = result
    end
    ll
end

using Test
@test bubbleSort([14,33,27,35,10]) == [10,14,27,33,35]
