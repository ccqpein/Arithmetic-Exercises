reserveStr :: String -> String
reserveStr []     = []
reserveStr (c:cs) =  reserveStr cs ++ [c]

isAnagram :: String -> String -> Bool
isAnagram = ((==).reserveStr)
