open List;;

let rec count_smaller (l:'a list) =
  match l with
  |[] -> []
  |x::xs -> length (filter (fun ele -> ele < x) xs) :: count_smaller xs
