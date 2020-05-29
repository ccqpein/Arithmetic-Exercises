function addStrings(s1::String, s2::String) :: Int64
    chars = ['0','1','2','3','4','5','6','7','8','9']
    dd = Dict([(chars[n+1],n) for n in 0:9])

    result = 0
    len1 = length(s1)
    for ind in 1:len1
        result += dd[s1[ind]] * (10 ^ len1)/(10 ^ ind)
    end

    len2 = length(s2)
    for ind in 1:len2
        result += dd[s2[ind]] * (10 ^ len2)/(10 ^ ind)
    end
    
    result
end
