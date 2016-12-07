filterCond :: Int -> Int -> Bool
filterCond n n2 = if ((==0).(`mod`n)) n2
                  then (/=n) n2
                  else False

filterL :: Int -> [Int] -> [Int]
filterL n nL = filter ((not).(filterCond n)) nL


--endNum = (ceiling.sqrt.fromIntegral) n

