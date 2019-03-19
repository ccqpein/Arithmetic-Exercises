makeTriangle :: Int -> Int -> IO ()
makeTriangle wholenum lineInd
  | lineInd < 0 ||
    wholenum < lineInd = return ()
  | otherwise = do
      makeALine wholenum lineInd
      makeTriangle wholenum (lineInd+1)

makeALine :: Int -> Int -> IO ()
makeALine w i =  let space = 2 * (w - 1) + 1
                     taken = 2 * (i - 1) + 1
                     head' = (space - taken) `div` 2
                     tail' = space - head' - taken
                 in
                   do
                     print $ [' ' | _ <- [1..head']]
                       ++ "*"
                       ++ (foldl (++) "" [" *" | _ <- [1..taken `div` 2]])
                       ++ [' '| _ <- [1..tail']]

makeTriangle2 :: Int -> IO ()
makeTriangle2 a = mconcat $ [makeALine a b| b <- [1..a]]

main = do
  makeTriangle 10 1
  makeTriangle 5 1
