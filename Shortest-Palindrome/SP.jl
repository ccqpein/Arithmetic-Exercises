function isPalindrome(str) :: Bool
    len = length(str)
    head = if isodd(len)
        str[1:Int(ceil(len/2) - 1)]
    else
        str[1:Int(ceil(len/2))]
    end

    str[Int(ceil(len/2))+1:len] == reverse(head)
end

function shortestPalindrome(str) :: String
    str == "" && return ""
    len = length(str)
    reverseStr = reverse(str)
    for ind in 1:len
        isPalindrome(reverseStr[ind:len]) &&
            return join([reverseStr[1:ind-1], str])
    end
end

using Test
@test shortestPalindrome("aacecaaa") == "aaacecaaa"
@test shortestPalindrome("abcd") == "dcbabcd"
@test shortestPalindrome("") == ""
@test shortestPalindrome("aba") == "aba"
@test shortestPalindrome("abbacd") == "dcabbacd"
@test shortestPalindrome("abb") == "bbabb"
@test shortestPalindrome("aaaaa") == "aaaaa"
@test shortestPalindrome("aabba") == "abbaabba"
@test shortestPalindrome("a") == "a"
@test shortestPalindrome("babbbabbaba") == "ababbabbbabbaba"
@test shortestPalindrome("aaaabbaa") == "aabbaaaabbaa"
