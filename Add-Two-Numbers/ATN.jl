mutable struct ListNode{T<:Number}
    val :: T
    next:: Union{ListNode, Nothing}    

    ListNode(data::Array{T,1}) where T<:Number = begin
        (head, rest) = Iterators.peel(data)
        if isempty(rest)
            new{T}(head::T, nothing)
        else
            new{T}(head, ListNode(collect(rest) :: Array{T,1}))
        end
    end
end

function node2list(x::Union{ListNode{T}, Nothing}) :: Array{T} where {T<:Number}
    result = []
    while x.next != nothing
        push!(result, x.val)
        x = x.next
    end
    push!(result, x.val)
    result
end

function addTwoNumbers(a::ListNode{T}, b::ListNode{T}) :: ListNode{T} where T<:Number
    this_a::Union{ListNode, Nothing}, this_b::Union{ListNode, Nothing} = a, b
    flag = false
    result::Array{T} = []
    while true
        (this_a == nothing && this_b == nothing) && begin
            flag && push!(result,1)
            return ListNode(result)
        end
        
        this_a == nothing && begin
            append!(result, begin
                    cache = node2list(this_b)
                    if flag cache[1] += 1 end
                    cache
                    end)
            return ListNode(result)
        end

        this_b == nothing && begin
            append!(result, begin
                    cache = node2list(this_a)
                    if flag cache[1] += 1 end
                    cache
                    end)
            return ListNode(result)
        end

        push!(result, this_a.val + this_b.val + (flag ? 1 : 0))
        if last(result) >= 10
            result[lastindex(result)] -= 10
            flag = true
        else
            flag = false
        end
        
        this_a, this_b = this_a.next, this_b.next
    end
    ListNode(result)
end
