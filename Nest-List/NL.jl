# using Pkg
# Pkg.add("Match")
# using Match

function nl(al)
    templ = [];
    for a in al
        if typeof(a) <: Array
            append!(templ, nl(a))
        else
            push!(templ, a)
        end
    end
    templ
end

function nl2(al)
    function innerfunc(nl, last) :: Array
        if length(nl) == 0
            return last
        end
        
        if typeof(nl[1]) <: Array
            innerfunc(nl[2:end], innerfunc(nl[1],last))
        else
            innerfunc(nl[2:end], push!(last, nl[1]))
        end
    end

    innerfunc(al,[])
end

function nlFromLisp(nl)
    if length(nl) == 0
        []
    elseif typeof(nl[1]) <: Array
        append!(nlFromLisp(nl[1]), nlFromLisp(nl[2:end]))
    else
        append!([nl[1]], nlFromLisp(nl[2:end]))
    end
end

using Test

@test nl([1, [2, 3, [4, 5], [6, 7, [8]], [9]], 10]) == [1,2,3,4,5,6,7,8,9,10]
@test nl2([1, [2, 3, [4, 5], [6, 7, [8]], [9]], 10]) == [1,2,3,4,5,6,7,8,9,10]
@test nlFromLisp([1, [2, 3, [4, 5], [6, 7, [8]], [9]], 10]) == [1,2,3,4,5,6,7,8,9,10]
