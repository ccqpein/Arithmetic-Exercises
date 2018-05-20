{-# LANGUAGE GADTs #-}
data MathExp a where
  Add :: (Num a) => MathExp a -> MathExp a -> MathExp a
  Minu :: (Num a) => MathExp a -> MathExp a -> MathExp a
  Mul :: (Num a) => MathExp a -> MathExp a -> MathExp a
--  Div :: (Num a, Fractional a) => MathExp a -> MathExp a -> MathExp a
  Val :: (Num a) => a -> MathExp a

{--
data MathExp a =
  Add (MathExp a) (MathExp a)|
  Minu (MathExp a) (MathExp a)|
  Mul (MathExp a) (MathExp a)|
  Div (MathExp a) (MathExp a)|
  Val a
--}

eval :: MathExp a -> a
eval (Add x y)  = eval x + eval y
eval (Minu x y) = eval x - eval y
eval (Mul x y)  = eval x * eval y
--eval (Div x y)  = eval x `div` eval y
eval (Val x)    = x

main :: IO()
main = do
  let a = Add (Val 1) (Mul (Val 2) (Val 3))
  print (eval a::Integer)

