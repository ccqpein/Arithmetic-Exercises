filterCond :: Int -> Int -> Bool
filterCond n n2 = if ((==0).(`mod`n)) n2
                  then (/=n) n2
                  else False

filterL :: Int -> [Int] -> [Int]
filterL n nL = filter ((not).(filterCond n)) nL

endNum :: Int -> [Int]
endNum n = [2..((ceiling.sqrt.fromIntegral) n)]

soe :: [Int] -> [Int] -> [Int]
soe (x:xs) nL = let nLL = filterL x nL in
                  soe xs nLL
soe [] nL = tail nL

