import           Data.List (delete)
-- import           Debug.Trace

main = do
  let testDate = [14,33,27,10,35,19,42,44]
  print (selectionSort testDate)


selectionSort :: [Integer] -> [Integer]
selectionSort [] = []
selectionSort a = let childResult = minimum' a in
                       (fst childResult):(selectionSort $ snd childResult)

minimum' :: [Integer] -> (Integer, [Integer])
minimum' a = let minVal = minimum a in
               (minVal, (delete minVal a))
