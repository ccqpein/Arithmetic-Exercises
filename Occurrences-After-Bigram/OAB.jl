function findOccurrences(text, first, second)
    str_list = split(text, " ");
    ind = 1
    result = []
    
    while true
        third = try
            str_list[ind+2]
        catch e
            e isa BoundsError && return result
        end
        _first = str_list[ind]
        _second = str_list[ind+1]

        if _first == first && _second == second
            push!(result, third)
        end
        ind += 1
    end
end

# test below
using Test
@test findOccurrences("we will we will rock you", "we", "will") == ["we", "rock"]
@test findOccurrences("alice is a good girl she is a good student", "a", "good") == ["girl","student"]
