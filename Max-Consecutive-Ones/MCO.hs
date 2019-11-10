import           Data.List.Split

findMaxConsecutiveOnes :: [Int] -> Int
findMaxConsecutiveOnes = maximum.(map length).(splitOn [0])
