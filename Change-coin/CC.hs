import qualified Data.Map.Lazy as Map
import           Debug.Trace

type NumOfCoin = Integer
type ResultList = Map.Map Integer NumOfCoin


coinChange :: Integer -> [Integer] -> ResultList -> (NumOfCoin, ResultList)
coinChange v coinL resultL
  | v == 0 = (0, resultL)
  | elem v coinL = (1, (Map.insert v 1 resultL))
  | Map.member v resultL = (resultL Map.! v, resultL)
  | otherwise = let vv = (minimum $ innerFunc v coinL resultL)
                in (vv, (Map.insert v vv resultL))

innerFunc :: Integer -> [Integer] -> ResultList -> [Integer]
innerFunc v coinL resultL
  = foldl (\a b -> a ++ [(1 + (fst $ coinChange (v-b) coinL resultL))])
    [] [c | c <- coinL, c <= v]

main = do
  let coinList = [1,5,10,25]
  let resultList = Map.fromList [(1,1),(5,1),(10,1),(25,1)]
  print (runCoinChange 50 0 coinList resultList)
    where runCoinChange n i coinL resultL
            | n == i = resultL
            | otherwise = runCoinChange n (i + 1) coinL (snd $ coinChange (i + 1) coinL resultL)
