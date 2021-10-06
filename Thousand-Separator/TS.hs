int2Chars :: Int -> [Chars]
int2Chars = show

thousandSeparator :: Int -> String
thousandSeparator n = reverse $ addDotIn $ everythree $ reverse (show n)
  where everythree :: [Char] -> [[Char]]
        everythree = chunks 3

        addDotIn :: [[Char]] -> [Char]
        addDotIn = init.(foldl (\acc x -> acc ++ x ++ ['.']) [])


chunks :: Int -> [a] -> [[a]]
chunks _ [] = []
chunks n xs =
    let (ys, zs) = splitAt n xs
    in  ys : chunks n zs
