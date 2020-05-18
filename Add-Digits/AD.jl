include("../Happy-Number/HN.jl")

function addDigit(n::Int) ::Int
    n<0&& return -1
    n in range(0,stop=9) && return n
    n >= 10 && addDigit(sum(eachBit(n)))
end
