function isHappy(s)::Bool
    a::Int = s isa Number ? s : (try
                                 parse(Int64, s)
                                 catch e
                                 throw(error("Wrong type $e"))
                                 end)
    innerHappy(a, Set([]))
end

function eachBit(n::Int)
    result = []
    while n != 0
        let left = n % 10
            push!(result, left)
            n = Int((n-left)/10)
        end
    end
    result
end

function innerHappy(n,se)
    n in se && return false
    next = sum(eachBit(n) .^ 2)
    next == 1 ? (return true) : (se = union(se, n))
    innerHappy(next,se)
end
