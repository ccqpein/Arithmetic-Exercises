function isValid(input::String)
    checkNine(n) = n > 9 ? n - 9 : n

    input = filter(x -> x != ' ' ,collect(input))

    length(input) <= 1 && return false

    parsedArr = try
        map(x -> parse(Int,x), input)
    catch
        return false
    end

    flag = true
    result = 0
    for i in reverse(parsedArr)
        if flag
            result += i
        else
            result += checkNine(2 * i)
        end
        flag = !flag
    end

    0 == (result % 10)
end
