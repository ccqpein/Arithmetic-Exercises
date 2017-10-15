import           Data.Char

detectCapitalUse :: String -> Bool
detectCapitalUse "" = False
detectCapitalUse s  = matchCapital s 0

matchCapital :: String -> Int -> Bool
matchCapital [] _ = True
matchCapital (x:xs) i
  | i == 0 = if isUpper x then matchCapital xs 1
             else matchCapital xs 2
  | i == 1 = if isUpper x then matchCapital xs 3
             else matchCapital xs 2
  | i == 2 = if isUpper x then False
             else matchCapital xs 2
  | i == 3 = if isUpper x then matchCapital xs 3
             else False

main = do
  print $ detectCapitalUse "USA"
  print $ detectCapitalUse "FlaG"
  print $ detectCapitalUse "ggg"
