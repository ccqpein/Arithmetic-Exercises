soe :: Int -> [Int]
soe n = let nL = [1..n]
            endNum = ceiling $ sqrt $ fromIntegral n in
          nL[endNum]
