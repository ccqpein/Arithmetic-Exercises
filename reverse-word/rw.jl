function reverseWord(s::String)
    result::Array{Char} = []
    cache::Array{Char} = []
    for c in s
        c == ' ' && begin
            append!(result,cache);
            cache = []
            push!(result,c);
            continue
        end

        cache = append!([c],cache)
    end

    append!(result,cache);
    String(result)
end
