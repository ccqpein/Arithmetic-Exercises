def isAnagram(s:String, t:String) = if (s.reverse == t) true else false

println(isAnagram("abc", "cba"))
println(isAnagram("abc", "cba "))
