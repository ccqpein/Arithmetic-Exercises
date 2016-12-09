filterCond :: Int -> Int -> Bool
filterCond n n2 = if ((==0).(`mod`n)) n2
                  then (/=n) n2
                  else False

filterL :: Int -> [Int] -> [Int]
filterL n nL = filter ((not).(filterCond n)) nL

endNum :: Int -> [Int]
endNum n = [2..((ceiling.sqrt.fromIntegral) n)]

innerSoe :: [Int] -> [Int] -> [Int]
innerSoe (x:xs) nL = let nLL = filterL x nL in
                  innerSoe xs nLL
innerSoe [] nL = tail nL

--innerSoe (endNum 50) [1..50]

soe :: Int -> [Int]
soe n = (innerSoe $ endNum n) [1..n]
