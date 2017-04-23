main = do
  let testDate = [14,33,27,10,35,19,42,44]


selectionSort :: [Integer] -> [Integer]
selectionSort [] = []
selectionSort a = foldl 
