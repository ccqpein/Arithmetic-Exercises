function isHappy(s)::Bool
    a = s isa Number ? (try
                        parse(Int64, s)
                        catch e
                        println("Wrong type $e")
                        end) : s
    println(a)
    true
end
