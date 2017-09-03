{-- --:= Mark: find better way to construct complex number
type ImaginaryUnit = Integer
type RealNumber = Integer
data ComplexNumber = ComplexN {real :: RealNumber,
                               imag :: ImaginaryUnit}
--}

import           Data.Complex

readComplex :: String -> Complex Double
readComplex = read

result :: (String, String) -> Complex Double
result (x,y) = (readComplex x) * (readComplex y)

main = do
  let exm1 = ("1 :+ 1","1 :+ 1")
  let exm2 = ("1 :+ -1","1 :+ -1")
  print $ result exm1
  print $ result exm2
