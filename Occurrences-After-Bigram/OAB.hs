import           Data.List.Split

findOcurrences :: String -> String -> String -> [String]
findOcurrences text first second =
  let text_l = splitOn " " text in
    loopBlock text_l (first, second) []
  where
    loopBlock :: [String] -> (String, String) -> [String] -> [String]
    loopBlock (x:y:[]) (f,s) result = reverse result
    loopBlock (x:y:z:rest) (f,s) result
      | x == f && y == s = loopBlock (y:z:rest) (f,s) (z:result)
      | otherwise = loopBlock (y:z:rest) (f,s) result
