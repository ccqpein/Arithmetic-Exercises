mutable struct ListNode{T<:Number}
    val :: T
    next:: ListNode
    ListNode{T}(data::Array{T}) where T<:Number = begin
        (head, rest) = Iterators.peel(data)
        new(head, new(collect(rest)))
    end
end

