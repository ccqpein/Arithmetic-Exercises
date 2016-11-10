data NestedInteger = Empty | NestedInteger Int NestedInteger deriving (Show)

ttest :: [Int] -> NestedInteger
ttest [] = Nothing
ttest (x:xs) = 
