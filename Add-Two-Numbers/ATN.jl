mutable struct ListNode{T<:Number}
    val :: T
    next:: Union{ListNode, Nothing}    

    ListNode(data::Array{T,1}) where T<:Number = begin
        (head, rest) = Iterators.peel(data)
        if isempty(rest)
            new{T}(head::T, nothing)
        else
            new{T}(head, ListNode{T}(collect(rest) :: Array{T,1}))
        end
    end
end
