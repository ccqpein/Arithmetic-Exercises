func isAnagram (s1:String, s2:String) -> Bool {
    let s2temp = String(s2.characters.reverse())

    if s2temp == s1{ return true}
    else {return false}
}

print(isAnagram("aa",s2:"aa"))
