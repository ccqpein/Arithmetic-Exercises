letter2int :: Char -> Int
letter2int a = case a of
  'I' -> 1
  'V' -> 5
  'X' -> 10
  'D' -> 500
  'L' -> 50
  'C' -> 100
  'M' -> 1000

roman2Int :: (Int,String) -> Int
roman2Int (i,"")      = i
roman2Int (0, (x:xs)) = case xs of
  [] -> roman2Int ((letter2int x), "")
  xs -> roman2Int ((letter2int x), (x:xs))
roman2Int (i, (x:"")) = i
roman2Int (i, (x:y:xs)) = case (y,x) of
  ('V', 'I') -> roman2Int (i+(letter2int y)-2,(y:xs))
  ('X', 'I') -> roman2Int (i+(letter2int y)-2,(y:xs))
  ('M', 'C') -> roman2Int (i+(letter2int y)-200,(y:xs))
  ('D', 'C') -> roman2Int (i+(letter2int y)-200,(y:xs))
  ('C', 'X') -> roman2Int (i+(letter2int y)-20,(y:xs))
  ('L', 'X') -> roman2Int (i+(letter2int y)-20,(y:xs))
  (_,_)      -> roman2Int (i+(letter2int y), (y:xs))
