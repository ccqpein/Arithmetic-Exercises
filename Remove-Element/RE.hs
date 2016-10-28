removeEle :: (Eq a) => a -> [a] -> [a]
removeEle a = filter (/= a)
